#![allow(dead_code)]
#![allow(unused)]

#[macro_use]
// extern crate 可以不需要了，2018版本之后直接用use就行
// extern crate crossbeam_channel;
// extern crate rayon;
// 使用crossbeam_channel里面的channel来通信
use crossbeam_channel::{select, unbounded};
// 使用rayon里面的线程池来跑任务
use rayon;
use std::collections::HashMap;
use std::sync::{Arc, Condvar, Mutex};

// use parking_lot::{Condvar, Mutex}
// use std::sync::Arc
use std::thread;

// 定义work的消息内容
#[derive(Debug)]
enum WorkMsg {
    Work(u8),
    Exit,
}

#[derive(Debug, Eq, PartialEq)]
enum CacheState {
    Ready,
    WorkInProgress,
}

// 返回的消息内容
enum ResultMsg {
    // 它不是Result<>内容，而是里面包含了一个元组的类型
    Result(u8, WorkPerformed),
    Exited,
}

// 工作状态
struct WorkerState {
    ongoing: i16,
    exiting: bool,
}

impl WorkerState {
    fn init() -> Self {
        WorkerState {
            ongoing: 0,
            exiting: false,
        }
    }

    // 利用可变引用设置里面的属性值
    fn set_ongoing(&mut self, count: i16) {
        self.ongoing += count;
    }

    fn set_exiting(&mut self, exit_state: bool) {
        self.exiting = exit_state;
    }

    fn is_exiting(&self) -> bool {
        self.exiting == true
    }

    // 不再有worker任务在运行了
    fn is_nomore_work(&self) -> bool {
        self.ongoing == 0
    }
}

// worker的结果从哪儿来的，是缓存队列还是新建的
#[derive(Debug, Eq, PartialEq)]
enum WorkPerformed {
    FromCache,
    New,
}

// CacheKey 在HashMap之中存储的key值
#[derive(Debug, Hash, Eq, PartialEq)]
struct CacheKey(u8);

const POOL_THREAD_COUNT: usize = 5;

