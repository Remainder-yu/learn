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
