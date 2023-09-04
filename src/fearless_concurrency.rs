pub fn demo() {
    // spawn_demo();
    // move_demo02();
    channel_demo();
}

use std::thread;
use std::time::Duration;
fn spawn_demo() {
    // 创建一个线程 函数闭包的方式
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep((Duration::from_millis(1)));
        }
    });
    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }
    // 等待线程执行完再结束
    handle.join().unwrap();
}

// rust 无法知道线程运行多久，无法知道v的引用是否一直有效，主线程可能会调用drop(v)操作
// fn move_demo() {
//     let v = vec![1,2,3,4];
//     // ^^ may outlive borrowed value `v`
//     let handle = thread::spawn(|| {
//         println!("here's a vector:{:?}",v)
//     });
//     handle.join().unwrap();
// }

// 能运行，但是中英文档上都写着该操作不被允许...
fn move_demo02() {
    let v = vec![1, 2, 3];
    let handle = thread::spawn(move || {
        println!("here's a vector: {:?}", v);
    });
    handle.join().unwrap();
}

use std::sync::mpsc;

// 使用消息传递在线程间传递数据
// 当接收者或是发送者任一方被丢弃时，被认为channel被close，再操作就panic
fn channel_demo() {
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();
    });
    let res = rx.recv().unwrap();
    println!("got:{}", res);
}