fn main() {
    // Create a channel of unbounded capacity. 无限制的channel
    let (work_sender, work_receiver) = unbounded();
    let (result_sender, result_receiver) = unbounded();
    // 添加一个新的Channel, Worker使用它来通知『并行』组件已经完成了一个工作单元
    let (pool_result_sender, pool_result_receiver) = unbounded();
    let mut worker_state = WorkerState::init();

    // 使用线程池
    let pool = rayon::ThreadPoolBuilder::new()
        .num_threads(POOL_THREAD_COUNT)
        .build()
        .unwrap();

    // 缓存 work, 由池中的 worker 共享
    let cache: Arc<Mutex<HashMap<CacheKey, u8>>> = Arc::new(Mutex::new(HashMap::new()));

    // 增加缓存状态，指示对于给定的key, 缓存是否已经准备好被读取
    let cache_state: Arc<Mutex<HashMap<CacheKey, Arc<(Mutex<CacheState>, Condvar)>>>> =
        Arc::new(Mutex::new(HashMap::new()));

    let _ = thread::spawn(move || loop {
        // https://docs.rs/crossbeam-channel/0.5.1/crossbeam_channel/macro.select.html
        // 使用 crossbeam提供的select!宏选择一个就绪工作
        select! {
            recv(work_receiver) -> msg => {
                match msg {
                    Ok(WorkMsg::Work(num)) => {
                        let result_sender = result_sender.clone();
                        let pool_result_sender = pool_result_sender.clone();
                        let cache = cache.clone();
                        let cache_state = cache_state.clone();

                        // 注意，这里正在池上启动一个新的工作单元, TODO: 只有单线程才能访问吗？
                        worker_state.set_ongoing(1);

                        pool.spawn(move || {
                            let num = {
                                // https://skyao.io/learning-rust/std/sync/condvar.html
                                // let cache_state, cvar 是获取的元组里面值的引用类型, 请看上面的例子
                                let (cache_state_lock, cvar) = {
                                    // state_map 获取得到的是Mutex里面包裹着的HashMap的指针，需要解引用才能使用
                                    let mut state_map = cache_state.lock().unwrap();
                                    // 为了让元组解引用之后再返回引用给外面的对向，所以需要再加一个引用
                                    // 这里的优先级暂不考虑
                                    &*state_map
                                        // https://kaisery.github.io/trpl-zh-cn/ch08-03-hash-maps.html
                                        .entry(CacheKey(num.clone()))
                                        .or_insert_with(|| { Arc::new((Mutex::new(CacheState::Ready), Condvar::new()))})
                                        // 因为上面or_insert_with()会返回插入值的可变引用，所以其插入值是一个Arc包着的元组值，
                                        // clone则是对Arc这里面的值的clone，堆上面对于堆的引用计算个数加1
                                        // 一个值被插入，另外clone的值会被最外层的cache_state, cvar 获取
                                        .clone()
                                    // `cache_state` 锁的临界区结束

                                    // TODO: 对于链式调用，每一个函数都会被执行的
                                };

                                // `state` 临界区开始 它是与条件变量condvar绑定的最内层的一把锁
                                let mut state = cache_state_lock.lock().unwrap();

                                // 注意： 使用while循环来防止条件变量的虚假唤醒
                                // while let 常用在 while let Some(i) = val {} 里面
                                // 当 let 将 val 解构成 Some(i) 时，就执行{}里面的语句，否则就break
                                // 这里 利用 while let 来做解构 *state的值 如果不是Ready 则再将等待唤醒，并且再次检测
                                while let CacheState::WorkInProgress = *state {
                                    // 阻塞直到状态是 `CacheState::Ready`.
                                    //
                                    // 当唤醒时会自动释放锁的
                                    let current_state = cvar
                                        .wait(state) // 这里会用state的所有权
                                        .unwrap();
                                    state = current_state;
                                }

                                // 循环外可以认为state 已经是 Ready 的了
                                assert_eq!(*state, CacheState::Ready);

                                let (num, result) = {
                                    // cache 缓存临界区开始
                                    let cache = cache.lock().unwrap();
                                    let key = CacheKey(num);
                                    let result = match cache.get(&key) {
                                       Some(result) => Some(result.clone()),
                                       None => None,
                                    };
                                    (key.0, result)
                                    // cache 缓存临界区结束
                                };

                                if let Some(result) = result {
                                    // 从缓存中获得一个结果，并将其发送回去，
                                    // 同时带有一个标志，表明是从缓存中获得了它的
                                    let _ = result_sender.send(ResultMsg::Result(result, WorkPerformed::FromCache));
                                    let _ = pool_result_sender.send(());

                                    // 不要忘记通知等待线程
                                    cvar.notify_one();
                                    return; // TODO: 这里的return 和下面的num 为什么在if语句里面不报类型匹配不一致的问题呢？
                                } else {
                                    // 如果缓存里没有找到结果，那么切换状态
                                    *state = CacheState::WorkInProgress;
                                    num
                                }
                            };

                            let _ = result_sender.send(ResultMsg::Result(num.clone(), WorkPerformed::New));

                            // 插入工作结果到缓存中
                            {
                                // cache 缓存临界区开始
                                let mut cache = cache.lock().unwrap();
                                let key = CacheKey(num.clone());
                                cache.insert(key, num);
                                // cache 缓存临界区结束
                            }

                            // 重新进入state元素的临界区，将状态改为Ready
                            let (lock, cvar) = {
                                let mut state_map = cache_state.lock().unwrap();
                                &*state_map
                                    .get_mut(&CacheKey(num))
                                    .expect("Entry in cache state to have been previously inserted")
                                    .clone()
                            };
                            // 重新进入 `state` 临界区
                            let mut state = lock.lock().unwrap();

                            // 在这里，由于已经提前设置了state, 并且任何其他worker都将等待状态切换回ready, 可以确定该状态是"in-progress"。
                            assert_eq!(*state, CacheState::WorkInProgress);

                            // 切换状态为 Ready
                            *state = CacheState::Ready;

                            // 通知等待线程
                            cvar.notify_one();

                            let _ = pool_result_sender.send(());
                        });

                    },
                    Ok(WorkMsg::Exit) => {
                        // 注意，这里接收请求并退出
                        worker_state.set_exiting(true);

                        // 如果没有正在进行的工作则可以立即退出
                        if worker_state.is_nomore_work() {
                            println!("is is_nomore_work so exited.");
                            let _ = result_sender.send(ResultMsg::Exited);
                            break;
                        }
                    },
                    _ => panic!("Error receiving a WorkMsg."),
                }
            },
            recv(pool_result_receiver) -> _ => {
                // 会不会有竞争的问题
                if worker_state.is_nomore_work() {
                    panic!("Received an unexpected pool result.");
                }

                // 注意，一个工作单元已经被完成, 这里又会不会有锁的问题,
                // 其实不会，因为至始至终worker_state都只有一线程在用它
                worker_state.set_ongoing(-1);

                // 如果没有正在进行的工作了，并且接收到了退出请求，那么就退出整个worker任务
                if worker_state.is_nomore_work() && worker_state.is_exiting() {
                    println!("pool result to exited");
                    let _ = result_sender.send(ResultMsg::Exited);
                    break;
                }

            },
        }
    });

    // TODO: 这时work_sender已经是最后一个了，为什么还要clone一下呢？
    // 应该是因为worker_state和worker_receiver的生命周期一定要是一致的，不然会有一些意想不到的事情发生
    let work_sender = work_sender.clone();
    let schedule = thread::spawn(move || {
        for i in 0..=10 {
            let _ = work_sender.send(WorkMsg::Work(i.clone()));
            // 发送两个相同的work
            if i % 2 == 0 {
                let _ = work_sender.send(WorkMsg::Work(i.clone()));
            }

            if i == 10 {
                let _ = work_sender.send(WorkMsg::Exit);
            }
        }
    });

    let mut counter = 0;

    loop {
        match result_receiver.recv() {
            Ok(ResultMsg::Result(num, cached)) => {
                if cached == WorkPerformed::New {
                    counter += 1;
                }

                println!("{} from:{:?}", num, cached);
            }
            Ok(ResultMsg::Exited) => {
                println!("result received exiting and counter:{}", counter);
                break;
            }
            Err(e) => {
                println!("result received error {:?} and counter:{}", e, counter);
                break;
            }
        }
    }

    schedule.join().unwrap();
}
