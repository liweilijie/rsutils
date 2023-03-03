# async concurrent stream

[https://kerkour.com/rust-async-combinators](https://kerkour.com/rust-async-combinators) 学习于这篇文章.


you can simple clone a before the async move block:

```rust
stream
      .for_each_concurrent(None, |stream_item| {
          let a = a.clone();
          async move {
            println!("{}", a);
            println!("{}", stream_item);
          }
      })
      .await;
```