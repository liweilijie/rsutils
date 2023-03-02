# [译]异步代码中的阻塞操作

本篇原文是:[Blocking inside async code](https://stjepang.github.io/2019/12/04/blocking-inside-async-code.html)。

[译文](https://colobu.com/2020/01/28/blocking-inside-async-code/)

以下是翻译：

大家好，很久没写博文了，这次回来感觉真好。首先带来一个好消息。在Crossbeam上花了两年的时间后，2019年我把我的焦点放在了运行时异步编程研究上(比如`async-std`和`tokio`)。尤其是我想让异步运行时(async runtimes)更有效、更健壮，同时也更简单。

在这篇文章中，我想谈谈所有的运行时都面临的一个有趣的问题：从异步代码中调用阻塞函数。

## Async(异步) 和 Sync(同步)

我们最终在stable rust版本中加上了`async/await`,现在准备重写所有的同步代码，让它们变成异步的。我们应该这么做吗？我不知道。

sync库和async库的差异愈来越大，比如`std`和`async-std`库。看起来两者很相近，除了一个有阻塞函数(sync)，另一个有非阻塞函数(async)。类似的还有[surf](https://docs.rs/surf)和[attohttpc](https://github.com/sbstp/attohttpc)库,它们都是http client库，一个是sync库，一个是async库。现在所有的新库的开发者都面临一个简单的抉择：应该提供一个sync库还是async库，还是两者都提供？

目前重复的API看起来很不幸，但是我个人很乐观，相信最终我们会找出一个办法出来。在任何情况下，我们都需要找到尽可能无缝地集成sync和async代码的方法。

## 从同步到异步，再到同步

rust的`main`函数是同步的，所以为了进入异步世界，我们需要显式的去做。使用`async-std`,我们可以通过调用`block_on()`函数进入异步世界：

```rust
use async_std::task;
// This is sync code.
fn main() {
    task::block_on(foo());
}
// This is async code.
async fn foo() {}
```

如果要从异步世界进入同步世界，可以在异步代码中调用同步代码：

```rust
// This is async code.
async fn foo() {
    bar();
}
// This is sync code.
fn bar() {}
```

所以为了从异步进入同步，我们不需要做任何额外的设置-只需调用同步函数就这么简单，除了...额...我们需要仔细关注需要花费很长时间的同步函数。孔子说在异步世界调用同步函数一定要三思。

## 阻塞影响并发

异步运行时的一个假设就是每次对`future`轮询的时候，它能很快返回`Ready`状态或者`Pending`状态。在异步代码中长时间阻塞是异步编程的一个很大的禁忌，一定要避免发生。

为了说明为什么，下面使用`surf`并发获取40个网页：

```rust
use async_std::task;
use std::time::Instant;
// Fetch the HTML contents of a web page.
async fn get(url: &str) -> String {
    surf::get(url).recv_string().await.unwrap()
}
fn main() {
    task::block_on(async {
        let start = Instant::now();
        let mut tasks = Vec::new();
        // Fetch the list of contributors for the first 40 minor Rust releases.
        for i in 0..40 {
            let url = format!("https://thanks.rust-lang.org/rust/1.{}.0/", i);
            // Spawn a task fetching the list.
            tasks.push(task::spawn(async move {
                let html = get(&url).await;
                // Display the number of contributors to this Rust release.
                for line in html.lines() {
                    if line.contains("individuals") {
                        println!("{}", line.trim());
                    }
                }
            }))
        }
        // Wait for all tasks to complete.
        for t in tasks {
            t.await;
        }
        // Display elapsed time.
        dbg!(start.elapsed());
    });
}
```

这个程序在我的机器上需要1.5秒就可以完成。注意因为`get`函数是异步的，所以获取40个页面是并发执行的。

现在让我们把`get`改成阻塞方式。我们使用`attohttpc`代替`surf`,它们比较类似，除了提供一个同步的接口:

```rust
async fn get(url: &str) -> String {
    attohttpc::get(url).send().unwrap().text().unwrap()
}
```

不出所料，这个程序现在效率更低，需要3秒钟完成。我的计算机有8个逻辑内核，这意味着异步std执行器生成8个工作线程，因此我们一次只能获取8个web页面。

这个例子的教训是：阻塞会损害并发性。很重要的一点是，我们不要在异步代码内部阻塞，否则执行器将无法执行有用的工作-相反，它只会浪费时间阻塞。

## 阻塞无处不在

通过上面的我们看到了异步代码中的阻塞是如何影响性能的。当然，这个例子有点”做作“，因为您只需要使用`surf`而不是`attohttpc`，问题就解决了。但坏消息是，阻塞是不易察觉的：它无处不在，你甚至没有意识到！

考虑**标准输入**和**标准输出**。很明显，读取标准输入块时，不应该在异步代码中使用`std::io::Stdin`。但是如果你看到`println!`你会皱眉头吗！我敢打赌，我们大部分时间都假设打印到标准输出不会阻塞，而它确实是阻塞的。

如果你想知道什么场景下`println!()`会阻塞，可以假想我们在shell中执行`program1 | program2`，这样`program1`的输出就通过管道传输到`program2`中。如果`program2`读取输入的速度非常慢，那么`program1`将不得不在打印某些内容并且管道已满时阻塞。

密集的计算也会导致阻塞。假设我们通过调用`v.sort()`对一个非常大的`Vec`进行排序。如果排序需要一秒钟左右的时间来完成，我们应该考虑将该计算从异步执行器中移除。

有时甚至有一些程序员不太小心会掉进的“陷阱”。例如，假设我们使用`Rayon`在异步代码中调用`v.par_sort()`,人们可能会天真地认为这是可以的，因为排序发生在Rayon的执行器中，而事实是异步执行器仍然会阻塞以等待Rayon的结果。

但性能下降并不是唯一的问题。如果异步执行器的每一个线程都陷在读取标准输入之类的事情上，那么整个程序也有可能陷入**死锁状态**，无法继续执行！

最后，值得一提的是，即使是简单的内存访问也可能被阻塞！例如，考虑驻留在旋转磁盘上的swap memory。如果线程正在访问磁盘上的swap memory，它将不得不阻塞，直到该页从物理磁盘中取出并移到主内存中。

所以阻塞是非常普遍的，很难从异步代码中完全分离出来。我相信我们必须接受这样一个事实：不管我们如何小心地消除阻塞，阻塞总是存在于异步代码中。

## 可能的解决方案

当我们预期在异步代码中阻塞时，我们应该考虑将阻塞逻辑移动到不同的线程池中，这样执行器就可以继续运行而不必等待它。像`async std`和`tokio`这样的运行时提供了[spawn\_blocking()](https://docs.rs/async-std/1.2.0/async_std/task/fn.spawn_blocking.html)函数来帮助解决这个问题。

为了演示如何使用该函数，让我们看看[fs::read\_to\_string()](https://docs.rs/async-std/1.2.0/async_std/fs/fn.read_to_string.html)是如何在`async std`中实现的：

```rust
async fn read_to_string<P: AsRef<Path>>(path: P) -> io::Result<String> {
    let path = path.as_ref().to_owned();
    spawn_blocking(move || std::fs::read_to_string(path)).await
}
```

函数`spawn_blocking()`将闭包生成到专用于运行阻塞函数的特殊线程池中。然后，异步执行器不必阻塞闭包的结果，而是异步`await`返回的`JoinHandle`的结果。

注意，我们不能将对path的引用传递到闭包中，因为在同步版本完成之前，可能会取消`read_to_string()`函数。不幸的是，将路径传递到闭包的唯一方法是克隆它。这有点低效，也有点笨重。

幸运的是，Tokio有一种运行阻塞函数的替代方法：它可以就地执行闭包，并告诉当前线程停止作为异步执行器的一部分，并将该职责移交给一个新线程。在某种程度上，它与`spawn_blocking()`相反——我们没有将闭包发送到新线程并继续事件循环，而是将事件循环发送到新线程并继续运行闭包。

这是`block_in_place()`实现异步`read_to_string()`的方式:

```rust
async fn read_to_string<P: AsRef<Path>>(path: P) -> io::Result<String> {
    block_in_place(|| std::fs::read_to_string(path))
}
```

注意我们不必再clone path，这是因为在内部`sync read_to_string()`完成之前你不可能取消外部的`async read_to_string()`。

虽然`spawn_blocking()`和`block_in_place()`都解决了异步执行器陷入阻塞代码的问题，但它们之间有一个重要的区别。注意`spawn_blocking()`实际上是一个异步函数，因为它返回一个可以等待的`future`，而`block_in_place()`只是一个普通的同步函数。

通过例子看看有什么区别：

```rust
let (s1, s2) = futures::join!(read_to_string("foo.txt"), read_to_string("bar.txt"));
```

如果`read_to_string()`是通过`spawn_blocking()`实现的，那么这两个文件可以并行的读取，而如果是通过`block_in_place()`实现的，那么这两个文件是串行读取的，一个读完才读下一个。

## 结论

关键结论是：

* 在异步代码中阻塞将使性能受损，甚至导致死锁。
* 我们需要使用`spawn_blocking()`或`block_in_place()`隔离程序的阻塞部分。
* 阻塞无处不在，很难完全隔离它。

此外，有时甚至很难说什么代码是阻塞的，什么代码不是阻塞的。如果一个函数需要1秒来完成，我们可能会认为它是阻塞的。但如果需要1毫秒呢？好吧，取决于特定的用例-有时我们应该考虑阻塞，有时我们不应该。这完全取决于你的场景。

阻塞是可怕的，我们需要防御地将它与异步代码隔离开来。但是我们只能做这么多，阻塞仍然不可避免地会潜入到我们的异步代码中。这听起来可能是一个令人悲伤和失望的状况，但我很乐观。我相信有比`spawn_blocking()`和`block_in_place()` 更好的解决方案，我将在下面的博客文章中讨论。

