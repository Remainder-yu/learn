# channel
## 概述
Crossbeam库中的channel模块提供了多种不同类型的通道（channel）实现，用于在多线程之间进行安全的消息传递和同步。
通道是线程间通信的一种方式，它允许一个线程将消息发送给另一个线程，并在需要时进行同步等待。
### 分类：
at – 在一定时间后传递消息的通道。消息会在特定的时间间隔后被发送到通道中，用于计划、定时或延迟任务的场景。
array – 基于预分配数组的有界通道。该通道具有预定义的容量限制，可以确保发送方和接收方之间的消息传递满足一定的限制条件。
list – 作为链表实现的无界通道。该通道没有容量限制，可以一直接收新的消息，适用于无需限制消息数量的场景。
never – 从不传递消息的通道。该通道无论何时都不会传递消息，可以用于模拟一些特殊情况或实现某些特定的行为。
tick – 周期性传递消息的通道。该通道会按照设定的时间间隔周期性地发送消息，适用于定期执行某些操作的场景。
zero – 零容量通道。该通道没有缓冲区，发送方和接收方必须同时准备好才能进行消息传递，用于确保发送和接收操作的同步性和精确性。
这些不同的通道风味在Crossbeam库中提供了不同的选择，以满足不同的需求和场景。开发者可以根据具体的使用场景选择适合的通道类型，以实现高效和可靠的消息传递和同步。
### 特点

安全性：Crossbeam通道的实现考虑多线程的并发安全性。它们在不需要额外的锁或同步原语的情况下实现了线程安全，确保消息的传递和同步操作是无竞争的。
灵活性：Crossbeam提供了多种通道类型，如有界通道、无界通道、定时通道等。这使得开发人员可以根据具体的使用场景和需求选择合适的通道类型。
效率：Crossbeam通道的实现采用了高效的算法和数据结构，以最大程度地减少内存开销和线程间的同步操作次数。这使得通道的传递和同步操作具有较低的延迟和较高的吞吐量。
通道的作用主要体现在以下几个方面：
消息传递：通道允许一个线程将消息发送给另一个线程。通过通道，线程可以安全地将数据或信息发送给其他线程，实现线程之间的数据交换和协作。
同步等待：通道提供了一种同步机制，允许线程在等待接收消息时进行阻塞。当线程尝试从通道接收消息时，如果通道为空，线程将被阻塞，直到有消息可用为止。这种同步等待的机制可以用于线程协调和任务调度。
线程间通信：通道为多个线程提供了一种可靠的通信机制，使得线程之间可以进行有效的协作和通信。通过发送和接收消息，线程可以共享数据、共享状态，以及进行任务的分发和协同处理。
总而言之，Crossbeam中的通道提供了一种安全、灵活、高效的线程间通信机制，用于实现多线程并发编程中的消息传递和同步等待操作。这对于构建并发应用程序和解决并行计算问题非常有价值。

## array

# waker篇
Q1：crossbeam中的waker唤醒机制原理？基于thread::park和unpark？unpark原理又是什么？
Crossbeam是一个用于并发编程的Rust库，其中包含了一些用于线程同步和通信的原语。在Crossbeam中，Waker是一种唤醒机制，用于在线程间进行通信和唤醒操作。
Waker的工作原理基于Rust标准库中的thread::park和thread::unpark机制，但在Crossbeam中经过了一些优化和扩展。
thread::park是一个用于暂停线程的方法，它会使线程进入等待状态，直到被唤醒。当线程调用park方法时，它会将自己放入一个等待队列中，并且不会继续执行，直到被其他线程调用unpark方法唤醒。
而在Crossbeam中，Waker是通过thread::park和thread::unpark机制实现的。在Waker的实现中，会创建一个与等待线程相关联的处理器（handler）。处理器内部维护了一个计数器，用于跟踪唤醒次数。当线程调用park方法后，它会将自己放入等待队列，并且等待被唤醒。而当其他线程调用unpark方法时，会将与待唤醒线程相关联的处理器从等待队列中取出，并且增加计数器的值。
当唤醒次数达到一定阈值或者处理器超过一定时间没有被唤醒时，Waker会通过将线程放回就绪队列，以便继续执行。这些优化能够减少不必要的线程唤醒开销，并提高并发性能。
总结起来，Crossbeam中的Waker机制利用了Rust标准库的thread::park和thread::unpark方法，通过等待队列和处理器来实现线程的暂停和唤醒。它通过优化唤醒次数和等待时间，减少了不必要的线程唤醒开销，并提高了并发性能。

