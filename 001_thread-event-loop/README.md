# eventloop 

利用thread的方式创建一个eventloop实例

用到的库有：
- crossbeam-channel
- rayon
- parking_lot

## channel
crossbeam中的`channel`很好用，和golang里面的channel用起来差不多，很舒服。

创建有限容量
```rust
use crossbeam_channel::bounded;



// 创建一个容量是5的channel
let (s, r) = bounded(5);

for i in 0..5 {
    s.send(i).unwrap();
}

// 超过5条就会阻塞了
// s.send(5).unwrap();
```

创建无限容量

```rust
use crossbeam_channel::unbounded;

// 创建一个无限容量的channel
let (s, r) = unbounded();

// 不会阻塞
for i in 0..1000 {
    s.send(i).unwrap();
}
```

## select
提供了类似`go`语言功能的`select`宏， 支持使用`default`分支处理超时等逻辑

```rust
use std::thread;
use std::time::Duration;
use crossbeam_channel::unbounded;

let (s1, r1) = unbounded();
let (s2, r2) = unbounded();

thead::spawn(move || s1.send(10).unwarp());
thead::spawn(move || s2.send(20).unwarp());

select! {
    recv(r1) -> msg => assert_eq!(msg, Ok(10)),
    recv(r1) -> msg => assert_eq!(msg, Ok(20)),
    default(Duration::from_secs(1)) => println!("timed out"),
}
```