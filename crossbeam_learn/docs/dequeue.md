# dequeue
分析该模块主要理解dequeue的作用：
dequeue是一个双端队列，主要用于并发编程的工作窃取算法。它的作用是提供一个高效的并发数据结构，用于任务调度和负载均衡。
Dequeue的主要特点是具有双端操作的能力，即可以在队列的头部和尾部进行元素的插入和删除操作。这使得它适用于工作窃取算法，其中线程可以从队列的尾部获取任务并在队列的头部添加任务。
工作窃取算法是一种常用于并行计算中的调度算法，它通过在任务队列头部添加任务，同时允许其他线程从队列尾部窃取任务来实现负载均衡。这样，具有较少负载的线程可以从具有较多负载的线程那里窃取任务，以实现任务的均衡分配和并行执行。
Crossbeam中的dequeue组件提供了一个循环双端队列（CvDeque）的实现，以支持工作窃取算法。它使用了分离式版本的双端队列，其中每个线程都有自己的私有存储来存储任务，并且只有在窃取任务时才需要与其他线程进行同步。
通过使用dequeue组件，开发者可以方便地实现工作窃取算法，并利用多线程的优势实现高效的任务调度和负载均衡，从而实现更好的并发性能。‘

## 主要组件及关系
在Crossbeam库中，Injector、Stealer和Worker是并发数据结构中的三个重要组件，用于实现多线程间的协作和数据共享。

* Injector（注入器）：Injector是一个用于向队列中注入数据的组件。它提供了一种向队列中发送数据的方法，即inject，可以用于通知其他线程有新的数据要处理。多个线程可以共享同一个Injector实例，从而实现并发处理数据的能力。最终通过Injector将task注入导致指定的Woker中去。
* Stealer（偷取器）：Stealer是一个用于从队列中偷取数据的组件。它提供了一种从队列中偷取数据的方法，即steal，用于获取其他线程尚未处理的数据。Stealer可以通过Injector获取共享数据，并且保留与其他线程相对独立的状态，以便并行处理数据。
* Worker（工作者）：Worker是一个用于处理任务的线程组件。一个Worker可以通过Injector接收新的任务，并使用Stealer从队列中偷取其他线程尚未处理的任务。通过这种方式，多个Worker可以并行处理任务并共享数据。
三者的关系是：一个或多个Worker可以通过共享同一个Injector来接收新任务，并根据需要使用独立的Stealer来偷取队列中的任务。这种设计使得多个线程可以在并发环境中高效地处理任务，并充分利用共享数据。
总结起来，Injector用于注入新的任务，Stealer用于偷取其他线程尚未处理的任务，而Worker则是具体的工作者线程，负责处理任务。它们合作组成了一个高效的并发任务处理系统。

## dequeue主要数据结构
Buffer数据结构主要作用是：
```rust
struct Buffer<T> {
    /// Pointer to the allocated memory.
    ptr: *mut T,

    /// Capacity of the buffer. Always a power of two.
    cap: usize,
}
```

在Rust中，copy_nonoverlapping函数是一个指针工具，用于将一段内存块从源地址复制到目标地址，且源地址和目标地址不会有重叠。它的原型如下：
```rust
pub fn copy_nonoverlapping<T>(src: *const T, dst: *mut T, count: usize)
where
    T: Copy;
```

该函数接受三个参数：
src：源地址的指针，指向需要复制的内存块。src是一个`*const T`类型的指针，其中T是复制类型的数据类型。
dst：目标地址的指针，指向将要复制到的内存块。dst是一个`*mut T`类型的指针，其中T是复制类型的数据类型。
count：要复制的元素数量。
需要注意的是，被复制的类型T必须实现了Copy trait，以确保可以按位复制。如果T不是Copy类型，编译器将会报错。
总结起来，copy_nonoverlapping函数用于按位复制源地址的内存块到目标地址，不会出现地址重叠。

