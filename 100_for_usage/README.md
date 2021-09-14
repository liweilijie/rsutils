# for,while,loop等循环使用和match模式匹配
for常用的一些形式
[for 循环的常用方式](https://rustwiki.org/zh-CN/rust-by-example/flow_control/for.html)

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

## match 匹配
match匹配很强大，用起来真的很爽，能让代码写得像python的代码一样美。

- 解构元组
    ```rust
    let pair(0, -2);
    match pair {
        // 解构出第二个值
        (0, y) => println!("First is `0` and `y` is `{:?}`", y),
        (x, 0) => println!("`x` is `{:?}` and last is `0`", x),
        _      => println!("It doesn't matter what they are"),
        // `_` 表示不将值绑定到变量
    }
    ```
- 解构枚举
    ```rust
    // 需要 `allow` 来消除警告，因为只使用了枚举类型的一种取值。
    #[allow(dead_code)]
    enum Color {
        // 这三个取值仅由它们的名字（而非类型）来指定。
        Red,
        Blue,
        Green,
        // 这些则把 `u32` 元组赋予不同的名字，以色彩模型命名。
        RGB(u32, u32, u32),
        HSV(u32, u32, u32),
        HSL(u32, u32, u32),
        CMY(u32, u32, u32),
        CMYK(u32, u32, u32, u32),
    }

    fn main() {
        let color = Color::RGB(122, 17, 40);
        // 试一试 ^ 将不同的值赋给 `color`

        println!("What color is it?");
        // 可以使用 `match` 来解构 `enum`。
        match color {
            Color::Red   => println!("The color is Red!"),
            Color::Blue  => println!("The color is Blue!"),
            Color::Green => println!("The color is Green!"),
            Color::RGB(r, g, b) =>
                println!("Red: {}, green: {}, and blue: {}!", r, g, b),
            Color::HSV(h, s, v) =>
                println!("Hue: {}, saturation: {}, value: {}!", h, s, v),
            Color::HSL(h, s, l) =>
                println!("Hue: {}, saturation: {}, lightness: {}!", h, s, l),
            Color::CMY(c, m, y) =>
                println!("Cyan: {}, magenta: {}, yellow: {}!", c, m, y),
            Color::CMYK(c, m, y, k) =>
                println!("Cyan: {}, magenta: {}, yellow: {}, key (black): {}!",
                    c, m, y, k),
            // 不需要其它分支，因为所有的情形都已覆盖
        }
    }
    ```
- 解构指针和引用:对指针来说，解构（destructure）和解引用（dereference）要区分开，因为这两者的概念 是不同的，和 C 那样的语言用法不一样。
  - 解引用使用 *
  - 解构使用 &、ref、和 ref mut
    ```rust
    fn main() {
          // 获得一个 `i32` 类型的引用。`&` 表示取引用。
          let reference = &4;

          match reference {
              // 如果用 `&val` 这个模式去匹配 `reference`，就相当于做这样的比较：
              // `&i32`（译注：即 `reference` 的类型）
              //    |
              // `&val`（译注：即用于匹配的模式）
              // ^ 我们看到，如果去掉匹配的 `&`，`i32` 应当赋给 `val`。
              // 译注：因此可用 `val` 表示被 `reference` 引用的值 4。
              &val => println!("Got a value via destructuring: {:?}", val),
          }

          // 如果不想用 `&`，需要在匹配前解引用。
          match *reference {
              val => println!("Got a value via dereferencing: {:?}", val),
          }

          // 如果一开始就不用引用，会怎样？ `reference` 是一个 `&` 类型，因为赋值语句
          // 的右边已经是一个引用。但下面这个不是引用，因为右边不是。
          let _not_a_reference = 3;

          // Rust 对这种情况提供了 `ref`。它更改了赋值行为，从而可以对具体值创建引用。
          // 下面这行将得到一个引用。
          let ref _is_a_reference = 3;

          // 相应地，定义两个非引用的变量，通过 `ref` 和 `ref mut` 仍可取得其引用。
          let value = 5;
          let mut mut_value = 6;

          // 使用 `ref` 关键字来创建引用。
          // 译注：下面的 r 是 `&i32` 类型，它像 `i32` 一样可以直接打印，因此用法上
          // 似乎看不出什么区别。但读者可以把 `println!` 中的 `r` 改成 `*r`，仍然能
          // 正常运行。前面例子中的 `println!` 里就不能是 `*val`，因为不能对整数解
          // 引用。
          match value {
              ref r => println!("Got a reference to a value: {:?}", r),
          }

          // 类似地使用 `ref mut`。
          match mut_value {
              ref mut m => {
                  // 已经获得了 `mut_value` 的引用，先要解引用，才能改变它的值。
                  *m += 10;
                  println!("We added 10. `mut_value`: {:?}", m);
              },
          }
      }
      ```
- 解构结构体
- 卫语句: 可以加上`match`卫语句（guard） 来过滤分支。
    ```rust
    fn main() {
        let pair = (2, -2);
        // 试一试 ^ 将不同的值赋给 `pair`

        println!("Tell me about {:?}", pair);
        match pair {
            (x, y) if x == y => println!("These are twins"),
            // ^ `if` 条件部分是一个卫语句
            (x, y) if x + y == 0 => println!("Antimatter, kaboom!"),
            (x, _) if x % 2 == 1 => println!("The first one is odd"),
            _ => println!("No correlation..."),
        }
    }
    ```
- 绑定: 在 match 中，若间接地访问一个变量，则不经过重新绑定就无法在分支中再使用它。match 提供了 @ 符号来绑定变量到名称：
  ```rust
  // 示例1:
  // `age` 函数，返回一个 `u32` 值。
  fn age() -> u32 {
      15
  }

  fn main() {
      println!("Tell me type of person you are");

      match age() {
          0             => println!("I'm not born yet I guess"),
          // 可以直接 `match` 1 ..= 12，但怎么把岁数打印出来呢？
          // 相反，在 1 ..= 12 分支中绑定匹配值到 `n` 。现在年龄就可以读取了。
          n @ 1  ..= 12 => println!("I'm a child of age {:?}", n),
          n @ 13 ..= 19 => println!("I'm a teen of age {:?}", n),
          // 不符合上面的范围。返回结果。
          n             => println!("I'm an old person of age {:?}", n),
      }
  }

  // 示例2， 你也可以使用绑定来“结构” enum 变体，例如 Option:
  fn some_number() -> Option<u32> {
      Some(42)
  }

  fn main() {
      match some_number() {
          // Got `Some` variant, match if its value, bound to `n`,
          // is equal to 42.
          Some(n @ 42) => println!("The Answer: {}!", n),
          // Match any other number.
          Some(n)      => println!("Not interesting... {}", n),
          // Match anything else (`None` variant).
          _            => (),
      }
  }
  ```