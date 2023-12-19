
```rust
/// Split the head value into the real head and the index a stealer is working
/// on.
fn unpack(n: UnsignedLong) -> (UnsignedShort, UnsignedShort) {
    let real = n & UnsignedShort::MAX as UnsignedLong;
    let steal = n >> (mem::size_of::<UnsignedShort>() * 8);

    (steal as UnsignedShort, real as UnsignedShort)
}
```
这个函数在tokio中用于将一个整数值拆分为真正的头部和一个索引，该索引表示一个stealer正在处理的位置。
函数名为unpack，接收一个名为n的UnsignedLong类型参数，并返回一个包含两个UnsignedShort类型的元组。
函数内部的逻辑如下：
real变量表示从n中提取的真正的头部值。它通过对n与UnsignedShort::MAX的按位与操作进行计算得到。UnsignedShort::MAX是UnsignedShort类型的最大值，通过将其强制类型转换为UnsignedLong来进行按位与操作。
steal变量表示从n中提取的表示stealer正在处理位置的索引。它通过对n进行右移操作来计算，右移的位数是UnsignedShort类型占据的字节数乘以8。
函数返回一个元组，包含steal强制类型转换为UnsignedShort和real强制类型转换为UnsignedShort的结果。
换句话说，这个函数的作用是将一个整数值拆分为两部分，第一部分是指定stealer正在处理的位置的索引，第二部分是真正的头部值。这在并发编程中常用于实现任务窃取算法，用于将工作任务均匀地分配给不同的处理器以实现负载均衡和并行处理。

```rust

```

## Cargo.toml

work_stealing:
怎么拿（如何获取task）：
全局队列：
本地队列：
通过steal：

runtime中的thread_pool如何调入task？


12.19：

## mio--
io：async_fd：
driver就是为了实现通知的结果。

poll方法就会重新进行poll：

```rust
        match self.poll.poll(events, max_wait) {
            Ok(()) => {}
            Err(ref e) if e.kind() == io::ErrorKind::Interrupted => {}
            #[cfg(target_os = "wasi")]
            Err(e) if e.kind() == io::ErrorKind::InvalidInput => {
                // In case of wasm32_wasi this error happens, when trying to poll without subscriptions
                // just return from the park, as there would be nothing, which wakes us up.
            }
            Err(e) => panic!("unexpected error when polling the I/O driver: {:?}", e),
        }
```
readiness怎么去做唤醒？
turn函数？
driver--是不是和mio绑定。

tokio对外原则，封装不暴露？ 对下面库的封装，为了安全。
driver：怎么调度起来？

driver中的pack？怎么调用？

 driver如何实现调度？
 wake？readiness--标记是否可以使用？
 driver可以理解位reactor，如何理解reactor？
 为什么READINESS执行位运算？为什么放在一个变量，原因在于原子，可以在一个地方替换？无锁数据结构，为了利用原子操作。
 为了配合硬件效率。。。

 loom：测试框架。
 哪里用户调用？哪里是driver调用？