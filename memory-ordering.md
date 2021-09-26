# rust并发编程-memory ordering
在并发编程领域，Rust 提供了完善的机制来保证并发编程的安全，我们可以非常方便的使用 Mutex，Arc，Channel 等库来处理我们的并发逻辑。 但在有些时候，为了更高效的性能，我们可能会去写一些 lock-free(无锁) 的数据结构，而 Rust 自身也提供了 atomic 的支持。

对于每个 atomic 操作，都需要显示的指定 Ordering，Rust 提供了 Relaxed，Release，Acquire，AcqRel，以及 SeqCst 这些 Ordering 的支持，使用不同的 Ordering 会让编译器或者 CPU 对某些指令进行重新排序执行，所以为了更正确的写出 lock-free 的代码，了解这些 Ordering 是如何工作的，就显得非常重要了。

## Keywords
在开始介绍 Rust 的 memory ordering 之前，我们需要知道两个常用的用来描述atomic 操作之间关系的概念，synchronizes-with 和 happens-before：

Synchronizes-with - 简单来说，两个线程 A 和 B，以及一个支持原子操作的变量 x，如果 A 线程对 x 写入了一个新的值（store），而 B 线程在 x 上面读取到了这个新的值（load），我们就可以认为，A 的 store 就是 synchronizes-with B 的 load 的。

Happens-before - 这应该算是一个更基础的概念。如果一个操作 B 能看到之前操作 A 产生的结果，那么 A 就是 happens-before B 的。譬如在单线程里面，如果一个操作 A 的语句在操作 B 的前面执行，通常叫做 sequenced-before，那么 A 就是 happens-before B 的，譬如这样：
```rust
vec.push(1);  // A
ready = true; // B
```
而对于跨线程（inter-thread） 的情况，要判断 happens-before，就需要借助于前面提到的 synchronizes-with 了。如果操作 A 是 synchronizes-with 另一个线程的操作 B 的，那么 A 就是 happens-before B 的。Happens-before 也具有传递性，如果 B 是 happens-before C 的，那么 A 也是 happens-before C。

如果 A 是 sequenced-before B，而 B 是 inter-thread happens-before C 的，那么 A 也是 inter-thread happens-before C。同理，如果 A inter-thread happens-before B，而 B sequenced-before C，那么 A 也是 inter-thread happens-before C。

可以看到，要写出正确的 atomic 代码，尤其是在多线程环境下面，关键就是要弄清楚两个 atomic 操作的 syncrhonizes-with 关系。而这个不同的 Ordering 是不一样的。

## Relaxed ordering
Relaxed ordering 只能保证原子操作，在同一个线程里面，对同一个变量的操作会满足 happens-before 关系，但对于 inter-thread 来说，它不能提供 synchronizes-with 支持，并不保证任何顺序。

下面是一个简单的例子：
```rust
fn write_x_then_y() {
    X.store(true, Ordering::Relaxed);
    Y.store(true, Ordering::Relaxed);
}

fn read_y_then_x() {
    while !Y.load(Ordering::Relaxed) {}
    if X.load(Ordering::Relaxed) {
        Z.fetch_add(1, Ordering::SeqCst);
    }
}

fn main() {
    let t1 = thread::spawn(move || {
        write_x_then_y();
    });

    let t2 = thread::spawn(move || {
        read_y_then_x();
    });

    t1.join().unwrap();
    t2.join().unwrap();

    assert_ne!(Z.load(Ordering::SeqCst), 0);
}
```
上面 assert 可能会失败，也就是 Z 的值在最后可能为 1。在函数 write_x_then_y 里面即使 store X happens-before store Y，即使在 read_y_then_x 里面 load Y 返回了 true，X 的值仍然可能是 false。因为对 X 和 Y 的两个操作都是 relaxed 的，虽然对于不同的线程，两个 load 或者两个 store 都可能满足 happens-before，但在 store 和 load 之间，并没有相关的约束，也就是意味着 load 可能看到乱序的 store。

通常来说，relaxed 适用的场景就是需要对某个变量进行原子操作，而且不需要考虑多个线程同步的情况，譬如 reference counter，其它时候需要考虑有更强约束的其他 ordering。

## Acquire-Release ordering
Acquire 和 Release 通常都是需要成对使用的，当对 store 使用 Release ordering 之后，后续任何的 Acquire ordering 的 load 操作，都会看到之前 store 的值。也就是说，通过 Acquire-Release，我们能支持 synchronizes-with。
```rust
fn write_x_then_y() {
    X.store(true, Ordering::Relaxed);
    Y.store(true, Ordering::Release);
}

fn read_y_then_x() {
    while !Y.load(Ordering::Acquire) {}
    if X.load(Ordering::Relaxed) {
        Z.fetch_add(1, Ordering::SeqCst);
    }
}

fn main() {
    let t1 = thread::spawn(move || {
        write_x_then_y();
    });

    let t2 = thread::spawn(move || {
        read_y_then_x();
    });

    t1.join().unwrap();
    t2.join().unwrap();

    assert_ne!(Z.load(Ordering::SeqCst), 0);
}
```

