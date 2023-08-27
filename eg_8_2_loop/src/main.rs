/**
 * @Author: Jack Jparrow
 * @Date: 2022-08-21 20:11:17
 * @LastEditTime: 2022-08-21 20:14:03
 * @LastEditors: Jack Jparrow
 * @Description: 可以使用 break 语句在任何时候退出一个循环，还可以使用 continue 跳过循环体的剩余部分并开始下一轮循环 https://rustwiki.org/zh-CN/rust-by-example/flow_control/loop.html
 */

fn main() {
    let mut count = 0u32;

    println!("Let's count until infinity!");

    // 无限循环
    loop {
        count += 1;

        if count == 3 {
            println!("three");

            // 跳过这次迭代的剩下内容
            continue;
        }

        println!("{}", count);

        if count == 5 {
            println!("OK, that's enough");

            // 退出循环
            break;
        }
    }
}
