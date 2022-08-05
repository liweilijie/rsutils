# keep in mind

 牢记的知识点。

 ```rust
 let a = Box::new(3);
 println!("a={}", a); // a = 3
 // 下面一行代码会报错
 // let b = a + 1;
 ```

 在表达式中，我们无法自动隐式地执行 **Deref** 解引用操作，你需要使用`* 操作符 let b = *a + 1`, 来显示的进行解引用。
