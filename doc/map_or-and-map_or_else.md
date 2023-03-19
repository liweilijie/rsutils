# rust 中 map_or() 和 map_or_else()用法

[from](https://blog.csdn.net/linysuccess/article/details/124873579)

`Option<T>`和`Result<T,E>`中都定义了`map_or/map_or_else`函数，函数内部通过`match`模式匹配，快速提取`Option/Result`内含的值，并进行进一步的处理。

## map_or()函数

先来看看`Option<T>`中`map_or()`函数的定义：

```rust
pub fn map_or<U, F: FnOnce(T) -> U>(self, default: U, f: F) -> U {
    match self {
        Some(t) => f(t),
        None => default,
    }
}
```

- 第一个参数为`self`，表明`map_or`是`Option`对象的成员方法（不是通过结构名称可以直接调用的静态方法）。
- 第二个参数`default`，是调用者传入的一个默认返回值，跟`map_or()`的返回值类型相同，都是泛型类型**U**。 当`Option`对象本身是`None`时，会将此`default`作为`map_or()`的返回值返回出去。
- 第三个参数`f`，其类型`F`是带一个`T`类型输入参数，返回`U`类型的`FnOnce`函数/闭包。其中`T`类型就是`Option<T>`中的泛型类型`T`。 
- `map_or`最终返回的是`U`类型。

可以看到，`map_or`内部封装了一个`match`匹配，可以在链式调用中，方便地提取`Option`中的值：

- 如果本身是`Some(t)`，那么函数调用结果`f(t)`将会被作为最终的返回值。
- 如果本身是`None`，直接返回传给`map_or`的第一个参数（这里把`self`当做第0个参数）作为最终的返回值。

接下来，通过定义如下一个`is_even()`函数来判断一个`Option<i32>`是不是偶数(None不算偶数）。以此来熟悉`map_or`的用法：

```rust
// 对于一个Option<i32>类型的x，如果x是None，直接返回传给map_or的第一个参数值（false），
// 如果x中有值，则会通过第二个闭包进行处理：i32的值是偶数返回true，是奇数返回false。
fn is_even(x: Option<i32>) -> bool {
    x.map_or(false, |x|x%2==0)
}
println!("{}", is_even(None));//false
println!("{}", is_even(Some(1)));//false
println!("{}", is_even(Some(2)));//true
```

## map_or_else()

`map_or_else`跟`map_or`不同的是，第一个参数也是FnOnce函数（不同的是，这是一个没有输入的闭包，因为当前只有一个None可用，没必要有输入），

在`match`遇到`None`时，会调用此函数，而不是直接返回一个默认值。

```rust

    /// Maps a `Result<T, E>` to `U` by applying fallback function `default` to
    /// a contained [`Err`] value, or function `f` to a contained [`Ok`] value.
    ///
    /// This function can be used to unpack a successful result
    /// while handling an error.
    ///
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// let k = 21;
    ///
    /// let x : Result<_, &str> = Ok("foo");
    /// assert_eq!(x.map_or_else(|e| k * 2, |v| v.len()), 3);
    ///
    /// let x : Result<&str, _> = Err("bar");
    /// assert_eq!(x.map_or_else(|e| k * 2, |v| v.len()), 42);
    /// ```
    pub fn map_or_else<U, D: FnOnce(E) -> U, F: FnOnce(T) -> U>(self, default: D, f: F) -> U {
        match self {
            Ok(t) => f(t),
            Err(e) => default(e),
        }
    }
```

示例：

```rust
fn my_unwrap(x: Option<u8>) -> i32 { 
    x.map_or_else(||-1, |x|x as i32)
}
println!("{}", my_unwrap(None));//-1
println!("{}", my_unwrap(Some(1)));//1
println!("{}", my_unwrap(Some(2)));//2
```


## 实战

实战：判断递增列表

> 定义一个函数，判断一个列表中的数是否是递增的（要求用`map_or`来实现）：

```rust
fn is_ascending(x: &Vec<i32>) -> bool {
    let mut iter = x.iter();
    // 取出迭代器中第一个元素进行map_or
    iter.next().map_or(true, |mut prev| {
    	// 遍历迭代器中除第一个元素外，其余的元素
        for cur in iter {
            if cur >= prev {
                prev = cur;
            } else {
                return false;
            }
        }
        true
    })
}
println!("{}", is_ascending(&vec![]));//true
println!("{}", is_ascending(&vec![1,2,5]));//true
println!("{}", is_ascending(&vec![9,2,5]));//false
```

## 补充

`Result＜T,E＞`中的`map_or/map_or_else`也是同样的用法。不同之处，仅仅在于`Option<T>`中用于匹配`None`的情况，在`Result＜T,E＞`中被换成了`Err(e)`。
