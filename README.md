# rust.utils 
作为程序员，总是幻想着有一门相对比较完美的能用的语言，就好比汽车一样，想要一辆比较完美的车，能越野，能操控，能舒服，能皮实耐用，能省油等。这是不太可能的。只是相对来说综合性好一点的。可以用一门语言打通技术全栈，所谓的全栈工程师。

当rust在以太坊上应用，再到filecoin上出现的时候，这种幻想就寄托在它的身上。因为它的理念和先进的特性让人很喜欢。

我希望在有生之年里能尽可能的用rust语言做大部分工作。并且还能很高效的完成，至少不慢于我现在写go的速度其实就能接受。这需要大量的时间和精力去学习rust的方方面面。但愿我能坚持下去，每一天学一个知识点并且整理成自己的文档并且消化掉它，最终的目的是用在自己的项目上，以后项目上用到的地方，在文档里面最好能有体现，以后我会做一个统计我用到的知识点有多少真正用在实际的工作中了。

> 当你对生活充满热情的时候，你就不会变老。


## 关于学习曲线 
Rust是被公认的很难学的语言，学习曲线很陡峭。其实你只要经过一段时期的思维转换，练习心智模型就可以了。

- 从命令式编程语言转换到函数式编程语言
- 从变量的可变性迁移到不可变性
- 从弱类型语言迁移到强类型语言
- 从手工或者自动内存管理到通过生命周期来管理内存 

这是常用的一些比较实用的**rust代码**，甚至是一些**项目**。或者一些代码**片断**。 目的就是希望能在写`rust`的时候开发速度能跟得上现在用`go`的开发速度。

## Rust真的值得我们花那么多时间学习吗？

- Rust能尽可能的将Bug解决在编译期。
- 无GC的实时控制
- zero cost abstraction 保证了性能
- ownership和lifetime很强
- 异步高并发

## 归档列表

项目所有列表

- [001_thread-event-loop](./001_thread-event-loop/README.md) 利用thread做的一个简单的event-loop并发应用。
- [002_httpie](./002_httpie/README.md) 类似curl的一个http请求及返回的CLI命令工具的简单实现。


## 命名规则 

- `0`开头的是最最基础的知识
- `1`开头的是标准库的使用教程
- `2`开头的是第三方库的常用方法
- `4`开头的是随便写的项目
- `5`开头的也是随便写的项目

## 常用命令

**rust环境相关的**
```bash
rustup show
# 升级最新的stable版本
rustup update stable
```

**生成项目**
```bash
cargo new 002_httpie --bin --name httpie
cargo new 204_error --bin --name errors
```

**安装crates以及使用features**
```bash
cargo install cargo-edit
cargo add anyhow colored jsonxf mime
cargo add clap --allow-prerelease
cargo add reqwest --features json
cargo add tokio --features full
```

## vscode
常用的vscode编辑代码

打开settings.json: cmd+shift+p -> open settings.json

- gd
- ctrl+o
- cmd+.
