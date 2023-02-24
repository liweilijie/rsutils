use notify::{Event, RecommendedWatcher, RecursiveMode, Watcher, Config};
use std::path::Path;
use tokio::sync::mpsc::channel;
use tokio::sync::mpsc::Receiver;
use lazy_static::lazy_static;
use tokio::time::{sleep, Duration};
use std::sync::Arc;
use std::sync::atomic::AtomicBool;
use tokio::runtime::Handle;
use tokio::task::spawn_blocking;

lazy_static! {
    static ref RUNNING: Arc<AtomicBool> = Arc::new(AtomicBool::new(true));
}


/// Async, futures channel based event watching
#[tokio::main]
async fn main() {
    let path = std::env::args()
        .nth(1)
        .expect("Argument 1 needs to be a path");
    println!("watching {}", path);

    let handle = Handle::current();
    // TODO:  参考这里新建一个线程来做 https://docs.rs/tokio/latest/tokio/runtime/struct.Handle.html
    std::thread::spawn(move || {
        handle.block_on(async {
            if let Err(e) = async_watch(path).await {
                println!("error: {:?}", e)
            }
        });
    });


    while RUNNING.load(std::sync::atomic::Ordering::Relaxed) {
        println!("sleeping.");
        sleep(Duration::from_secs(10)).await;
    }

    println!("done.");
}

fn async_watcher() -> notify::Result<(RecommendedWatcher, Receiver<notify::Result<Event>>)> {
    let (tx, rx) = channel(1);

    // 这个 handle 一定要在 closure 外面才可以，不然会 panic
    let handle = Handle::current();

    // Automatically select the best implementation for your platform.
    // You can also access each implementation directly e.g. INotifyWatcher.
    let watcher = RecommendedWatcher::new(move |res| {
        handle.block_on(async {
            tx.send(res).await.unwrap();
        });
    }, Config::default())?;

    Ok((watcher, rx))
}

async fn async_watch<P: AsRef<Path>>(path: P) -> notify::Result<()> {
    println!("async_watch {}", path.as_ref().display());
    let (mut watcher, mut rx) = async_watcher()?;
    println!("return async_watcher..");

    // Add a path to be watched. All files and directories at that path and
    // below will be monitored for changes.
    watcher.watch(path.as_ref(), RecursiveMode::Recursive)?;

    println!("waiting change.");
    while let Some(res) = rx.recv().await {
        match res {
            Ok(event) => println!("changed: {:?}", event.kind),
            Err(e) => println!("watch error: {:?}", e),
        }
    }

    Ok(())
}

async fn shutdown_signal() {
    tokio::signal::ctrl_c()
        .await
        .expect("failed to install CTRL+C signal handler");
    RUNNING.store(false, std::sync::atomic::Ordering::Relaxed);
}
