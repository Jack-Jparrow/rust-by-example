/**
 * @Author: Jack Jparrow
 * @Date: 2022-08-21 21:03:04
 * @LastEditTime: 2022-08-21 21:06:43
 * @LastEditors: Jack Jparrow
 * @Description: for 代替 while 来写 FizzBuzz 程序 https://rustwiki.org/zh-CN/rust-by-example/flow_control/for.html
 */

fn main() {
    // `n` 将在每次迭代中分别取 1, 2, ..., 100
    for n in 1..101 {
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
