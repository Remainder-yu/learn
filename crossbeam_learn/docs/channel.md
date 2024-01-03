# channel
分析channel是一个大头，该部分是crossbeam的核心，channel是并发编程的mpms模型的典型应用。所以分析该模块需要先分析其用途。
其中分析channel核心的就两种，Array、List。

分析主要目录文件：

这里的"flavors"指的是SenderFlavor枚举类型的不同变体。crossbeam-channel库中的SenderFlavor枚举用于表示不同种类的发送器，它们在内部实现上有所不同。具体来说，这里列举了三种不同的SenderFlavor变量：Array、List和Zero。

SenderFlavor::Array表示使用数组作为内部通道的发送器。这种发送器使用固定大小的数组来存储待发送的消息。
SenderFlavor::List表示使用链表作为内部通道的发送器。这种发送器使用链表结构来存储待发送的消息，可以动态地扩展和收缩。
SenderFlavor::Zero表示使用零大小缓冲区作为内部通道的发送器。这种发送器不会保留任何待发送的消息，只是在需要时立即发送消息。
根据代码片段中的调用，chan.send(msg, None)用于在相应的发送器中发送消息msg。具体到每种发送器的实现细节和性能特征可能会有所不同，你可以通过参考crossbeam-channel的官方文档或源码来了解更多相关信息。


![Alt text](image.png)

## counter.rs分析

![Alt text](counter.png)



# 画图工具：
https://www.processon.com/diagraming/65957d88a308bf5d33e11d47
https://handraw.top/