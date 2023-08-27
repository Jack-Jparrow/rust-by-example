/**
 * @Author: Jack Jparrow
 * @Date: 2022-08-21 20:51:06
 * @LastEditTime: 2022-08-21 21:02:01
 * @LastEditors: Jack Jparrow
 * @Description: while 循环写FizzBuzz https://rustwiki.org/zh-CN/rust-by-example/flow_control/while.html
 */

fn main() {
    // 计数器变量
    let mut n = 1;

    // 当 `n` 小于 101 时循环
    while n < 101 {
        if n % 15 == 0 {
            println!("fizzbuzz");
        } else if n % 3 == 0 {
            println!("fizz");
        } else if n % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", n);
        }

        // 计数器值加 1
        n += 1;
    }
}
