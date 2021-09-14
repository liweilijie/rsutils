# for循环使用

for 循环的常用方式

#### 常用的区间

```rust
for n in 1..=100 {} // 表示1到100包含100
```

#### for与迭代器Iterator
for in 结构能以几种方式与 Iterator 互动。在 迭代器 trait 一节将会谈 到，如果没有特别指定，for 循环会对给出的集合应用 into_iter 函数，把它转换成 一个迭代器。这并不是把集合变成迭代器的唯一方法，其他的方法有 iter 和 iter_mut 函数。
这三个函数会以不同的方式返回集合中的数据。

- iter - 在每次迭代中借用集合中的一个元素。这样集合本身不会被改变，循环之后仍 可以使用。
    ```rust
    fn main() {
        let names = vec!["Bob", "Frank", "Ferris"];
        for name in names.iter() {
            match name {
                &"Ferris" => println!("There is a rustacean among us!"),
                _ => println!("Hello {}", name),
            }
        }
    }
    ```

- into_iter - 会消耗集合。在每次迭代中，集合中的数据本身会被提供。一旦集合被消 耗了，之后就无法再使用了，因为它已经在循环中被 “移除”（move）了。
    ```rust
    fn main() {
        let names = vec!["Bob", "Frank", "Ferris"];

        for name in names.into_iter() {
            match name {
                "Ferris" => println!("There is a rustacean among us!"),
                _ => println!("Hello {}", name),
            }
        }
    }
    ```
- iter_mut - 可变地（mutably）借用集合中的每个元素，从而允许集合被就地修改。
    ```rust
    fn main() {
        let mut names = vec!["Bob", "Frank", "Ferris"];
        for name in names.iter_mut() {
            *name = match name {
                &mut "Ferris" => "There is a rustacean among us!",
                _ => "Hello",
            }
        }

        println!("names: {:?}", names);
    }
    ```
在上面这些代码中，注意 match 的分支中所写的类型不同，这是不同迭代方式的关键 区别。因为类型不同，能够执行的操作当然也不同。 

#### 处理嵌套循环和标签
在处理嵌套循环的时候可以 break 或 continue 外层循环。在这类情形中，循环必须 用一些 'label（标签）来注明，并且标签必须传递给 break/continue 语句。
[例子中学习rust](https://rustwiki.org/zh-CN/rust-by-example/flow_control/loop/nested.html)

```rust
#![allow(unreachable_code)]
fn main() {
    'outer: loop {
        println!("Entered the outer loop");
        
        'inner: loop {
            println!("Entered the inner loop");
            
            // 这只是中断内部的循环
            // break;
            
            // 这会中断最外层的循环
            break 'outer;
        }
        
        println!("This point will never be reached");
    }
    println!("Exit the outer loop");
}
```