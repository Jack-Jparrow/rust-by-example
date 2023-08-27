use std::thread;

/**
 * @Author: 白银
 * @Date: 2022-09-02 14:44:14
 * @LastEditTime: 2022-09-02 15:12:30
 * @LastEditors: 白银
 * @Description: Rust 通过 spawn 函数提供了创建本地操作系统（native OS）线程的机制 https://rustwiki.org/zh-CN/rust-by-example/std_misc/threads.html
 */

static NTHREADS: i32 = 10;

// 这是主（`main`）线程
fn main() {
    // 提供一个 vector 来存放所创建的子线程（children）
    let mut children = vec![];

    for i in 0..NTHREADS {
        // 启动（spin up）另一个线程
        children.push(thread::spawn(move || {
            println!("this is thread number {}", i)
        }))
    }

    for child in children {
        // 等待线程结束。返回一个结果
        let _ = child.join();
    }
}
