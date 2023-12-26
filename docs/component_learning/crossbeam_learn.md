
# crossbeam概述 
## learn note(参考资料)

参考资料：
[crossbeam](https://cloud.tencent.com/developer/article/1651494)
[crossbeam](https://github.com/crossbeam-rs/crossbeam.git)

crossbeam是rust并发库，代表性的mpmc channel。其实crossbeam对无锁并发有很多支持。
1. 有锁并发、无锁并发和crossbeam简介
2. crossbeam-epoch：基于epoch的无锁垃圾收集，以及reiber_stack的例子
3. crossbeam-deque：work-stealing算法
4. crossbeam-channel：与std中channel的对比，channel的设计哲学
5. crossbeam-queue和crossbeam-utils：并发队列和杂七杂八的实用小工具
6. crossbeam-skiplist：无锁数据结构之Skip lists

有锁并发、无锁并发和crossbeam
## 有锁并发
线程数量比较大的时候，大部分的时间被用在了同步上（等待锁能被获取），性能就会变得非常差。

## 无锁并发
无锁对象：如果一个共享了无论其他线程做任何操作，总会有一些线程会在有限的操作的系统操作步骤后完成一个对其的操作。
也就是说，至少有一个线程对其操作会取得成效。使用锁的并发明显就不属于这一范畴：如果获取了锁的线程被延迟，那么这段时间里没有任何线程能够完成任何操作。极端情况下如果出现了死锁，那么没有任何线程能够完成任何操作。
CAS（compare and swap）原语
那大家可能会好奇，无锁并发要怎么实现呢？有没有例子呢？在此之前让我们先看一下一个公认的在无锁并发中非常重要的原子原语：CAS。CAS的过程是用指定值去比较一储存值，只有当他们相同时，才会修改储存值为新的指定值。CAS是个原子操作（由处理器支持，比如x86的compare and exchange (CMPXCHG)），该原子性保证了如果其他线程已经改变了储存值，那么写入就会失败。Rust标准库中的std::sync::atomic中的类型就提供了CAS操作，比如原子指针std::sync::atomic::AtomicPtr

```rust
pub fn compare_and_swap(
    &self,
    current: *mut T,
    new: *mut T,
    order: Ordering
) -> *mut T

```
尝试用CAS替换原来的数据。如果快照和数据一致，说明这一期间数据没有被写操作过，于是更新成功。如果不一致，说明在此期间有其他线程修改过数据，那么一切从头再来。这就是一个无锁的栈。
而crossbeam也提供了工具去方便我们更容易的写出代码。
下回就让我们深入crossbeam-epoch去看一下基于epoch的垃圾收集工具，以及如何使用它来完成我们的无锁并发栈。

### CAS：
CAS 是 Compare and Swap（比较并交换）的缩写，是一种并发编程中用于实现原子操作的机制。
在多线程或并发环境中，多个线程可能同时访问和修改共享的数据。为了确保数据的一致性和避免竞态条件（race condition），需要使用原子操作来保证数据的正确性。
CAS 是一种原子操作，它通过比较内存中的值与预期值是否相等，如果相等则将新值写入内存，否则不做任何操作。这个比较和写入操作是原子执行的，即在整个操作过程中不会被其他线程中断。
CAS 操作通常有三个参数：内存地址（或变量）、预期值和新值。它会先读取内存地址中的值与预期值进行比较，如果相等，则将新值写入内存地址，并返回 true，表示操作成功。如果不相等，则不做任何操作，并返回 false，表示操作失败。
CAS 操作可以用于实现各种并发算法和数据结构，如无锁数据结构、线程同步等。它提供了一种高效且线程安全的方式来更新共享数据，避免了使用锁带来的性能开销和死锁等问题。
需要注意的是，CAS 操作虽然可以保证原子性，但仍然需要注意处理操作失败的情况，以避免数据不一致或竞态条件的问题。

# crossbeam-epoch
由于实现一个无锁的并发栈，但是由于Rust没有GC，简单的实现会导致内存泄露。于是crossbeam提供了一个基于epoch的垃圾收集（epoch based reclamation）库。首先来简单的说一下这一垃圾回收的原理。

场景：首先，对于一块内存，当我们从逻辑上移除了它时，我们不知道究竟是否有其他线程还在访问这块的内容，所以我们不能直接在物理上释放那块内存。对于这种情况，我们能做的是把需要释放内存的地址（也就是一个数值）保存下来，在之后的某个可以判定没有其他线程访问时再释放它。

## 基于epoch的垃圾回收


# crossbeam-deque
## 参考文档：
https://cloud.tencent.com/developer/article/1689822
https://github.com/crossbeam-rs/crossbeam/pull/290
https://websockets-rs.github.io/rust-websocket/doc/crossbeam_deque/index.html
https://docs.rs/crossbeam-deque/0.7.2/crossbeam_deque/struct.Injector.html
https://blog.csdn.net/u012067469/article/details/107273831

## 解决的问题
Crossbeam-deque是一个并发队列库，它提供了多线程环境下的双端队列数据结构。它的主要目标是在高并发情况下提供高性能和可伸缩性。
## crossbeam的双向队列
Arora，Blumofe和Plaxton[1]基于Blumofe&Leiserson[4]，提出了使用yield系统调用以及无锁数据结构的无锁工作窃取算法，即ABP工作窃取算法。（论文中后者使用的就是上一讲中提到的CAS。为什么需要CAS呢？试想，当双向队列只有一个元素，而窃取和本地取任务同时发生时就会产生竞态。基本上和上一讲提到的无锁并发栈的问题类似）。而Chase&Lev[2]则是改进了Blumofe&Leiserson[4]的deque，使其在保持简洁和高性能的同时，底层不受限于固定长数组(固定长数组会有溢出的问题)。而crossbeam的deque是在Chase&Lev[2]的deque的基础上又作出了一些改进：（注意，接下来我们就不讨论处理器中的线程调度，而是线程中的任务调度问题了。比如tokio，goroutine面临的都是这样的问题）

1. 支持先进先出的工作者队列（既本地可以当队列而不是栈使用）
2. 支持一次窃取多个任务
3. 加入了一个注水器队列（Injector queue），和原来的工作者队列可以配合使用。
这里我们先来说一下这个注水器队列。这是一个先进先出MPMC队列（任务从一端入队，从另一端被窃取），被线程共享（全局）。新的任务可以被放到这个队列中供空闲的线程窃取。相对于将新任务放进工作者队列的，这一操作更加公平（即只要有空闲的线程，这个队列的处理就会有进展）
### 主要原理
Crossbeam-deque的主要数据结构是一个双向链表Injector，链表的每个节点是一个块（Block），块内部包含一个固定大小的数组（Slots）。每个槽位（Slot）可以存储一个任务（Task）。
双向链表的头部和尾部由两个指针（head和tail）来标识。head指针指向链表的头部，tail指针指向链表的尾部。在并发环境下，head和tail指针需要使用原子操作来保证并发安全。
在Crossbeam-deque中，每个块（Block）的大小是固定的，块的大小由常量BLOCK_CAP指定。块的大小通常是2的幂次方，这样可以通过位运算来计算索引在块中的偏移量，而无需进行除法运算。
除了双向链表和块数组外，Crossbeam-deque还使用了一些辅助的原子变量，如计数器（count）和标志位（stealer_flag和reclaimer_flag），来帮助实现并发操作的正确性。

>Q:为什么需要保证并发的正确？哪些数据需要保证并发的正确性？怎么保证并发的正确性？

在Crossbeam-deque中，任务的推入（push）和弹出（pop）操作都是通过修改head和tail指针来实现的。推入操作将任务添加到队列的尾部，弹出操作从队列的头部取出任务。
在推入操作中，会先判断当前块是否已满，如果已满，则需要分配一个新的块，并将新块添加到链表的尾部。然后将任务添加到块的槽位中，并更新tail指针。
在弹出操作中，会先判断当前链表是否为空，如果为空，则返回None。否则，从链表的头部取出任务，并更新head指针。
Crossbeam-deque还提供了一些其他的功能，如批量推入（batch_push）和批量弹出（batch_pop）操作，用于提高推入和弹出操作的性能。
**总之，Crossbeam-deque通过双向链表和块数组的组合，结合原子操作和辅助变量，实现了一个高性能且可伸缩的并发队列数据结构。**

## work-stealing算法简介
crossbeam-deque包提供了一个无锁的双向队列(deque)。那么这个双向队列在并发中启动什么重要作用？涉及重要算法：work-stealing算法，即工作窃取算法。
最初，工作窃取算法是在join、fork模型下，作为调度算法用来给多线程计算分配任务的。简单说就是：每个处理器先处理自己的任务，如果处理完了，就去别的处理器的任务列表中头一个过来执行。
**与之相对的有一个work-sharing算法，该算法中，新产生的任务由调度算法直接分配给相应的处理器，每个处理器都是被动的。**
> 而在工作窃取算法中，每个处理器是主动的。

### 以下是该算法的一个概述，其中每个处理器都维护一个双向队列。

初始状态下，计算是一个线程（类比一个main）函数并被分配给某个处理器，而其他处理器都处于空闲状态。处于空闲状态的处理器会立即执行窃取操作。每个处理器按指令逐一执行当前线程，直到遇到以下四种情况：
1. 遇到spawn指令，产生一个新的线程。当前线程被放入双向队列底部，处理器开始执行新的线程。
2. 线程被挂起（处于阻塞状态）。这个时候处理器会从双向队列的底部取出一个线程去执行。如果双向队列为空，那么处理器就会去窃取。
3. 指令导致线程死亡，这时和2相同处理。
4. 指令激活了另一个线程，此时被激活的线程会放入双向队列的底部，处理器继续执行现在的线程，窃取操作：处理器随机选取另一个处理器，如果被选择的处理器的双向队列非空，那么从该队列的头部取出一个线程并开始执行，否则再次进入随机选取。

>窃取算法的关键就是双向队列：从本地任务总是从栈顶（也即双向队列的底部）取出，这在crossbeam中被称为工作者队列（worker queue）。而在窃取时，则把它当作队列来使用：总是从队列的头部窃取。

work-stealing算法是一种实现任务并行的算法，主要用于多核处理器.
主要思想：当某个处理器的任务执行完毕后，他可以从其他处理器的任务队列中偷取一些任务来执行，以充分利用系统资源，提高系统的并行度和性能。
### working-stealing算法实质

通常是一个双端队列的数据结构。每个处理器都有自己的任务队列，也就是双端队列的一端，该队列用于存放自己的任务。当一个处理器处理完自己的任务后，他会从其他处理器的队列中随即选择一些任务。如果其他处理器的队列中没有任务可偷，则该处理器会轮询其他队列，指导找到可以偷的为止。
work-stealing算法的优点在于它可以充分利用系统资源，提高系统的并行度和性能。它不需要进行任务调度，因为每个处理器都可以自主选择要执行的任务，这样可以减少任务调度的开销。同时，它也可以避免任务之间的互斥和同步问题，因为每个任务都是独立的，没有共享的状态。
需要注意的是，在使用work-stealing算法时，需要考虑任务的负载均衡问题。如果某个处理器的任务比其他处理器的任务多，那么它就会成为系统的瓶颈，导致系统的性能下降。因此，需要采取一些策略来均衡任务的分配，使得每个处理器的任务负载尽可能均衡。

crossbeam-deque包提供了一个无锁双向队列（deque）。那么这个双向队列在并发中起到什么作用？
"crossbeam-deque"是Crossbeam库中的一个模块，用于实现无锁的双端队列（deque）。Deque是一种数据结构，支持在队列的两端进行插入和删除操作。
Crossbeam-deque提供了一种高效的无锁实现，可以在多个线程之间进行并发访问。它使用一种基于追加-弹出（push-pop）操作的算法，可以在多个线程之间高效地共享和处理任务。
Crossbeam-deque的主要特点包括：

无锁：使用了一套无锁算法，没有互斥锁的开销，适合高并发环境。
双端操作：支持在队列的两端进行插入和删除操作，可以根据需要选择合适的操作。
高效：通过使用CAS（Compare and Swap）原子指令来实现并发操作，提供了高性能的并发处理能力。
可扩展：可以动态地增加或减少队列的容量，以适应不同的并发负载。
使用Crossbeam-deque可以方便地构建并发任务调度器、工作线程池等并发应用。它提供了一种无锁的队列实现，可以在多个线程之间高效地共享和处理任务。


## Injector注水器

```rust
pub struct Injector<T> {
    //TODO The head of the queue.
    head: CachePadded<Position<T>>,

    //@REMAINDER The tail of the queue.
    tail: CachePadded<Position<T>>,

    /// Indicates that dropping a `Injector<T>` may drop values of type `T`.
    _marker: PhantomData<T>,
}
```
### push
该操作核心：用于将任务添加到队列中。代码中使用了自旋锁和原子操作来确保并发安全。

首先，代码使用了Backoff类型的实例来提供自旋等待的功能。
然后，代码通过原子加载操作获取队列尾部的索引和块。
接下来，代码进入一个循环，直到成功地将任务添加到队列中。
在循环中，代码首先计算索引在块中的偏移量，如果偏移量等于BLOCK_CAP（块容量），说明当前块已满，需要等待下一个块的安装。此时，代码会调用backoff.snooze()方法来等待一段时间，然后重新加载索引和块。
如果偏移量加1等于BLOCK_CAP，并且下一个块尚未分配，则代码会提前分配一个新的块。
接下来，代码尝试使用原子比较交换操作将队列尾部的索引向前移动。如果成功，说明当前线程已经成功将任务添加到队列中，并且可以安全地返回。如果失败，则说明其他线程已经修改了队列的尾部索引，此时代码会重新加载索引和块，并使用backoff.spin()方法进行自旋等待。
如果成功将任务添加到队列中，代码会首先判断是否需要安装下一个块。如果需要安装，则将下一个块的指针存储到当前块的next字段中，并更新队列尾部的块指针和索引。
最后，代码将任务写入到块的指定槽位，并设置相应的状态位。
总之，这段代码实现了一个并发安全的任务队列的push操作，使用了自旋锁和原子操作来确保多线程环境下的正确性。

```rust
// 先进先出MPMC队列（任务从一端入队，从另一端被窃取）
struct Injector<T>;

// 本地的工作者队列
struct Worker<T>;

// 用来从相应的工作者队列窃取任务
struct Stealer<T>;

#[must_use]
enum Steal<T> {
    Empty,
    Success(T),
    Retry,
}

impl<T> Injector<T> {
    fn new() -> Injector<T>;

    fn is_empty(&self) -> bool;
    fn len(&self) -> usize;
    fn push(&self, task: T);

    // 从队列窃取一个任务
    fn steal(&self) -> Steal<T>;

    // 从队列窃取多个任务并交给目标工作者
    fn steal_batch(&self, dest: &Worker<T>) -> Steal<()>;

    // 从队列窃取多个任务并交给工作者，并弹出第一个任务
    fn steal_batch_and_pop(&self, dest: &Worker<T>) -> Steal<T>;
}

impl<T> Worker<T> {
    // 初始化一个先进先出工作者队列
    fn new_fifo() -> Worker<T>;
    // 初始化一个后进先出工作者队列
    fn new_lifo() -> Worker<T>;

    // 从当前队列产生一个窃取者
    fn stealer(&self) -> Stealer<T>;
    fn is_empty(&self) -> bool;

    fn push(&self, task: T);
    fn pop(&self) -> Option<T>;
}

impl<T> Stealer<T> {
    fn is_empty(&self) -> bool;

    // 从队列窃取一个任务
    fn steal(&self) -> Steal<T>;

    // 从队列窃取多个任务并交给目标工作者
    fn steal_batch(&self, dest: &Worker<T>) -> Steal<()>;

    // 从队列窃取多个任务并交给工作者，并弹出第一个任务
    fn steal_batch_and_pop(&self, dest: &Worker<T>) -> Steal<T>;
}

impl<T> Steal<T> {
    fn is_empty(&self) -> bool;
    fn is_success(&self) -> bool;
    fn is_retry(&self) -> bool;

    // 如果是Success(T)则返回内容
    fn success(self) -> Option<T>;

    // 如过没有steal到任务，则执行F
    fn or_else<F: FnOnce() -> Steal<T>>(self, f: F);
}

// 一直找到一个Success(T)为止
impl<T> FromIterator<Steal<T>> for Steal<T>;

```

```rust
use crossbeam_deque::Deque;
use crossbeam_utils::thread;
fn main() {
    let deque = Deque::new();
    thread::scope(|s| {
        // 线程1进行入队操作
        s.spawn(|_| {
            for i in 0..10 {
                deque.push(i);
            }
        });
        // 线程2进行出队操作
        s.spawn(|_| {
            for _ in 0..5 {
                while let Some(item) = deque.pop() {
                    println!("Thread 2: {}", item);
                }
            }
        });
        // 线程3进行窃取操作
        s.spawn(|_| {
            for _ in 0..5 {
                while let Some(item) = deque.steal() {
                    println!("Thread 3: {}", item);
                }
            }
        });
    })
    .unwrap();
    // 验证最终的结果
    while let Some(item) = deque.pop() {
        println!("Main Thread: {}", item);
    }
}

```

在上面的示例中，创建了一个 Deque，并在三个线程中进行并发的入队、出队和窃取操作。最后，主线程进行验证，输出最终的结果。
需要注意的是，为了使用 Crossbeam 库，需要在 Cargo.toml 文件中添加以下依赖项：

```rust
[dependencies]
crossbeam = "0.8"
crossbeam-deque = "0.8"
crossbeam-utils = "0.8"
```
## 主要函数接口及实现原理
### steal_batch_with_limit

1. 首先，通过`self.inner.front.load(Ordering::Acquire)`加载Deque的前端索引`f`
2. 然后，通过`epoch::is_pinned()`判断当前线程是否已经固定在一个时代中，如果是，则手动发出`Ordering::SeqCst`的屏障
3. 接下来，通过`epoch::pin()`方法来固定当前线程在一个时代中。
4. 然后，通过`self.inner.back.load(Ordering::Acquire)`加载`Deque`的后端索引`b`.
5. 接着，通过判断b - f是否小于等于0来判断队列是否为空，如果为空，则返回Steal::Empty。
6. 接下来，计算可偷取的任务数量，即`min(limit, b-f)`,并创建一个`Vec<T>`类型的同期用于存储偷取的任务
7. 通过`self.inner.buffer.load(Ordering::Acquire, guard`加到缓冲区，并将偷取的任务赋值到容器中。
8. 然后，尝试增加前端索引`f`来更新偷取的任务数量，如果缓冲区已经被交换或者增加索引的操作失败，则返回`Steal::Retry`
9. 最后，通过steal::Success(tasks)返回成功偷取的任务，其中tasks是包含偷取任务的容器。

总之，该方法是允许以批量方式从Deque中偷取一定数量的任务，并通过时代机制和原子操作保证了并发安全性。

代码分析：

`atomic::fence(Ordering::Release);`
以上代码片段是在steal_batch_with_limit_and_pop方法中的一部分，用于进行一个释放语义的屏障操作。下面是对代码的解释：
atomic::fence(Ordering::Release)是一个原子操作，用于在当前线程执行到这个位置时，确保之前的所有内存操作都已经完成，并且对其他线程可见。
使用Ordering::Release参数可以确保当前线程的所有内存操作都在这个屏障之前完成，并且对其他线程可见。这样可以保证在屏障之前的所有数据修改都在屏障之后对其他线程可见。
释放语义的屏障操作常用于将数据从当前线程发布到其他线程，确保其他线程能够正确地观察到已经发布的数据。
这段代码的作用是在某个特定的位置，通过释放语义的屏障操作来保证之前的内存操作对其他线程可见，以确保在后续操作中其他线程能够正确地观察到已经发布的数据。

```rust
struct Backoff{
    step: Cell<u32>,
}
```
在crossbeam库中，pub struct Backoff是一个用于自旋等待的结构体。
Cell是rust标准库中的一个类型，用于提供内部可变性。它允许我们在不可变引用的情况下修改其内部的值。
在Backoff结构体中，step字段用于记录当前自旋等待的步数。它允许我们在不可变引用的情况下修改其内部的值。
在Backoff结构体中，step字段用于记录当前自旋等待的步数。在自旋等待的过程中，可以使用step字段来控制自旋等待的策略，例如每次自旋等待时递增步数，以实现一种退避backoff策略。
通过使用Backoff结构体，可以在并发编程中实现一种简单的自旋等待机制。这种机制可以用于等待某个条件的满足，以避免线程频繁地进行无效的轮询。通过适当地调整自旋等待的策略，可以在一定程度上提高并发程序的性能。

# crossbeam scope
```rust
crossbeam::scope(|s| {
    s.spawn(|_| {
        for i in 0..n_msgs {
            snd.send(i).unwrap();
            thread::sleep(time::Duration::from_millis(100));
        }
    });
}).unwrap();
```
crossbeam scope函数来创建一个线程作用域，确保在作用域结束后所有的子线程都已经完成。在作用域中，使用spawn函数创建了一个新的子线程，并在子线程中循环发送数据。
使用crossbeam的scope函数创建了一个线程作用域。scope函数接受一个闭包作为参数，并在闭包中创建子线程。

在作用域中，使用spawn函数创建了一个新的子线程。spawn函数接受一个闭包作为参数，并在子线程中执行该闭包。

子线程的闭包使用for循环发送数据到snd通道，并使用thread::sleep函数休眠100毫秒。

使用unwrap函数处理可能的错误，确保操作成功。
通过这段代码，可以在一个作用域中创建子线程，并在子线程中发送一系列数据到通道。使用crossbeam的scope函数可以有效地管理线程的生命周期，并确保在作用域结束后所有的子线程都已经完成。

crossbeam crate中的scope函数提供了一种方便的方式来创建线程作用域，它的原理和作用如下原理：
scope函数使用了Rust的生命周期（lifetime）机制来管理线程的生命周期。
在scope函数的闭包中创建的子线程，其生命周期与scope函数的生命周期相同，也就是说，在scope函数执行完毕后，所有在scope闭包中创建的子线程都会被等待并加以回收。
作用：
线程作用域（thread scope）是一种用于管理线程生命周期的模式。它可以确保在作用域结束后，所有的子线程都已经完成并被回收，避免了线程的泄漏或提前终止。
使用scope函数可以避免手动创建和管理线程的问题，以及手动等待和回收线程的麻烦。
在scope函数的作用域内，可以方便地创建多个子线程，并在子线程中执行需要的操作，无需手动管理线程的生命周期。
scope函数还能够有效地避免资源竞争问题，因为所有的子线程都在作用域结束后被回收，不会在作用域外继续执行。
总之，crossbeam crate中的scope函数通过利用Rust的生命周期机制，提供了一种方便、安全和高效的方式来管理线程的生命周期，避免了手动管理线程和资源竞争的问题。它是在并发编程中非常有用的工具。



# crossbeam中的CachePadded

在crossbeam库中，CachePadded是一个用于填充缓存行（cache line）的结构体包装器。缓存行是计算机内存中的一小段连续存储区域，通常为64字节（或者其他特定大小）。
CachePadded结构体的作用是将其内部的字段进行填充，以确保每个字段都位于不同的缓存行中。这样做的目的是为了避免不同字段之间的数据竞争（false sharing），从而提高并发程序的性能。
在多线程并发执行的场景中，如果不同线程同时访问位于同一缓存行中的字段，就会导致缓存行的无效ation（invalidation）和更新（update）。这种无效ation和更新的开销可能会降低并发程序的性能。
通过使用CachePadded结构体，可以将不同字段分散到不同的缓存行中，从而避免不同线程之间的数据竞争，减少缓存行的无效ation和更新的开销，提高并发程序的性能。
需要注意的是，CachePadded结构体只是一个包装器，它没有额外的方法或功能。它的主要作用是通过填充来调整内存布局，以提高并发性能。

只是调整了内存布局。

# 补充知识点

#### unsafecell
是标准库中的一个类型，它是用于在Rust中实现内部可变性。
在Rust中，引入unsafecell的目的是为了允许在不可变引用之下修改其内部的值，而不违反Rust的借用规则。UnsafeCell可以看作是一种黑魔法，它绕过了借用检查器，允许在使用内部可变性的场景下进行不安全的操作。

使用UnsafeCell非常小心，因为它绕过了Rust的安全保障，可能引入内存安全和数据竞争问题，因此，通常情况下应该避免直接使用UnsafeCell的实例，而是使用Mutex、Rwlock或Cell。

#### PhantomData

下列代码是用于标识该结构体的实例不能在多个线程之间共享。
```rust
/// Indicates that the worker cannot be shared among threads.
    _marker: PhantomData<*mut ()>, // !Send + !Sync
```
PhantomData：标准库类型，用于对泛型参数的占位符。
在这个代码中，PhantomData 的泛型参数是 *mut ()，表示一个指向 () 类型的可变指针。*mut () 表示一个不可变的原始指针类型，指向一个没有实际值的空单位类型 ()。这个类型在 Rust 中通常用作占位符，表示不需要实际的数据。
PhantomData<*mut ()> 的作用是告诉编译器，这个字段在类型系统上具有特定的属性，即不可发送（!Send）和不可同步（!Sync）。这意味着该字段不能在多个线程之间共享，并且不能跨线程传递给其他线程。
通过在结构体中添加这个字段，可以在编译时防止将该结构体的实例传递给其他线程或共享给其他线程，从而避免了多线程环境下的数据竞争和并发问题。
需要注意的是，PhantomData 本身没有运行时开销，它只是一种编译时的辅助工具。在这个代码中，PhantomData<*mut ()> 的作用是通过类型系统来禁止共享该结构体实例，而不是通过运行时的机制来实现。

#### crossbeam_deque ：注水器
工作窃取算法是一种用于实现任务调度的技术，它通常用于多线程环境下的任务并行处理。
注水器在该算法的作用，当一个线程需要从其他线程的工作队列中偷取任务时，它会先通过注水器获取一些任务，然后再去偷取其他线程的任务。注水器可以看成是一个临时缓冲区，用于存储其他线程偷取的任务，以减少线程之间的竞争。

在crossbeam_deque中，注水器的具体实现是Injector结构体。它包含了一个任务队列和一个计数器。
当一个线程需要从其他线程偷取任务时，它会先尝试从注水器中获取任务。如果注水器中有任务，则直接返回；否则，它会通过工作窃取算法从其他线程的工作队列中偷取任务，并将偷取到的任务存储在注水器中。使用注水器可以提高工作窃取算法的效率，减少线程之间的竞争。它在crossbeam_deque中被用于实现线程池等并发场景，以提高任务调度和执行效率。

#### MaybeUninit
`MaybeUninit<T>`是标准库一个类型，它用于表示未初始化的类型T的值，它提供了一种安全的方式来处理可能存在未初始化的情况。
为了确保内存安全性，变量在使用之前被正确初始化。但有些情况下，我们可能需要延迟初始化一个变量，或者在特定条件下才初始化变量。
这时，就可以使用MaybeUninit<T>。
MaybeUninit<T>类型具有以下特点：

* 它是一个零大小类型（zero-sized type），不会分配任何内存空间。
* 它的值可能是未初始化的，即它不保证包含有效的值。
* 它提供了一些方法来安全地处理未初始化的值，如assume_init()和write()。
使用MaybeUninit<T>可以确保对未初始化的值进行正确的处理，避免了悬空指针和未定义行为等问题。

```rust
use std::mem::MaybeUninit;
fn main() {
    let mut uninitialized_value: MaybeUninit<i32> = MaybeUninit::uninit();
    // 在特定条件下才初始化变量
    let condition = true;
    if condition {
        unsafe {
            uninitialized_value.as_mut_ptr().write(42);
        }
    }
    // 安全地将MaybeUninit<T>转换为T类型
    let initialized_value: i32 = unsafe {
        uninitialized_value.assume_init()
    };
    println!("Initialized value: {}", initialized_value);
}
```
需要注意的是，对于`MaybeUninit<T>`类型的值进行读取操作前，必须确保它已经被正确初始化，否则可能会导致未定义行为。

# 概述
crossbeam-qeue分为两种无锁队列，一种是利用数组实现，一种是链表实现。
基于数据实现有限队列性能更高。
基于链表实现的无限队列，因为需要动态分配所以性能较差。
从两种数据结构的封装开始分析。

两种数据结构都是无锁队列，基于原子操作实现，所以性能较高。其中包括crossbeam特有得一些技巧。

# array_queue

其中主要的数据结构：`pub struct ArrayQueue<T> `，其中除了位置标记参数和容量属性等，最为核心就是存放数据成员的`struct Slot<T>`。其主要封装结构如下：

```rust
/// A slot in a queue.
struct Slot<T> {
    /// The current stamp.
    ///
    /// If the stamp equals the tail, this node will be next written to. If it equals head + 1,
    /// this node will be next read from.
    stamp: AtomicUsize,

    /// The value in this slot.
    value: UnsafeCell<MaybeUninit<T>>,
}

pub struct ArrayQueue<T> {
    /// The head of the queue.
    ///
    /// This value is a "stamp" consisting of an index into the buffer and a lap, but packed into a
    /// single `usize`. The lower bits represent the index, while the upper bits represent the lap.
    ///
    /// Elements are popped from the head of the queue.
    head: CachePadded<AtomicUsize>,

    /// The tail of the queue.
    ///
    /// This value is a "stamp" consisting of an index into the buffer and a lap, but packed into a
    /// single `usize`. The lower bits represent the index, while the upper bits represent the lap.
    ///
    /// Elements are pushed into the tail of the queue.
    tail: CachePadded<AtomicUsize>,

    /// The buffer holding slots.
    buffer: Box<[Slot<T>]>,

    /// The queue capacity.
    cap: usize,

    /// A stamp with the value of `{ lap: 1, index: 0 }`.
    one_lap: usize,
}
```
由于该结构体需要线程同步，所以需要实现：send,sync等特征trait。

针对`ArrayQueue<T>`结构体主要方法：
```rust
// 核心成员方法，分析以下几个即可
pub fn new(cap: usize) -> Self { }
fn push_or_else<F>(&self, mut value: T, f: F) -> Result<(), T>
pub fn push(&self, value: T) -> Result<(), T>
pub fn force_push(&self, value: T) -> Option<T>
pub fn pop(&self) -> Option<T>

pub fn capacity(&self) -> usize
pub fn is_empty(&self) -> bool
pub fn is_full(&self) -> bool
pub fn len(&self) -> usize
```

### new

```rust
// One lap is the smallest power of two greater than cap.
        let one_lap = (cap + 1).next_power_of_two();·
```
这段代码的作用是计算一个大于cap的最小的2的幂值。
具体来说，cap表示一个容量（通常是作为分配或缓冲区大小的参考）。使用(cap + 1).next_power_of_two()表达式可以找到一个比cap大的最小2的幂值。
代码中的处理过程如下：
1. 将cap的值加1，保证了当cap本身就是2的幂时，不会返回原来的值。
2. 然后使用.next_power_of_two()方法计算出最小的2的幂值，即大于等于cap + 1的最小2的幂。
例如，假设cap的值为5：
首先，将5加1得到6。
接着，找到大于等于6的最小的2的幂值，即8。
所以，经过这段代码后，one_lap的值将为8，它是大于5的最小的2的幂。这样的计算通常用于优化内存分配或缓冲区的大小，以提供更高效的操作和资源利用。

### push

```rust
pub fn push(&self, value: T) -> Result<(), T>
--> fn push_or_else<F>(&self, mut value: T, f: F) -> Result<(), T>

    pub fn push(&self, value: T) -> Result<(), T> {
        self.push_or_else(value, |v, tail, _, _| {
            let head = self.head.load(Ordering::Relaxed);

            // If the head lags one lap behind the tail as well...
            if head.wrapping_add(self.one_lap) == tail {
                // ...then the queue is full.
                Err(v)
            } else {
                Ok(v)
            }
        })
    }

//
pub fn force_push(&self, value: T) -> Option<T>
--> fn push_or_else<F>(&self, mut value: T, f: F) -> Result<(), T>
```

#### 核心函数详细分析：

由`pub fn push(&self, value: T) -> Result<(), T>`直接调用push_or_else函数。核心参数就是针对Array_queue进队value参数。
主要思路：
1. 建立Backoff::new();和获取当前队列的tail索引标记位。
2. 进入主循环：
   1. 获取tail的index和圈数，圈数是为了区分head和tail，因为当前队列类似操作ringbuffer。（tail只是标记位，真正获取slot，通过index获取）
   2. 获取new_tail,如果当前index+1小于self.cap，说明还在一个圈数内部，直接tail+1为new_tail;
   3. 如果index+1 > self.cap,说明当前tail的index大于缓冲区长度，此时new_tail就是lap.wrapping_add(self.one_lap),类似索引位和标记位区别。就是超过了one_lap.
   4. 获取index指向的slot和stamp
      * tail == stamp：If the tail and the stamp match, we may attempt to push
      * stamp.wrapping_add(self.one_lap) == tail + 1 ： 如果stamp和tail+1相差self.one_lap，表示槽位slot的stamp已经循环了初始位置，而tail+1表示tail已经移动了一圈
      * 其他情况：如果以上两个条件都不满足，则需要等待stamp更新，进行退避等待

```rust
    fn push_or_else<F>(&self, mut value: T, f: F) -> Result<(), T>
    where
        F: Fn(T, usize, usize, &Slot<T>) -> Result<T, T>,
    {
        let backoff = Backoff::new();
        let mut tail = self.tail.load(Ordering::Relaxed);

        loop {
            // Deconstruct the tail.
            let index = tail & (self.one_lap - 1);
            let lap = tail & !(self.one_lap - 1);

            let new_tail = if index + 1 < self.cap {
                // Same lap, incremented index.
                // Set to `{ lap: lap, index: index + 1 }`.
                tail + 1
            } else {
                // One lap forward, index wraps around to zero.
                // Set to `{ lap: lap.wrapping_add(1), index: 0 }`.
                lap.wrapping_add(self.one_lap)
            };

            // Inspect the corresponding slot.
            debug_assert!(index < self.buffer.len());
            let slot = unsafe { self.buffer.get_unchecked(index) };
            let stamp = slot.stamp.load(Ordering::Acquire);

            // If the tail and the stamp match, we may attempt to push.
            if tail == stamp {
                // Try moving the tail.
                match self.tail.compare_exchange_weak(
                    tail,
                    new_tail,
                    Ordering::SeqCst,
                    Ordering::Relaxed,
                ) {
                    Ok(_) => {
                        // Write the value into the slot and update the stamp.
                        unsafe {
                            slot.value.get().write(MaybeUninit::new(value));
                        }
                        slot.stamp.store(tail + 1, Ordering::Release);
                        return Ok(());
                    }
                    Err(t) => {
                        tail = t;
                        backoff.spin(); // 进行自旋等待
                    }
                }
            } // 如果stamp和tail+1相差self.one_lap，表示槽位slot的stamp已经循环了初始位置，而tail+1表示tail已经移动了一圈
            else if stamp.wrapping_add(self.one_lap) == tail + 1 {
                // 全局的内存屏障，确保在将value写入slot之前，所有访问顺序的一致
                atomic::fence(Ordering::SeqCst); // 通过内存屏障，能够保证数据的正确性并避免竞态条件的发生
                // 调用指定的回调函数f对value、tail、new_tail和slot进行处理，获取新的value。
                value = f(value, tail, new_tail, slot)?;
                backoff.spin();
                tail = self.tail.load(Ordering::Relaxed); //进行自旋等待并更新tail的值。
                // 在内存屏障之后，还调用了一个回调函数f，该函数通过处理value、tail、new_tail和slot等参数，可以对数据进行进一步处理。
                // 这个回调函数的作用是确保在推送数据之前，对数据进行一些额外的处理操作，以满足特定的需求。
            } else {
                // Snooze because we need to wait for the stamp to get updated.
                backoff.snooze(); // 如果以上两个条件都不满足，则需要等待stamp更新，进行退避等待
                tail = self.tail.load(Ordering::Relaxed);
            }
        }
    }

```


### pop
主要流程：
1. 创建backoff = Backoff::new(); 获取head标记位
2. 进入主循环，直到成功弹出一个元素或者获取到一个非空的通道。
   1. 首先从head变量中提取出索引和lap两个值。
   2. 使用索引值从缓冲区中获取到对应的槽位slot。
   3. 检查槽位中存储的stamp值，如果该值比head大1，则说明可以尝试弹出一个元素。
   4. 如果当前索引加1小于缓冲区长度，则新的head值为head加1，否则新的head值为lap加一个循环（也就是队列实际长度）的长度。
   5. 使用compare_exchange_weak原子操作尝试更新head的值为新的head值，如果成功，则说明成功移动了head指针。
   6. 在更新head指针成功后，从槽位中读取值，并更新stamp的值为head加上一个循环的长度。然后返回该值作为pop函数的结果。
   7. 如果更新head指针失败，则说明其他线程已经修改了head的值，需要重新获取新的head值，并进行退避策略。
   8. 如果stamp与head相等，则说明通道为空，返回None表示无法弹出元素。
   9. 如果stamp与head不相等，说明需要等待stamp值更新，进行退避策略，然后重新获取新的head值。
重复以上步骤进行下一轮循环，直到成功弹出元素或者获取到非空通道。
代码实现如下：
```rust
   pub fn pop(&self) -> Option<T> {
        let backoff = Backoff::new();
        let mut head = self.head.load(Ordering::Relaxed);

        loop {
            // Deconstruct the head.
            let index = head & (self.one_lap - 1);
            let lap = head & !(self.one_lap - 1);

            // Inspect the corresponding slot.
            debug_assert!(index < self.buffer.len());
            let slot = unsafe { self.buffer.get_unchecked(index) };
            let stamp = slot.stamp.load(Ordering::Acquire);

            // If the the stamp is ahead of the head by 1, we may attempt to pop.
            if head + 1 == stamp {
                let new = if index + 1 < self.cap { // 如果当前索引+1小于缓冲区长度，
                    // Same lap, incremented index.
                    // Set to `{ lap: lap, index: index + 1 }`.
                    head + 1
                } else {
                    // One lap forward, index wraps around to zero.
                    // Set to `{ lap: lap.wrapping_add(1), index: 0 }`.
                    lap.wrapping_add(self.one_lap) // head值为lap加一个循环的长度。
                };

                // Try moving the head.
                match self.head.compare_exchange_weak(
                    head,
                    new,
                    Ordering::SeqCst,
                    Ordering::Relaxed,
                ) {
                    Ok(_) => {
                        // Read the value from the slot and update the stamp.
                        let msg = unsafe { slot.value.get().read().assume_init() };
                        slot.stamp
                            .store(head.wrapping_add(self.one_lap), Ordering::Release);
                        return Some(msg);
                    }
                    Err(h) => {
                        // 如果更新失败说明其他线程已经修改了head的值，需要重新获取新的head值，并进行退避策略
                        head = h;
                        backoff.spin();
                    }
                }
            } else if stamp == head { // stamp值与head不相等，则说明通道为空，返回None表示无法弹出元素
                atomic::fence(Ordering::SeqCst);
                let tail = self.tail.load(Ordering::Relaxed);

                // If the tail equals the head, that means the channel is empty.
                if tail == head {
                    return None;
                }

                backoff.spin();
                head = self.head.load(Ordering::Relaxed);
            } else { // 如果stamp与head不相等，说明需要等待stamp值更新，进行退避策略，然后重新获取新的head值。
                // Snooze because we need to wait for the stamp to get updated.
                backoff.snooze();
                head = self.head.load(Ordering::Relaxed);
            }
        }
    }
```

# seg_queue

该部分主要是用链表实现队列。
实现了一个基于块的无锁队列，通过头指针和块指针来实现pop操作的原子性和线程安全性。pop操作会从队列头部取出一个元素，并将头指针移动到下一个位置。如果队列为空，pop操作会返回None。

## 主要数据结构

### slot
```rust
struct Slot<T> {
    /// The value.
    value: UnsafeCell<MaybeUninit<T>>,

    /// The state of the slot.
    state: AtomicUsize,
}
```

### Block
```rust
struct Block<T> {
    /// The next block in the linked list.
    next: AtomicPtr<Block<T>>,

    /// Slots for values.
    slots: [Slot<T>; BLOCK_CAP],
}

```
### Position
```rust
/// A position in a queue.
struct Position<T> {
    /// The index in the queue.
    index: AtomicUsize,

    /// The block in the linked list.
    block: AtomicPtr<Block<T>>,
}
```

### SegQueue
```rust
pub struct SegQueue<T> {
    /// The head of the queue.
    head: CachePadded<Position<T>>,

    /// The tail of the queue.
    tail: CachePadded<Position<T>>,

    /// Indicates that dropping a `SegQueue<T>` may drop values of type `T`.
    _marker: PhantomData<T>,
}
```

## 编程技巧

```rust
let offset = (tail >> SHIFT) % LAP;
```
用于确定下一个元素将被写入到缓冲区中的哪个位置。
具体而言，这个表达式分为两个步骤：
* tail >> SHIFT：通过右移操作符>>，将tail的值向右移动SHIFT位。tail表示队尾的索引值，而SHIFT是一个常数，用于确定每个索引需要存储在缓冲区中的位数。通过右移操作符，可以将tail转换为缓冲区的索引范围内。
* % LAP：计算上一步结果与常数LAP之间的模运算。LAP是一个在队列实现中使用的掩码，它确定了缓冲区的大小。通过计算模运算，可以将上一步得到的索引值归入合适范围内，确保它不会超过缓冲区的边界。
综合而言，通过这个计算，offset可以得到下一个元素在缓冲区中的写入位置，从而实现了循环队列的功能。这是因为缓冲区的存储是循环的，当索引超过缓冲区边界时，会回到缓冲区的起始位置，形成循环。

```rust
            // Try advancing the tail forward.
            match self.tail.index.compare_exchange_weak(
                tail,
                new_tail,
                Ordering::SeqCst,
                Ordering::Acquire,
            ) {
                Ok(_) => unsafe {
                    // If we've reached the end of the block, install the next one.
                    if offset + 1 == BLOCK_CAP {
                        let next_block = Box::into_raw(next_block.unwrap());
                        let next_index = new_tail.wrapping_add(1 << SHIFT);

                        self.tail.block.store(next_block, Ordering::Release);
                        self.tail.index.store(next_index, Ordering::Release);
                        (*block).next.store(next_block, Ordering::Release);
                    }

                    // Write the value into the slot.
                    let slot = (*block).slots.get_unchecked(offset);
                    slot.value.get().write(MaybeUninit::new(value));
                    slot.state.fetch_or(WRITE, Ordering::Release);

                    return;
                },
                Err(t) => {
                    tail = t;
                    block = self.tail.block.load(Ordering::Acquire);
                    backoff.spin();
                }
            }
```
这段代码段展示了对环形缓冲区中的tail指针进行推进（advance）的过程。它使用了原子操作和无序的内存访问来确保的正确性和性能。
首先，代码使用compare_exchange_weak函数尝试原子地更新tail.index的值。如果当前的tail.index的值与传入的tail相等，那么将其更新为new_tail的值。这个操作使用SeqCst（顺序一致性）和Acquire（获取语义）内存序，以确保正确的同步和顺序性。如果更新成功，接下来的代码将会执行。
在成功更新tail.index之后，代码首先检查是否达到了环形缓冲区的末尾位置。如果offset + 1 == BLOCK_CAP为真，说明已经到达了当前块的末尾，需要安装下一个块。
接下来，代码通过使用Box::into_raw将next_block转换为原始指针，并使用new_tail.wrapping_add(1 << SHIFT)计算下一个块的索引位置。然后，将next_block和next_index分别存储到self.tail.block和self.tail.index中，并使用Release内存序进行存储，以确保对这些存储的修改对其他线程可见。
接着，代码将当前块的next字段设置为next_block，同样使用Release内存序。
然后，代码将value写入到当前偏移位置的插槽（slot）中。它使用get_unchecked函数来获取插槽的引用，但是因为此处使用了unsafe代码块，需要确保在使用之前已正确验证索引的有效性。之后，代码使用write方法将value的值写入到插槽中，并使用Release内存序设置插槽的状态为WRITE。
最后，代码通过return语句退出函数。
如果compare_exchange_weak操作失败（即Err(t)），说明其他线程已经修改了tail.index的值，那么代码将更新tail和block的值，并通过自旋等待来重新尝试操作，这里使用了backoff变量的spin方法来进行自旋等待。
总体而言，这段代码实现了一种并发安全且高效的环形缓冲区的推进操作，它利用原子操作和无序内存访问来确保多线程环境下对tail指针的正确管理。

## pop

```rust
            // Try moving the head index forward.
            match self.head.index.compare_exchange_weak(
                head,
                new_head,
                Ordering::SeqCst,
                Ordering::Acquire,
            ) {
                Ok(_) => unsafe {
                    // If we've reached the end of the block, move to the next one.
                    if offset + 1 == BLOCK_CAP {
                        let next = (*block).wait_next();
                        let mut next_index = (new_head & !HAS_NEXT).wrapping_add(1 << SHIFT);
                        if !(*next).next.load(Ordering::Relaxed).is_null() {
                            next_index |= HAS_NEXT;
                        }

                        self.head.block.store(next, Ordering::Release);
                        self.head.index.store(next_index, Ordering::Release);
                    }

                    // Read the value.
                    let slot = (*block).slots.get_unchecked(offset);
                    slot.wait_write();
                    let value = slot.value.get().read().assume_init();

                    // Destroy the block if we've reached the end, or if another thread wanted to
                    // destroy but couldn't because we were busy reading from the slot.
                    if offset + 1 == BLOCK_CAP {
                        Block::destroy(block, 0);
                    } else if slot.state.fetch_or(READ, Ordering::AcqRel) & DESTROY != 0 {
                        Block::destroy(block, offset + 1);
                    }

                    return Some(value);
                },
                Err(h) => {
                    head = h;
                    block = self.head.block.load(Ordering::Acquire);
                    backoff.spin();
                }
            }

```
首先，代码使用compare_exchange_weak方法比较并交换self.head.index的值。如果成功，接下来进行一些操作，否则，代码会根据返回的新值进行后续处理。这个部分通常是一个自旋锁来等待其他线程释放资源。

* 在成功进行比较并交换后，代码检查偏移量offset是否为BLOCK_CAP - 1。
  * 如果是，说明已经到达了当前块的末尾。在这种情况下，它会获取下一个块的引用，并计算下一个块的索引next_index。如果下一个块的next指针不为空，则设置HAS_NEXT位。然后，通过self.head.block.store方法将下一个块的引用存储到self.head.block中，通过self.head.index.store方法将计算得到的next_index存储到self.head.index中。
  * 接下来，代码通过指针操作访问了块中的槽位，并获取了槽位中的值。然后，根据当前的偏移量判断是否需要销毁当前块。如果是块的末尾，调用Block::destroy方法销毁块，否则，如果其他线程已经标记要销毁块，调用Block::destroy方法销毁块，并指定销毁的偏移量。
最后，代码返回从槽位中读取到的值。

问题1：销毁的过程？


