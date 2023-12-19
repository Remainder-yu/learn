# tokio源码阅读分析+调试

分析整体目录结构，以及每个子目录主要作用，分析子目录的数据结构，理清楚各个子目录的数据结构之间的关系。

## runtime
其核心就是runtime，各目录分析：
1. blocking目录：
作用：提供了tokio::task::blocking函数，允许用户在异步上下文中执行阻塞操作，以避免阻塞全部的异步任务。
主要原理：通过将阻塞操作委托给独立线程池中的线程来避免阻塞整个异步任务的执行。该独立线程池是通过tokio-threadpool库实现的。
2. context目录：
作用： 定义了context结构体，表示异步任务的上下文环境。
主要原理：context结构体包含了异步任务执行所需的环境信息，例如调度器任务执行的调度器、定时器等，以及提供了一些API用于与底层运行时进行交互和管理。
3. IO目录：
作用：提供与IO 操作相关的功能与接口，例如文件操作、套接字操作等。
主要作用：基于底层操作系统接口或基础库，实现了异步IO操作，使得可以在异步上下文中进行高效的I/O操作。
4. metrics目录
作用：提供质量和监控相关功能，用于收集和展示与tokio执行引擎相关的度量指标
主要原理：通过使用metrics库或其他度量库，对tokio运行时的性能和行为进行度量和统计，以便于用户了解和监控tokio的性能指标。
5. scheduler目录：
作用：管理和调度异步任务的执行。
主要原理：通过任务调度器的管理机制，选择下一个要执行的任务，以提高并发性能。调度器可以基于不同的策略和算法，如工作窃取、优先级队列等，来决定任务的执行顺序。
6. signal目录：
作用：提供处理信号（signal）的功能，例如终止信号、中断信号等。
主要原理：通过底层的操作系统接口或其他信号处理的库，实现对信号的处理和监听，以便于在异步任务中进行相应的操作。
7. task目录：
作用：定义了Task结构体，表示tokio中的异步任务。
主要原理：Task结构体包含了异步任务的执行逻辑和状态信息，通过调度器调度执行，异步任务可以在异步上下文中被调度和运行。
8. time目录：
作用：提供与时间相关的功能，例如定时器和时间操作。
主要原理：通过使用底层的系统调用或其他时间库，实现了与时间和定时操作相关的功能，例如延迟执行任务、周期性执行任务等。
总的来说，这些runtime目录下的子目录提供了tokio运行时的核心功能和模块，包括异步任务调度、I/O操作、时间管理、信号处理等。这些子目录的组合和协作，构成了tokio运行时的基础设施，使得异步编程更加高效和方便。

### blocking

image.png

```rust
pub(crate) fn spawn_blocking<F, R>(func: F) -> JoinHandle<R>
    --> rt.spawn_blocking(func)
        --> self.inner.blocking_spawner().spawn_blocking(self, func)
            --> pub(crate) fn spawn_blocking<F, R>(&self, rt: &Handle, func: F) -> JoinHandle<R>
                --> self.spawn_blocking_inner(func, Mandatory::NonMandatory, None, rt)
                    --> let spawned = self.spawn_task(Task::new(task, is_mandatory), rt); // 将task添加到tokio运行时的任务调度器中

重点分析：spawn_task。

```

重要结构体分析：
在tokio中，BlockingPool结构体主要用于管理和调度阻塞型操作的执行。它包含以下主要成员变量：

* spawner: Spawner：一个Spawner实例，用于生成并管理执行阻塞型操作的任务执行句柄。它负责将阻塞型操作转换为异步任务，并将其提交给tokio的任务调度器执行。
* shutdown_rx: shutdown::Receiver：一个用于接收关闭信号的接收器。可以通过该接收器来监听是否需要关闭或终止阻塞池。
BlockingPool结构体的主要作用是提供一个任务调度器，用于将阻塞型操作转化为异步操作，避免在其中的阻塞型操作阻塞整个事件循环。它可以有效地将阻塞型操作与异步非阻塞操作进行协调，提高整体的并发性能。

```rust
fn spawn_task(&self, task: Task, rt: &Handle) -> Result<(), SpawnError> :pool.rs
    --> match self.spawn_thread(shutdown_tx, rt, id) ://满足创建线程的条件进行创建，不满足则会执行唤醒当前空闲线程
            // 同时在这里之前实现了rt：handle和task绑定创建，spawn_thread方法用于创建一个新的工作线程，并在该线程中执行任务。
        --> let mut builder = thread::Builder::new().name((self.inner.thread_name)());
        --> builder.spawn(move || {
             // Only the reference should be moved into the closure
             let _enter = rt.enter();
             rt.inner.blocking_spawner().inner.run(id); // 调用impl Inner { fn run(&self, worker_thread_id: usize)
             drop(shutdown_tx);
           })
           --> rt.inner.blocking_spawner().inner.run(id); // 执行run
```

### BlockingTask实现future

pub(crate) fn spawn_blocking_inner<F, R>函数中，调用let fut = BlockingTask::new(func); 将func转换成了future，为什么这里可以转换成future？

