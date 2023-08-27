/**
 * @Author: Jack Jparrow
 * @Date: 2022-08-20 21:51:25
 * @LastEditTime: 2022-08-20 22:08:24
 * @LastEditors: Jack Jparrow
 * @Description: 把字符串转成数字，只要对目标类型实现了 FromStr trait，就可以用 parse 把字符串转换成目标类型 https://rustwiki.org/zh-CN/rust-by-example/conversion/string.html
 */

fn main() {
    let parsed: i32 = "5".parse().unwrap();
    let turbo_parsed = "10".parse::<i32>().unwrap();
    let sum = parsed + turbo_parsed;

    println!("Sum: {:?}", sum);
}
