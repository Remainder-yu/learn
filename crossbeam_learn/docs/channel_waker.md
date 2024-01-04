# crossbeam-channel休眠唤醒--阻塞等待
查看crossbeam_channel休眠唤醒机制，可以看出涉及休眠唤醒的主要源码在waker.rs文件中。
```rust
/// Represents a thread blocked on a specific channel operation.
pub(crate) struct Entry {
    /// The operation.
    pub(crate) oper: Operation,

    /// Optional packet.
    pub(crate) packet: *mut (),

    /// Context associated with the thread owning this operation.
    pub(crate) cx: Context,
}

/// A queue of threads blocked on channel operations.
///
/// This data structure is used by threads to register blocking operations and get woken up once
/// an operation becomes ready.
pub(crate) struct Waker {
    /// A list of select operations.
    selectors: Vec<Entry>,

    /// A list of operations waiting to be ready.
    observers: Vec<Entry>,
}

/// A waker that can be shared among threads without locking.
///
/// This is a simple wrapper around `Waker` that internally uses a mutex for synchronization.
pub(crate) struct SyncWaker {
    /// The inner `Waker`.
    inner: Mutex<Waker>,

    /// `true` if the waker is empty.
    is_empty: AtomicBool,
}
```
主要结构体如上。

## 创建注册机制：

## 唤醒机制：
例如，在channel为array情况下，当channel读取操作read时，会唤醒发送者sender，例如，`self.senders.notify();`其中senders为 SyncWaker类型。唤醒过程如下：
```rust
pub(crate) fn notify(&self)：impl SyncWaker
--> pub(crate) fn notify(&mut self): impl Waker
    --> entry.cx.try_select(Selected::Operation(entry.oper)).is_ok()
            |
            |--> pub fn try_select(&self, select: Selected) -> Result<(), Selected>:crossbeam-channel/src/context.rs
            | 在try_select内部主要是操作inner.select状态，比较他的值与Selected::Waiting.into(),进行原子操作
        --> entry.cx.unpark(); 
            --> self.inner.thread.unpark();实际上还是操作的thread的unpark函数。
        
```
同理，在write的时候如果写入成功，也会唤醒对端的接收事件self.receivers.notify()。


## 阻塞机制：

```rust
pub(crate) fn recv(&self, deadline: Option<Instant>) -> Result<T, RecvTimeoutError> 
--> self.receivers.register(oper, cx);//先注册
--> let sel = cx.wait_until(deadline);
    --> pub fn wait_until(&self, deadline: Option<Instant>) -> Selected :crossbeam-channel/src/context.rs
        --> return match self.try_select(Selected::Aborted)
        --> thread::park(); // 这个就是当前线程park
```
可以发现都会在实际不同情况进入context的 try_select方法，所以可以详细分析具体代码逻辑。
```rust
    pub fn try_select(&self, select: Selected) -> Result<(), Selected> {
        self.inner
            .select
            .compare_exchange(
                Selected::Waiting.into(),
                select.into(),
                Ordering::AcqRel,
                Ordering::Acquire,
            )
            .map(|_| ())
            .map_err(|e| e.into())
    }
```
同理，在发送send操作的过程中，也会如上进行操作注册，然后如果满足阻塞条件，就会阻塞线程。

也就是实际就是在读写过程中主要实现唤醒操作，在发送和接收过程中实现阻塞操作。这个逻辑其实就是，当你写入数据到channel中时，写入成功即唤醒接收者接收。当接收者接收数据时，如果接收数据失败且满足阻塞条件，则阻塞当前线程，直到被唤醒或者超时。