在Crossbeam库中，`let guard = &epoch::pin();`用于创建一个Guard实例，它在跨线程共享数据的情况下提供了内存安全的访问方式。
Guard是Crossbeam库中的核心概念之一，它是基于“Epoch-Based Reclamation”（EBR）机制的一种保护方式。EBR是一种内存回收机制，用于解决多线程并发环境下共享数据的内存安全问题。
通过创建Guard实例，可以确保在该实例的作用域内，可以安全地对跨线程共享的数据进行访问，而不会遭遇内存安全问题，如数据竞争或使用已释放的内存。
在`let guard = &epoch::pin();`语句中，&epoch::pin()创建了一个Guard实例，并将它绑定到变量guard上。该实例在作用域中被持有，保证了在该作用域中对跨线程共享数据的访问是安全的。
一旦Guard实例超出作用域，Crossbeam库会根据EBR机制自动处理内存回收，以避免悬挂指针的问题。
所以，`let guard = &epoch::pin();`语句的作用是创建一个Guard实例并将其用于安全地访问跨线程共享的数据。

## worker

### 分析worker对象的resize函数方法：

```rust
    unsafe fn resize(&self, new_cap: usize) {
        // Load the back index, front index, and buffer.
        let b = self.inner.back.load(Ordering::Relaxed);
        let f = self.inner.front.load(Ordering::Relaxed);
        let buffer = self.buffer.get();
        // 针对self.inner

        // Allocate a new buffer and copy data from the old buffer to the new one.
        let new = Buffer::alloc(new_cap);
        let mut i = f;
        while i != b {
        // 依次拷贝之前工作队列中的buffer元素到新分配的buffer中
            unsafe { ptr::copy_nonoverlapping(buffer.at(i), new.at(i), 1) }
            i = i.wrapping_add(1);
        }

        let guard = &epoch::pin();

        // Replace the old buffer with the new one.
        // buffer是原子类型，对其swap操作，利用guard
        // 替换当前worker的buffer成员，修改成员方法，针对buffer是Cell包裹的一种智能指针类型，可以直接replace。
        self.buffer.replace(new);
        
        let old =
            self.inner
                .buffer
                .swap(Owned::new(new).into_shared(guard), Ordering::Release, guard);

        // Destroy the old buffer later.
        unsafe { guard.defer_unchecked(move || old.into_owned().into_box().dealloc()) }

        // If the buffer is very large, then flush the thread-local garbage in order to deallocate
        // it as soon as possible.
        if mem::size_of::<T>() * new_cap >= FLUSH_THRESHOLD_BYTES {
            guard.flush();
        }
    }
```
这里buffer是inner的成员：`buffer: CachePadded<Atomic<Buffer<T>>>,`可以看出buffer是Atomic包一层，所以swap函数是Atomic方法。而Atomic是crossbeam自己实现的，所以在crossbeam-epoch/src/atomic.rs文件中，该swap中的guard能保证线程共享，且无内存泄露。然后再释放old内存释放。
```rust
        let old =
            self.inner
                .buffer
                .swap(Owned::new(new).into_shared(guard), Ordering::Release, guard);`

crossbeam-epoch/src/atomic.rs    
swap原型：     
    pub fn swap<'g, P: Pointer<T>>(&self, new: P, order: Ordering, _: &'g Guard) -> Shared<'g, T> {
        unsafe { Shared::from_ptr(self.data.swap(new.into_ptr(), order)) }
    }
   
```
### 分析worker对象的reserve函数方法：
同时，fn reserve(&self, reserve_cap: usize)函数是为了用于预留足够的容量，以便在不需要扩展缓冲区的情况下可以推入reserve_cap个任务。首先获取了当前缓冲区的长度len，以及缓冲区的容量cap。然后判断如果cap - len小于reserve_cap，即缓冲区可用容量不足以容纳reserve_cap个任务，需要进行扩容操作。
扩容操作是，循环将容量new_cap翻倍，直到new_cap - len大于等于reserve_cap。这样可以确保新容量足够大以容纳所需的reserve_cap个任务。
最后，通过调用self.resize(new_cap)方法进行缓冲区的重分配，实现了容量的扩展。
```rust
    fn reserve(&self, reserve_cap: usize) {
        if reserve_cap > 0 {
            // Compute the current length.
            let b = self.inner.back.load(Ordering::Relaxed);
            let f = self.inner.front.load(Ordering::SeqCst);
            let len = b.wrapping_sub(f) as usize;

            // The current capacity.
            let cap = self.buffer.get().cap;

            // Is there enough capacity to push `reserve_cap` tasks?
            if cap - len < reserve_cap {
                // Keep doubling the capacity as much as is needed.
                let mut new_cap = cap * 2;
                while new_cap - len < reserve_cap {
                    new_cap *= 2;
                }

                // Resize the buffer.
                unsafe {
                    self.resize(new_cap);
                }
            }
        }
    }
