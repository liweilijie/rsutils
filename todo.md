# 自己思考的知识汇总

fs::write()接口
RHS(right hand side) 泛型参数的功能
clap 命令行参数解析熟练掌握
as_ref()什么时候用
map()的使用 `self.downstream.as_ref().map(|v|v.clone())`
RefCell::borrow_mut()内部可变借用的使用
struct Node {
  id: usize,
  // 使用 Rc<RefCell<T>> 让节点可以被修改
  downstream: Option<Rc<RefCell<Node>>>,
}
 如果要在多线程的环境下使用： downstream: Option<Arc<Mutex<T>>> 或者
 downstream: Option<Arc<RwLock<T>>>
 lazy_static! 解决全局变量的问题


## project

- [https://rustrepo.com/](https://rustrepo.com/)

- [https://github.com/fredr/data-exporter](https://github.com/fredr/data-exporter)
- [https://github.com/kbknapp/wireguard_exporter](https://github.com/kbknapp/wireguard_exporter)
- [https://github.com/blobcode/kagi.git](https://github.com/blobcode/kagi.git) simple kv sotre
- [https://github.com/blobcode/pine.git](https://github.com/blobcode/pine.git) A simple hyper-based reverse proxy built in rust.
- [https://github.com/ttys3/static-server](https://github.com/ttys3/static-server) web server
