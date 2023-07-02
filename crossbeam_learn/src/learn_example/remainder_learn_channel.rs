use crossbeam::channel;
use std::thread;

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