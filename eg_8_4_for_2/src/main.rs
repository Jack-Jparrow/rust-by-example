/**
 * @Author: Jack Jparrow
 * @Date: 2022-08-21 21:03:48
 * @LastEditTime: 2022-08-21 21:08:56
 * @LastEditors: Jack Jparrow
 * @Description: 使用 for 代替 while 来写 FizzBuzz 程序，用a..=b表示两端都包含在内的范围 https://rustwiki.org/zh-CN/rust-by-example/flow_control/for.html
 */

fn main() {
    for n in 1..=100 {
        if n % 15 == 0 {
            println!("fizzbuzz");
        } else if n % 3 == 0 {
            println!("fizz");
        } else if n % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", n);
        }
    }
}