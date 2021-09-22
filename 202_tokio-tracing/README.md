# tracing
打印日志

## 常用的打印所有的日志记录

意思是所有比`TRACE`更高等级:`info`,`warn`,`debug`,`error`等都可以打印出来的意思。
```rust
fn main() {
	tracing_subscriber::fmt()
		 // all spans/events with a level higher than TRACE (e.g, info, warn, etc.)
		 // will be written to stdout.
		 .with_max_level(tracing::Level::TRACE)
		 // sets this to be the default, global collector for this application.
		 .init();
}
```
