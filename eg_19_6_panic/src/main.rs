/**
 * @Author: 白银
 * @Date: 2022-09-02 13:54:40
 * @LastEditTime: 2022-09-02 14:07:31
 * @LastEditors: 白银
 * @Description: panic! 宏可用于产生一个 panic （恐慌），并开始回退（unwind）它的栈 https://rustwiki.org/zh-CN/rust-by-example/std/panic.html
 */

// 整型除法（/）的重新实现
fn division(dividend: i32, divisor: i32) -> i32 {
    if divisor == 0 {
        // 除以 0 会引发 panic
        panic!("division by zero");
    } else {
        dividend / divisor
    }
}

// `main` 任务
fn main() {
    // 堆分配的整数
    let _x = Box::new(0i32);

    // 此操作将会引发一个任务失败
    division(3, 0);

    println!("This point won't be reached!");

    // `_x` 应当会在此处被销毁
}
