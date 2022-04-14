# String 常用方法


## split a string
[https://stackoverflow.com/questions/26643688/how-do-i-split-a-string-in-rust](https://stackoverflow.com/questions/26643688/how-do-i-split-a-string-in-rust) 好好仔细阅读一下.


- Use`.count()`. `len()` is only for`iterators` which know their exact size without needing to be consumed, `count()` consumes the iterator. 
- `split` returns an `Iterator`, which you can convert into a `Vec` using `collect`: `split_line.collect::<Vec<_>>()`. Going through an iterator instead of returning a Vec directly has several advantages:
- `split` is lazy. This means that it won't really split the line until you need it. That way it won't waste time splitting the whole string if you only need the first few values: `split_line.take(2).collect::<Vec<_>>()`, or even if you need only the first value that can be converted to an integer: `split_line.filter_map(|x| x.parse::<i32>().ok()).next()`. This last example won't waste time attempting to process the "23.0" but will stop processing immediately once it finds the "1".
- `split` makes no assumption on the way you want to store the result. You can use a Vec, but you can also use anything that implements `FromIterator<&str>`, for example a LinkedList or a VecDeque, or any custom type that implements `FromIterator<&str>`.

```rust
fn main() {
    let mut split = "some string 123 ffd".split("123");

    // this gives an iterator, which you can loop over, or collect() into a vector

    for s in split {
        println!("{}", s);
    }

    let vec = split.collect::<Vec<&str>>();
}
```

## Collect items from an iterator at a specific index
[https://stackoverflow.com/questions/31986628/collect-items-from-an-iterator-at-a-specific-index](https://stackoverflow.com/questions/31986628/collect-items-from-an-iterator-at-a-specific-index)

```rust
fn main() {
    let line = "Some line of text for example";
    let l = line.split(" ");
    let lvec: Vec<&str> = l.collect();
    let text = &lvec[3];
    // But what would be nice is something like:
    let text: &str = l.collect(index=(3));
    
    // 上面做的这个操作一行代码就搞定了
    // There is a nth function on Iterator that does this:
    let text = line.split(" ").nth(3).unwrap();
}
```

No, it's not; however you can easily filter before you collect, which in practice achieves the same effect.

If you wish to **filter by index**, you need to add the index in and then strip it afterwards:

- `enumerate` (to add the index to the element)
- `filter` based on this index
- `map` to **strip** the index from the element
- 
Or in code:

```rust
fn main() {
    let line = "Some line of text for example";
    let l = line.split(" ")
        .enumerate()
        .filter(|&(i, _)| i == 3 )
        .map(|(_, e)| e);
    let lvec: Vec<&str> = l.collect();
    let text = &lvec[0];
    println!("{}", text);
}
```

