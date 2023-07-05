
bili学习视频，源码分享：
https://www.bilibili.com/video/BV1uT4y1R75U/?spm_id_from=333.337.search-card.all.click&vd_source=238ae623fc8447b840a509ffd36fb24e

入门秘籍
https://rust-book.junmajinlong.com/ch100/02_understand_tokio_task.html

tokio是rust中使用最广泛的runtime，它性能高、功能丰富、便于使用，是使用Rust实现高并发不可不学的一个框架。

在学习tokio之前，当然是在cargo.toml中引入tokio。
```toml
// 开启全部功能的tokio，
// 在了解tokio之后，只开启需要的特性，减少编译时间，减小编译大小
tokio = {version = "1.4", features = ["full"]}
```

# runtime
在Rust中，"runtime"是指运行时环境（runtime environment），它是一种软件框架，提供了在程序运行时所需的支持和服务。Rust是一种系统级编程语言，设计初衷是为了提供高性能和内存安全，因此它没有内建的运行时系统。与其他一些编程语言（如Java和C#）不同，Rust的运行时环境相对较小，主要由标准库提供的一些功能组成。
Rust的运行时环境主要包括：
内存分配器（Allocator）：Rust标准库提供了用于动态内存分配和管理的接口，例如Box、Vec等类型，这些类型使用的内存分配器是运行时环境的一部分。Rust默认使用的是系统分配器，但也可以自定义分配器。
线程调度（Thread scheduling）：Rust支持多线程编程，并提供了标准库中的thread模块，用于创建和管理线程。线程的调度和切换是由运行时环境负责的。
异常处理（Exception handling）：Rust使用panic和Result类型来处理异常情况。当发生panic时，运行时环境负责处理并提供相关的错误信息。
需要注意的是，Rust的运行时环境相对较小，这是为了保持语言的性能和安全性。在某些情况下，可能需要使用其他库或框架来提供更全面的运行时支持，例如Tokio用于异步编程、Rocket用于Web开发等。

创建线程时需要传入闭包，怎么处理呢？
