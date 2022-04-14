# 函数式编程

## filter 为什么 double reference
[https://stackoverflow.com/questions/64303118/why-do-i-get-double-references-when-filtering-a-vec](https://stackoverflow.com/questions/64303118/why-do-i-get-double-references-when-filtering-a-vec)

It looks like the vector doesn't hold Move's at all, but merely references to Moves? But then, where are the Move's stored? In addition, the `filter()` function adds another level of indirection. Is this correct? Please explain to me!

No, `Vec<Move>` definitely holds moves. The part you're missing is that aside from `filter()` getting a reference to the iterator item, [slice::iter](https://doc.rust-lang.org/std/slice/struct.Iter.html) creates an **iterator on references** to the slice (or vec here) items, so `Vec<Move>` -> `Iterator<Item=&Move>` -> `filter(predicate: FnMut(&&Move) -> bool)`, and that's why you've got `two indirections` in your `filter` callback.

When I have vector elements with a type that implements Copy, is there a way to avoid all this useless reference taking stuff. I understand how it would make sense with vector elements of a notable size one does not want to copy around. However, I definitely want to avoid `&&value` in `filter()`. Can I?

Yes. You can use `into_iter` which will consume the source vector but iterate on the contained values directly, or you can use the `Iterator::copied` adapter which will Copy the iterator item, therefore going from `Iterator<Item=&T>` to `Iterator<Item=T>`. However filter will never get a T, the most it can get is an &T since otherwise the item would get "lost" (it would be consumed by the filter, which would only return a boolean, yielding… nothing useful).

The alternative is to use something like `filter_map` which does get a `T` input, and returns an `Option<U>`. Because (as the name indicates) it both `filters` and `maps`, it gets to consume the input item and either return an output item (possibly the same) or return "nothing" and essentially remove the item from the collection.

Incidentally, there's also an `Iterator::cloned` adapter for types which are Clone but not (necessarily) Copy.

Also you could have basically done that by hand by just flipping map and filter around in the original:

```rust
    vec.iter().map(|m| *m).filter(|m| self.apply(*m).notInCheck()).collect()
```

`map` transforms the `Iterator<Item=&T>` into an `Iterator<Item=T>`, then `filter` just gets an `&T` instead of an `&&T`.

That aside, I don't really get why apply needs to consume the input move. Or why rawMoves doesn't just… create the vector internally and return it? I get the optimisation allowing for reusing the buffer but that seems like a case of premature optimisation maybe?

And your Move seems… both over-complicated and a bit too simple? If you just want to newtype a u32 then using a tuple-struct seems more than sufficient.

And repr(transparent) is wholly unnecessary, it's only a concern in FFI contexts where the newtype is intended as a type-safety measure which only exists on the Rust side (aka the newtype itself is not visible from / exposed to C, only the wrapped type is).