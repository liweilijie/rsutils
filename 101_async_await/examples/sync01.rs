use tokio::runtime::Builder;
use tokio::time::sleep;
use std::time;
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use std::sync::mpsc::{Receiver, channel};
use std::time::Duration;

async fn nice() {
    sleep(time::Duration::from_secs(1)).await;
}

// https://www.bilibili.com/video/BV1CN411f7nF?from=search&seid=1603161245627727353&spm_id_from=333.337.0.0
fn main() {
    // 在runtime启动当前的一个线程里面
    // let rt = Builder::new_current_thread().enable_all().build().unwrap();
    // 在runtime上面启动多个线程里面
    let rt = Builder::new_multi_thread().enable_all().build().unwrap();

    let job1 = async {
        // 重启启动了一个任务在这里会有安全问题，因为block_on(job1)并不知道内部任务，也并不会等待它的结束
        tokio::spawn(async{
            println!("在子任务中又启动一个新的任务");
            nice().await;
        });
        nice().await;
        println!("block over.");
    };
    rt.block_on(job1);

    let job2 = Hello::Started{};
    let result = rt.block_on(job2);
    println!("job2 return:{}", result);
}

// 自定义Future的方式
enum Hello {
    Started,
    Working(Receiver<i32>),
    Done,
}

impl Future for Hello {
    type Output = i32;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let mut h = self.get_mut();
        let waker = cx.waker().clone(); // 唤醒的时候用

        // 刚开始是Started，然后置为Working, 睡1秒之后唤醒，并且发送数据
        match *h {
            Hello::Started => {
                let (tx, rx) =channel();
                *h = Hello::Working(rx);
                std::thread::spawn(move || {
                    std::thread::sleep(Duration::from_secs(1));
                    waker.wake(); // 唤醒让调度器去调度，不然调度器不会去调度的，会一直卡在哪儿等待他的唤醒
                    tx.send(100);
                });
                Poll::Pending
            },
            Hello::Working(ref rx) => {
                let v = rx.recv().unwrap();
                Poll::Ready(v)
            },
            Hello::Done => {
                panic!("Not here!")
            }
        }
    }
}