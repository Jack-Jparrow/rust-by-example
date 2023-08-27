/**
 * @Author: 白银
 * @Date: 2022-09-01 19:18:59
 * @LastEditTime: 2022-09-01 19:26:29
 * @LastEditors: 白银
 * @Description: 使用 Partition() 收集所有合法的值与错误 https://rustwiki.org/zh-CN/rust-by-example/error/iter_result.html
 */

fn main() {
    let strings = vec!["tofu", "93", "18"];
    let (numbers, errors): (Vec<_>, Vec<_>) = strings
        .into_iter()
        .map(|s| s.parse::<i32>()).partition(Result::is_ok);

    println!("Numbers: {:?}", numbers);
    println!("Errors: {:?}", errors);
}