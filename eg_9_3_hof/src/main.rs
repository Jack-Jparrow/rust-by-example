/**
 * @Author: Jack Jparrow
 * @Date: 2022-08-23 21:43:43
 * @LastEditTime: 2022-08-23 21:50:05
 * @LastEditors: Jack Jparrow
 * @Description: Rust 提供了高阶函数（Higher Order Function, HOF），指那些输入一个或多个函数，并且/或者产生一个更有用的函数的函数 https://rustwiki.org/zh-CN/rust-by-example/fn/hof.html
 */

fn is_odd(n: u32) -> bool {
    n % 2 == 1
}

fn main() {
    println!("Find the sum of all the squared odd numbers under 1000");

    let upper = 1000;
    // 命令式（imperative）的写法
    // 声明累加器变量
    let mut acc = 0;

    // 迭代：0，1, 2, ... 到无穷大
    for n in 0.. {
        let n_squared = n * n;

        if n_squared >= upper {
            // 若大于上限则退出循环
            break;
        } else if is_odd(n_squared) {
            // 如果是奇数就计数
            acc += n_squared;
        }
    }

    println!("imperative style: {}", acc);

    // 函数式的写法
    let sum_of_squared_odd_numbers: u32 = (0..)
        .map(|n| n * n) // 所有自然数取平方
        .take_while(|&n| n < upper) // 取小于上限的
        .filter(|&n| is_odd(n)) // 取奇数
        .fold(0, |sum, i| sum + i); // 最后加起来

    println!("functional style: {}", sum_of_squared_odd_numbers)
}
