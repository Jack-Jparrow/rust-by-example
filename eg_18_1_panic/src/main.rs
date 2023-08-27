/**
 * @Author: 白银
 * @Date: 2022-09-01 14:27:42
 * @LastEditTime: 2022-09-01 15:10:23
 * @LastEditors: 白银
 * @Description: panic会打印一个错误消息，开始 回退（unwind）任务，且通常会退出程序 https://rustwiki.org/zh-CN/rust-by-example/error/panic.html
 */

fn give_princess(gift: &str) {
    // 公主讨厌蛇，所以如果公主表示厌恶的话我们要停止
    if gift == "snake" {
        panic!("AAAaaaaa!!!!");
    }

    println!("I love {}s!!!!!", gift);
}

fn main() {
    give_princess("teddy bear");
    give_princess("snake");
}