```rust
pub(crate) struct SyncWaker {
    /// The inner `Waker`.
    inner: Mutex<Waker>,

    /// `true` if the waker is empty.
    is_empty: AtomicBool,
}

pub(crate) struct Waker {
    /// A list of select operations.
    selectors: Vec<Entry>,

    /// A list of operations waiting to be ready.
    observers: Vec<Entry>,
}

pub(crate) struct Entry {
    /// The operation.
    pub(crate) oper: Operation,

    /// Optional packet.
    pub(crate) packet: *mut (),

    /// Context associated with the thread owning this operation.
    pub(crate) cx: Context,
}
```

参考文献：
https://xiaopengli89.github.io/posts/crossbeam-channel/
[Crossbeam的无锁并发Channel解析](https://xiaopengli89.github.io/posts/crossbeam-channel/)

# channel

```rust
            Context::with(|cx| {
                // Prepare for blocking until a receiver wakes us up.
                let oper = Operation::hook(token);
                self.senders.register(oper, cx);

                // Has the channel become ready just now?
                if !self.is_full() || self.is_disconnected() {
                    let _ = cx.try_select(Selected::Aborted);
                }

                // Block the current thread.
                let sel = cx.wait_until(deadline);

                match sel {
                    Selected::Waiting => unreachable!(),
                    Selected::Aborted | Selected::Disconnected => {
                        self.senders.unregister(oper).unwrap();
                    }
                    Selected::Operation(_) => {}
                }

```
在crossbeam-channel/src/flavors/array.rs中的line332的send函数中可知，在发送过程中，会进行本地的上下文获取Context::with(|cx|，然后注册至syncWaker中，接下来需要进入waker.rs中进行分析。然后就会注册至self.register_with_packet(oper, ptr::null_mut(), cx);注册到self.selectors.push。这个就是context与waker之间的主要关系，实现了休眠唤醒的实现。

主要问题是：select又是怎么实现了channel的可选择执行？


## select

在SelectedOperation结构体中，最重要的就是index参数，可以通过该参数找到对应的sender或者receiver。例如：let oper = sel.select();获得了SelectedOperation<'a>，然后根据oper.recv(&r2)找到对应的接收通道。
```rust
pub struct SelectedOperation<'a> {
    /// Token needed to complete the operation.
    token: Token,

    /// The index of the selected operation.
    index: usize,

    /// The address of the selected `Sender` or `Receiver`.
    ptr: *const u8,

    /// Indicates that `Sender`s and `Receiver`s are borrowed.
    _marker: PhantomData<&'a ()>,
}
```
找到接收通道了，然后执行channel::read(r, &mut self.token)，即可通过该操作获取channel对应相关的操作。select操作只是为了在多个channel找到可以读取或者写入那个channel。相当于channel内部做了一次管理，让channel传输数据更加有效。
```rust
    pub fn recv<T>(mut self, r: &Receiver<T>) -> Result<T, RecvError> {
        assert!(
            r as *const Receiver<T> as *const u8 == self.ptr,
            "passed a receiver that wasn't selected",
        );
        let res = unsafe { channel::read(r, &mut self.token) };
        mem::forget(self);
        res.map_err(|_| RecvError)
    }

```
通过channel-select的联合，我们可以认知到select的作用。


### trait SelectHandle
在crossbeam-channel/src/select.rs文件中，定义了trait SelectHandle，也就是select的一些基本操作。
然后`impl<T: SelectHandle> SelectHandle for &T`实现了操作，

例如，`impl<T> SelectHandle for Receiver<'_, T>`，针对Receiver实现了trait SelectHandle，而Receiver由`pub(crate) struct Receiver<'a, T>(&'a Channel<T>);`封装,以针对Receiver实现的watch为例，watch注册channel的receiver端事件。
```rust
impl<T> SelectHandle for Receiver<T> {

    fn watch(&self, oper: Operation, cx: &Context) -> bool {
        match &self.flavor {
            ReceiverFlavor::Array(chan) => chan.receiver().watch(oper, cx),
            ReceiverFlavor::List(chan) => chan.receiver().watch(oper, cx),
            ReceiverFlavor::Zero(chan) => chan.receiver().watch(oper, cx),
            ReceiverFlavor::At(chan) => chan.watch(oper, cx),
            ReceiverFlavor::Tick(chan) => chan.watch(oper, cx),
            ReceiverFlavor::Never(chan) => chan.watch(oper, cx),
        }
    }
}
怎么通过array-channel调用到chan.receiver().watch(oper, cx)；
    /// Returns a receiver handle to the channel.
    pub(crate) fn receiver(&self) -> Receiver<'_, T> {
        Receiver(self)
    }

然后调用： ReceiverFlavor::Array(chan) => chan.receiver().watch(oper, cx),
针对array：
    fn watch(&self, oper: Operation, cx: &Context) -> bool {
        self.0.receivers.watch(oper, cx);
        self.is_ready()
    }

根据channel的receivers：syncWaker成员调用对应的watch注册等待时间：
impl syncWaker {
        pub(crate) fn watch(&self, oper: Operation, cx: &Context) {
        let mut inner = self.inner.lock().unwrap();
        inner.watch(oper, cx);
        self.is_empty.store(
            inner.selectors.is_empty() && inner.observers.is_empty(),
            Ordering::SeqCst,
        );
    }
}

然后调用：
impl Waker {
    pub(crate) fn watch(&mut self, oper: Operation, cx: &Context) {
        self.observers.push(Entry {
            oper,
            packet: ptr::null_mut(),
            cx: cx.clone(),
        });
    }
}

```

### notify-waker的实现
针对array-channel中，send函数会调用write将数据msg写入通道，然后调用Context::with获取当前上下文环境，然后将channel对应的sender注册self.senders.register(oper, cx);操作类型Operation::hook(token);返回的就是pub struct Operation(usize);类型，然后阻塞当前线程let sel = cx.wait_until(deadline);
根据wait_until返回的Selected具体的枚举类型执行对应的操作。针对Selected::Operation(_) => {}，说明operation不做任何操作。

同时在send发送过程中，循环执行let res = unsafe { self.write(token, msg) };其中write写入数据至通道，完成后唤醒接收端self.receivers.notify();整个唤醒过程后续分析：
主要核心就是entry.cx.unpark(); //通知相关线程可以执行被选择的操作。怎么唤醒对应的操作呢，就是entry.cx.try_select(Selected::Operation(entry.oper))找到对应的entry。

其实在注册过程中，self.senders.register(oper, cx);就将operation与context绑定在一起然后后续唤醒可以通过operation匹配对应的context。

所以send过程写入操作过程唤醒对方，然后同时确定是否需要阻塞自己并注册至waker。

然后如果需要分析详细过程，需要分析waker源码。

```rust
pub enum Selected {
    /// Still waiting for an operation.
    Waiting,

    /// The attempt to block the current thread has been aborted.
    Aborted,

    /// An operation became ready because a channel is disconnected.
    Disconnected,

    /// An operation became ready because a message can be sent or received.
    Operation(Operation),
}

    /// Writes a message into the channel.
    pub(crate) unsafe fn write(&self, token: &mut Token, msg: T) -> Result<(), T> {
        // If there is no slot, the channel is disconnected.
        if token.array.slot.is_null() {
            return Err(msg);
        }

        let slot: &Slot<T> = unsafe { &*token.array.slot.cast::<Slot<T>>() };

        // Write the message into the slot and update the stamp.
        unsafe { slot.msg.get().write(MaybeUninit::new(msg)) }
        slot.stamp.store(token.array.stamp, Ordering::Release);

        // Wake a sleeping receiver.
        self.receivers.notify();
        Ok(())
    }

```
唤醒接收端之前将自己注册至syncwaker，然后一直阻塞当前线程cx.wait_until(deadline);

```rust
                Context::with(|cx| {
                // Prepare for blocking until a receiver wakes us up.
                let oper = Operation::hook(token);
                self.senders.register(oper, cx);

                // Has the channel become ready just now?
                if !self.is_full() || self.is_disconnected() {
                    let _ = cx.try_select(Selected::Aborted);
                }

                // Block the current thread.
                let sel = cx.wait_until(deadline);

                match sel {
                    Selected::Waiting => unreachable!(),
                    Selected::Aborted | Selected::Disconnected => {
                        self.senders.unregister(oper).unwrap();
                    }
                    Selected::Operation(_) => {}
                }
            });
```
这段代码中，首先使用Context::with方法创建一个上下文cx，用于管理并发操作。
接下来，创建一个代表发送操作的Operation对象oper，并将其注册到self.senders（发送者集合）中，以便执行该操作。
然后，检查通道是否已满或者是否已断开连接。如果通道未满或者已断开连接，尝试用cx.try_select(Selected::Aborted)来选择一个操作，表示发送操作中止。
接着，使用cx.wait_until(deadline)方法阻塞当前线程，等待直到指定的截止时间。
最后，根据sel的不同取值，执行对应的操作。如果sel为Selected::Aborted或Selected::Disconnected，表示发送操作被取消，需要从发送者集合中注销该操作；如果sel为Selected::Operation(_)，表示操作执行成功；如果sel为Selected::Waiting，表示出现了意外情况，即代码无法到达的位置，使用unreachable!()宏触发panic。
需要注意，sel是在cx.wait_until(deadline)方法中确定的，代表在指定的截止时间内选择的操作。具体的选择逻辑是由Context和相关操作句柄协同完成的。这里的代码片段没有给出Selected的定义，但可以假设Selected是一个枚举类型，表示选择操作的结果。
