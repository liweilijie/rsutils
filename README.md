# rust.utils 
作为程序员，总是幻想着有一门相对比较完美的能用的语言，就好比汽车一样，想要一辆比较完美的车，能越野，能操控，能舒服，能皮实耐用，能省油等。这是不太可能的。只是相对来说综合性好一点的。可以用一门语言打通技术全栈，所谓的全栈工程师。

当rust在以太坊上应用，再到filecoin上出现的时候，这种幻想就寄托在它的身上。因为它的理念和先进的特性让人很喜欢。

我希望在有生之年里能尽可能的用rust语言做大部分工作。并且还能很高效的完成，至少不慢于我现在写go的速度其实就能接受。这需要大量的时间和精力去学习rust的方方面面。但愿我能坚持下去，每一天学一个知识点并且整理成自己的文档并且消化掉它，最终的目的是用在自己的项目上，以后项目上用到的地方，在文档里面最好能有体现，以后我会做一个统计我用到的知识点有多少真正用在实际的工作中了。

> 当你对生活充满热情的时候，你就不会变老。
>
> 这个世界上真正只能内心平静的人才能过得安稳,如果我还能活10年的话,我会如何对待我的每一天,对待我身边的人呢?现在这种状态是我想要的生活吗?


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

**运行测试或者代码**
```bash
cargo test -- --color always --nocapture
# 如果指定运行某一个 workspace 的话
cargo test --workspace db -- --color always --nocapture
cargo test -- --show-output
```

## Cargo代理设置

[cargo 代理设置](https://skyao.io/learning-rust/docs/build/cargo/settings.html)

先设置 git 的代理 

```bash
 # 使用 http 代理
git config --global http.proxy http://222.213.23.73:2500

git config --global http.proxy http://127.0.0.1:8001
git config --global https.proxy http://127.0.0.1:8001

# 使用 socks5 代理
git config --global http.proxy socks5://192.168.0.1:23456

# 删除
git config --global --unset http.proxy
git config --global --unset https.proxy
```

`cat ~/.gitconfig` 查看设置结果

但是，上面的设置只对直接使用 git 命令有效，当使用 cargo 命令时，依然会卡住。

需要为 cargo 单独设置代理，新建或打开文件 `~/.cargo/config` ，使用 `http` 代理:

```toml
[http]
proxy = "222.213.23.73:2500"
[https]
proxy = "222.213.23.73:2500"
```
使用`socks5`代理:

```toml
[http]
proxy = "socks5://192.168.0.1:23456"
[https]
proxy = "socks5://192.168.0.1:23456"
```

安装一些常用的编译环境

 ```bash
 sudo apt install build-essential -y
 sudo apt install libssl-dev -y
 sudo apt install  pkg-config -y
 ```

## 编译中文文档

构建参考:
- [https://github.com/rust-lang/rust](https://github.com/rust-lang/rust)
- [https://github.com/wtklbm/rust-library-i18n](https://github.com/wtklbm/rust-library-i18n)


详细的步骤,以**1.58.1**为例:

1. Make sure you have installed the dependencies:
    - g++ 5.1 or later or clang++ 3.5 or later
    - python 3 or 2.7
    - GNU make 3.81 or later
    - cmake 3.13.4 or later
    - ninja
    - curl
    - git
    - ssl which comes in libssl-dev or openssl-devel
    - pkg-config if you are compiling on Linux and targeting Linux
2. Clone the source with git:
```bash
git clone https://github.com/wtklbm/rust-library-i18n.git

git clone https://github.com/rust-lang/rust.git
cd rust

git checkout 1.58.1
rm -rf library
git clone https://github.com/rust-lang/cargo.git src/tools/cargo
git clone https://github.com/rust-lang/rust-installer.git src/tools/rust-installer
git clone https://github.com/
git clone https://github.com/rust-lang/rls.git src/tools/rls
git clone https://github.com/rust-lang/miri.git src/tools/miri
git clone https://github.com/rust-lang/stdarch.git library/stdarch
git clone https://github.com/rust-lang/backtrace-rs.git library/backtrace
git clone https://github.com/rust-lang/libbacktrace library/backtrace/crates/backtrace-sys/src/libbacktrace

# 复制中文目录
cd ../rust-library-i18n/dist
unzip v1.58.1_contrast.zip # contrast是中英对照表,没有此后缀的是只有中文
cp -r library ../../rust/ # 复制到rust目录
cd ../../rust

# 提交一下
git add -A
git commit -m none


# echo -e "changelog-seen = 2\n[llvm]\nninja = false" >> config.toml\n
# 将niaja = false打开
cp config.toml.example config.toml

# 编译,会花很久的时间
./x.py doc

# 完成之后打开
open ./build/x86_64-pc-windows-msvc/doc/std/index.html
```


## vscode
常用的vscode编辑代码

打开settings.json: cmd+shift+p -> open settings.json

- gd
- ctrl+o
- cmd+.

## 笔记

- [rust错误处理](./204_rust-error-handle/README.md)
- [字符串常用备忘](String.md)
- [函数式编程备忘](func.md)
- [泛型](generics.md)
- [并发对比](concurrent.md)
- [异步代码中的阻塞操作](async_block_option.md)
- [智能指针](smart_pointer_in_rust.md)