不同于之前的 relaxed，这里我们对 Y 使用了 Acquire 和 Release，那么最后 Z 就一定不可能为 0 了。主要是因为 store Y 是 synchronizes-with load Y 的，也就是 store Y happens before load Y，因为 store X 是 sequenced-before store Y，那么 store X 就是 happens-before load X 的。

通常，我们还可以使用 AcqRel ordering，它其实就是组合了 Acquire 和 Release，对于 load 使用 Acquire，而对于 store 则是使用 Release。

## Sequence ordering
Sequence ordering 不光提供了 Acquire，Release 的 ordering 支持，同时也确保所有线程会看到完全一致的原子操作顺序。
```rust
fn write_x() {
    X.store(true, Ordering::SeqCst);    // 1
}

fn write_y() {
    Y.store(true, Ordering::SeqCst);    // 2
}

fn read_x_then_y() {
    while !X.load(Ordering::SeqCst) {}
    if Y.load(Ordering::SeqCst) {       // 3
        Z.fetch_add(1, Ordering::SeqCst);  
    }
}

fn read_y_then_x() {
    while !Y.load(Ordering::SeqCst) {}
    if X.load(Ordering::SeqCst) {       // 4
        Z.fetch_add(1, Ordering::SeqCst); 
    }
}

fn main() {
        let t1 = thread::spawn(move || {
        write_x();
    });

    let t2 = thread::spawn(move || {
        write_y();
    });

    let t3 = thread::spawn(move || {
        read_x_then_y();
    });

    let t4 = thread::spawn(move || {
        read_y_then_x();
    });

    t1.join().unwrap();
    t2.join().unwrap();
    t3.join().unwrap();
    t4.join().unwrap();

    assert_ne!(Z.load(Ordering::SeqCst), 0);
}
```
上面的例子，只有使用 SeqCst ordering，才能保证 Z 最后的值不为 0，任何其他的 ordering，都不能保证，我们来具体分析一下。因为两个 read 函数都是有 while 循环，退出之前一定能确保 write 函数被调用了。因为使用 SeqCst 能保证所有线程看到一致的操作顺序，假设 3 返回了 false，表明 X 为 true，而 Y 为 false，这时候一定能保证 store Y 还没调用，一定能保证 store X 在 store Y 之前发生，4 就一定会返回 true。

如果这里我们对 load 使用 Acquire，而对 store 使用 Release，read_x_then_y 和 read_y_then_x 可能看到完全相反的对 X 和 Y 的操作顺序。

SeqCst 在有些时候，可能会有性能瓶颈，因为它需要确保操作在所有线程之前全局同步，但是它其实又是最直观的一种使用方式， 所以通常，当我们不知道用什么 ordering 的时候，用 SeqCst 就对了。

## Memory fence
出了使用不同的 ordering，我们还可以使用 memory fence 来支持 synchronizes-with，如下：
```rust
fn write_x_then_y() {
    X.store(true, Ordering::Relaxed); // 1
    fence(Ordering::Release);         // 2
    Y.store(true, Ordering::Relaxed); // 3   
}

fn read_y_then_x() {
    while !Y.load(Ordering::Relaxed) {}  // 4
    fence(Ordering::Acquire);            // 5
    if X.load(Ordering::Relaxed) {       // 6
        Z.fetch_add(1, Ordering::SeqCst);
    }
}

fn main() {
    let t1 = thread::spawn(move || {
        write_x_then_y();
    });

    let t2 = thread::spawn(move || {
        read_y_then_x();
    });

    t1.join().unwrap();
    t2.join().unwrap();

    assert_ne!(Z.load(Ordering::SeqCst), 0);
}
```
在上面的例子中，2 Release fence 是 synchronizes-with 5 Acquire fence 的，而4 load Y 的时候一定会读取到 3 store Y 的值，加上 1 store X 是 sequenced-before 3 的，那么自然能确定 1 是 happens-before 6 的。也就是 Z 一定不会等于 0。

## Epilogue
可以看到，要弄清楚 memory ordering，其实并不是一件容易的事情，不过多数时候，为了不出错，使用 SeqCst 就成。


## from

- [rust并发编程-memory ordering](https://www.jianshu.com/p/511cde6b62a6)