```

### worker：push/pop

```rust
/// Worker<T>结构体可能用于实现一个工作者线程的对象，用于执行特定的任务并与其他线程共享内部数据。
/// inner字段可能用于共享队列的内部表示，
/// buffer字段可能用于快速访问缓冲区数据，
/// 而flavor字段可能用于表示工作者线程所使用的队列类型。
/// _marker字段可能用于防止该结构体在多线程之间共享，即它是一个私有（non-send、non-sync）的工作者线程对象。
pub struct Worker<T> {
    /// A reference to the inner representation of the queue.
    inner: Arc<CachePadded<Inner<T>>>,

    /// A copy of `inner.buffer` for quick access.
    buffer: Cell<Buffer<T>>,

    /// The flavor of the queue.
    flavor: Flavor,

    /// Indicates that the worker cannot be shared among threads.
    _marker: PhantomData<*mut ()>, // !Send + !Sync
}
```

## Stealer

## Injector

```rust
struct Slot<T> {
    /// The task.，存放task的槽位。
    task: UnsafeCell<MaybeUninit<T>>,

    /// The state of the slot.
    state: AtomicUsize,
}

struct Block<T> {
    /// The next block in the linked list.
    next: AtomicPtr<Block<T>>,

    /// Slots for values.
    slots: [Slot<T>; BLOCK_CAP],
}`

/// A position in a queue.
struct Position<T> {
    /// The index in the queue.
    index: AtomicUsize,

    /// The block in the linked list.
    block: AtomicPtr<Block<T>>,
}

pub struct Injector<T> {
    /// The head of the queue.
    head: CachePadded<Position<T>>,

    /// The tail of the queue.
    tail: CachePadded<Position<T>>,

    /// Indicates that dropping a `Injector<T>` may drop values of type `T`.
    _marker: PhantomData<T>,
}

```
其中核心就是`fn steal_batch(&self, dest: &Worker<T>) -> Steal<()> `Steals a batch of tasks and pushes them into a worker.核心就是获取目标worker，然后注入worker中 dest_buffer.write(dest_b.wrapping_add(i as isize), task)。
```rust
    pub fn steal_batch_with_limit(&self, dest: &Worker<T>, limit: usize) -> Steal<()> {
            ········

        // Reserve capacity for the stolen batch.
        let batch_size = new_offset - offset;
        dest.reserve(batch_size);

        // Get the destination buffer and back index.
        // 获取目标worker队列信息，然后再后面匹配写入
        let dest_buffer = dest.buffer.get();
        let dest_b = dest.inner.back.load(Ordering::Relaxed);

        unsafe {
                ·······
            // Copy values from the injector into the destination queue.
            match dest.flavor {
                Flavor::Fifo => {
                    for i in 0..batch_size {
                        // Read the task.
                        let slot = (*block).slots.get_unchecked(offset + i);
                        slot.wait_write();
                        let task = slot.task.get().read();
                        // 将 dest_buffer写入需要注入的task，而task是Injector中的，拿出来的。
                        // Write it into the destination queue.
                        dest_buffer.write(dest_b.wrapping_add(i as isize), task);
                    }
                }

                Flavor::Lifo => {
                    for i in 0..batch_size {
                        // Read the task.
                        let slot = (*block).slots.get_unchecked(offset + i);
                        slot.wait_write();
                        let task = slot.task.get().read();

                        // Write it into the destination queue.
                        dest_buffer.write(dest_b.wrapping_add((batch_size - 1 - i) as isize), task);
                    }
                }
            }

    }
```
