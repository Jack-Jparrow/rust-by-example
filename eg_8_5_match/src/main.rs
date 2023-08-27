/**
 * @Author: Jack Jparrow
 * @Date: 2022-08-22 20:02:24
 * @LastEditTime: 2022-08-22 20:57:32
 * @LastEditors: Jack Jparrow
 * @Description: 通过 match 关键字来提供模式匹配，和 C 语言的 switch 用法类似。第一个匹配分支会被比对，并且所有可能的值都必须被覆盖 https://rustwiki.org/zh-CN/rust-by-example/flow_control/match.html
 */

fn main() {
    let number = 13;

    println!("Tell me about {}", number);

    match number {
        // 匹配单个值
        1 => println!("One!"),
        // 匹配多个值
        2 | 3 | 5 | 7 | 11 => println!("This is a prime"),
        // 匹配一个闭区间范围
        13..=19 => println!("A teen"),
        // 处理其他情况
        _ => println!("Ain't special"),
    }

    let boolean = true;
    // match 也是一个表达式
    let binary = match boolean {
        // match 分支必须覆盖所有可能的值
        false => 0,
        true => 1,
    };

    println!("{} -> {}", boolean, binary);
}