```rust
impl<T> BlockingTask<T> {
    /// Initializes a new blocking task from the given function.
    pub(crate) fn new(func: T) -> BlockingTask<T> {
        BlockingTask { func: Some(func) }
    }
}

以上返回的就是BlockingTask<T>，而同时又对BlockingTask<T>实现了future trait：
impl<T, R> Future for BlockingTask<T>
where
    T: FnOnce() -> R + Send + 'static,
    R: Send + 'static,
{
    type Output = R;

    fn poll(mut self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<R> {
        let me = &mut *self; //将self解引用为可变引用，并赋给me变量。
        let func = me
            .func
            .take()
            .expect("[internal exception] blocking task ran twice.");
            // 从me变量中获取闭包，并将其从BlockingTask实例中拿出。
            // 这里使用take()方法，调用一次后将闭包从BlockingTask中移除，避免闭包被多次调用。

        // This is a little subtle:
        // For convenience, we'd like _every_ call tokio ever makes to Task::poll() to be budgeted
        // using coop. However, the way things are currently modeled, even running a blocking task
        // currently goes through Task::poll(), and so is subject to budgeting. That isn't really
        // what we want; a blocking task may itself want to run tasks (it might be a Worker!), so
        // we want it to start without any budgeting.
        crate::runtime::coop::stop();
        // 调用coop::stop()函数，停止tokio对协作任务的预算。
        // 这样做是为了让阻塞任务可以在没有预算的情况下执行，以避免阻塞任务本身可能需要运行其他任务的情况。

        Poll::Ready(func()) // 调用闭包func并将其返回值作为Poll::Ready的结果，表示阻塞任务已完成并产生了指定的返回值。
    }
}

然后这里就是从阻塞的任务不断拿，然后查看时候Ready，并执行。是如何不断poll的？

```
分析上面代码：
在poll函数中，首先获取闭包的引用，然后将闭包实例取出。取出闭包后，调用crate::runtime::coop::stop()方法来停止tokio的协作任务预算，以避免对该闭包进行预算。最后，使用闭包调用func()来执行具体的任务，并将任务的返回值作为Poll::Ready的结果返回。


```rust
fn spawn_task(&self, task: Task, rt: &Handle) -> Result<(), SpawnError> :pool.rs
    --> match self.spawn_thread(shutdown_tx, rt, id) ://满足创建线程的条件进行创建，不满足则会执行唤醒当前空闲线程
            // 同时在这里之前实现了rt：handle和task绑定创建，spawn_thread方法用于创建一个新的工作线程，并在该线程中执行任务。
        --> let mut builder = thread::Builder::new().name((self.inner.thread_name)());
        --> builder.spawn(move || {
             // Only the reference should be moved into the closure
             let _enter = rt.enter();
             rt.inner.blocking_spawner().inner.run(id); // 调用impl Inner { fn run(&self, worker_thread_id: usize)
             drop(shutdown_tx);
           })
           --> rt.inner.blocking_spawner().inner.run(id); // 执行run
```

spawn_thread方法用于创建一个新的工作线程，并在该线程中执行任务。它首先创建一个thread::Builder对象，并设置线程的名称和栈大小（如果有指定）。然后，它克隆当前的运行时句柄，并使用它创建一个闭包，该闭包会在新线程中执行。闭包中的代码会进入运行时并执行任务队列中的任务，然后在任务执行完成后关闭线程并发送关闭通知。

其实builder.spawn就是调用了tokio/src/runtime/handle.rs中的`pub fn spawn<F>(&self, future: F) -> JoinHandle<F::Output>`函数，然后根据类型调用：
```rust
tokio/src/runtime/scheduler/multi_thread/handle.rs
    pub(crate) fn spawn<F>(me: &Arc<Self>, future: F, id: task::Id) -> JoinHandle<F::Output>
    where
        F: crate::future::Future + Send + 'static,
        F::Output: Send + 'static,
    {
        Self::bind_new_task(me, future, id)
    }
```

```rust
    pub(super) fn bind_new_task<T>(me: &Arc<Self>, future: T, id: task::Id) -> JoinHandle<T::Output>
    where
        T: Future + Send + 'static,
        T::Output: Send + 'static,
    {
        let (handle, notified) = me.shared.owned.bind(future, me.clone(), id);

        me.schedule_option_task_without_yield(notified);

        handle
    }

    pub(super) fn schedule_option_task_without_yield(&self, task: Option<Notified>) {
        if let Some(task) = task {
            self.schedule_task(task, false);
        }
    }

        pub(super) fn schedule_task(&self, task: Notified, is_yield: bool) {
        with_current(|maybe_cx| {
            if let Some(cx) = maybe_cx {
                // Make sure the task is part of the **current** scheduler.
                if self.ptr_eq(&cx.worker.handle) {
                    // And the current thread still holds a core
                    if let Some(core) = cx.core.borrow_mut().as_mut() {
                        self.schedule_local(core, task, is_yield);
                        return;
                    }
                }
            }

            // Otherwise, use the inject queue.
            self.push_remote_task(task);
            self.notify_parked_remote();
        });
    }

// tokio/src/runtime/scheduler/multi_thread/worker.rs
    fn schedule_local(&self, core: &mut Core, task: Notified, is_yield: bool) {
        core.stats.inc_local_schedule_count();

        // Spawning from the worker thread. If scheduling a "yield" then the
        // task must always be pushed to the back of the queue, enabling other
        // tasks to be executed. If **not** a yield, then there is more
        // flexibility and the task may go to the front of the queue.
        let should_notify = if is_yield || !core.lifo_enabled {
            core.run_queue
                .push_back_or_overflow(task, self, &mut core.stats);
            true
        } else {
            // Push to the LIFO slot
            let prev = core.lifo_slot.take();
            let ret = prev.is_some();

            if let Some(prev) = prev {
                core.run_queue
                    .push_back_or_overflow(prev, self, &mut core.stats);
            }

            core.lifo_slot = Some(task);

            ret
        };

        // Only notify if not currently parked. If `park` is `None`, then the
        // scheduling is from a resource driver. As notifications often come in
        // batches, the notification is delayed until the park is complete.
        if should_notify && core.park.is_some() {
            self.notify_parked_local();
        }
    }
```