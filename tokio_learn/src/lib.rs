use std::thread;
use std::time::Duration;
use tokio::runtime::Runtime;
use chrono::Local;

pub fn test_runtime_main() {
  // 在第一个线程内创建一个多线程的runtime
  let t1 = thread::spawn(||{
    let _rt = Runtime::new().unwrap();
    thread::sleep(Duration::from_secs(10));
  });

  // 在第二个线程内创建一个多线程的runtime
  let t2 = thread::spawn(||{
    let _rt = Runtime::new().unwrap();
    thread::sleep(Duration::from_secs(10));
  });

  t1.join().unwrap();
  t2.join().unwrap();
  test_runtime();
}

pub fn test_runtime() {
    let rt = Runtime::new().unwrap();
    rt.block_on(async {
        println!("before sleep {}", Local::now().format("%F %T.%3f"));
        tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
        println!("after sleep: {}", Local::now().format("%F %T.%3f"));
    });
}