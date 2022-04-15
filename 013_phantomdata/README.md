# PhantomData

现在我们确认了：**在定义数据结构时，对于额外的、暂时不需要的泛型参数， 用 PhantomData 来“拥有”它们，这样可以规避编译器的报错。**
PhantomData 正如其名，它实际上长度为零，是个 ZST（Zero-Sized Type），就像不存在一样，唯一作用就是类型的标记。

## customer 示例

customer 示例请参照 

![customer](./customer.png)