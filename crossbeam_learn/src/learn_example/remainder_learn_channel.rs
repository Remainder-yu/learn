use crossbeam::channel::{self, unbounded};
use std::{thread, time};

pub fn test_mod_channel() {
    // 创建一个无限容量的通道
    let (sender, receiver) = channel::unbounded();
    // 创建一个线程来发送数据
    let sender_thread = thread::spawn(move || {
        for i in 0..5 {
            // 发送数据到通道
            sender.send(i).unwrap();
            thread::sleep(std::time::Duration::from_secs(1));
        }
    });
    // 创建一个线程来接收数据
    let receiver_thread = thread::spawn(move || {
        for _ in 0..5 {
            // 从通道接收数据
            let received_data = receiver.recv().unwrap();
            println!("test mod channel Received: {}", received_data);
        }
    });
    // 等待发送者和接收者线程的完成
    sender_thread.join().unwrap();
    receiver_thread.join().unwrap();
}

// spsc：建立一个生产者和一个消费者
// 使用crossbeam::scope和Scope::spwan创建一个ex-crossbeam-swawn的实例来管理生产者线程

pub fn test_scope_main(){
    let (snd,rcv) = unbounded();
    let n_msgs = 5;
    crossbeam::scope(|s| {
        s.spawn(|_| {
            for i in 0..n_msgs {
                snd.send(i).unwrap();
                thread::sleep(time::Duration::from_millis(100));
            }
        });
    }).unwrap();
    for _ in 0..n_msgs {
        let msg = rcv.recv().unwrap();
        println!("scope Received {}", msg);
    }

}