# rust 和 go的简单并发对比速度
rust的实现，要慢go一个数量级。
```rust
// use tokio::sync::Mutex;
// use std::ops::{AddAssign, Div};
use std::ops::Div;
use std::sync::atomic::{AtomicI32, AtomicU64, Ordering};
use std::thread;
use std::time::Duration;
use std::{sync::Arc, time::SystemTime};

use tokio::{runtime::Builder, sync::mpsc};

#[derive(Debug)]
struct Event {
    _type: Arc<String>, // type是只读的，利用Arc来避免多次堆分配
    time: SystemTime,
}

fn main() {
    let (tx, mut rx) = mpsc::unbounded_channel::<Event>();

    let rt = Builder::new_multi_thread()
        .worker_threads(8)
        .build()
        .unwrap();

    let count: Arc<AtomicI32> = Arc::new(AtomicI32::new(0));
    // let total: Arc<Mutex<Duration>> = Arc::new(Mutex::new(Duration::new(0, 0))); // 虽然是时间，但是只是为了计数
    let total: Arc<AtomicU64> = Arc::new(AtomicU64::new(0)); // 所以改成了AtomicU64比较高效

    {
        let count = count.clone();
        let total = total.clone();

        rt.spawn(async move {
            while let Some(res) = rx.recv().await {
                let elapsed = res.time.elapsed().ok().unwrap();
                count.fetch_add(1, Ordering::SeqCst);
                total.fetch_add(elapsed.as_micros() as u64, Ordering::SeqCst);
            }
        });
    }

    let max_loop = 10_0000;

    let type_name = Arc::new(String::from("hello"));

    for _ in 0..max_loop {
        let event = Event {
            _type: type_name.clone(),
            time: SystemTime::now(),
        };
        let _ = tx.send(event);

        thread::sleep(Duration::from_micros(1));
    }

    thread::sleep(Duration::from_millis(4000));

    println!("{:?}", count.load(Ordering::SeqCst));

    rt.block_on(async move {
        let total2 = total.load(Ordering::SeqCst);
        print!(
            "total: {:?}, average: {:?}, {:?}/s",
            total2,
            (total2 as f64).div(count.load(Ordering::SeqCst) as f64),
            count.load(Ordering::SeqCst) as f64 / total2 as f64
        );
    });
}
```

## go实现的版本

```go
package main

import (
	"fmt"
	"reflect"
	"sync/atomic"
	"time"
)

var count int32
var total int32

type event struct {
	args    []interface{}
	event   chan struct{}
	handler interface{}
}

func onEvent(sig chan struct{}, t time.Time) {
	d := time.Since(t)
	atomic.AddInt32(&count, 1)
	atomic.AddInt32(&total, int32(d))

	sig <- struct{}{}
}

func main() {

	queue := make(chan event, 200)
	defer close(queue)

	go func(q chan event) {
		// 循环读取事件
		for ev := range q {
			v := reflect.ValueOf(ev.handler)
			if v.Kind() != reflect.Func {
				panic("not a function")
			}

			vArgs := make([]reflect.Value, len(ev.args))
			for i, arg := range ev.args {
				vArgs[i] = reflect.ValueOf(arg)
			}
			v.Call(vArgs)
		}
	}(queue)

	const COUNT = 100000

	var sig = make(chan struct{}, COUNT)

	for i := 0; i < COUNT; i++ {
		// 发事件
		func(args ...interface{}) {
			var ev = event{args: args, handler: onEvent}
			queue <- ev
		}(sig, time.Now())

		time.Sleep(time.Microsecond)
	}

	for i := 0; i < COUNT; i++ {
		<-sig
	}

	fmt.Println("total:", total, "average:", time.Duration(total)/time.Duration(count), count)
}
```

