threadpool库是Rust编程之道的例子，可以参考该书进行学习。

创建线程时需要传入闭包，怎么处理呢？

```rust
type Thunk<'a> = Box<dyn FnBox + Send + 'a>;
```
闭包？Thunk代表什么？
`Thunk<'a>` 是一个类型别名，表示一个不带参数且返回类型为 `'a` 的可执行闭包（closure）。
`Box<dyn FnBox + Send + 'a>` 是一个 trait 对象，可以存储任意实现了 `FnBox + Send + 'a trait` 的类型。其中 FnBox 是一个 trait，表示可以被调用的闭包，Send 表示该闭包可以安全地在线程间传递，'a 表示闭包中可能引用的数据的生命周期。
这个类型别名用于定义线程池中的任务类型，可以存储任意的可执行闭包，并且能够在多线程环境中安全地使用。

为了让线程池中维护的线程可以共享相同的数据，还需要一个共享数据的结构体：
```rust
struct ThreadPoolSharedData {
    name: Option<String>, //用于标记线程的名称，线程池内的线程都用统一的名称
    job_receiver: Mutex<Receiver<Thunk<'static>>>,//
    empty_trigger: Mutex<()>,
    empty_condvar: Condvar,
    join_generation: AtomicUsize,
    queued_count: AtomicUsize,
    active_count: AtomicUsize,
    max_thread_count: AtomicUsize,
    panic_count: AtomicUsize,
    stack_size: Option<usize>,
}
```
* job_receiver:用于存储从channel中接收任务的接收端rx，线程安全，所以才会需要加锁。
* 
