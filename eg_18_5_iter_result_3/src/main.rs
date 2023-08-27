/**
 * @Author: 白银
 * @Date: 2022-09-01 19:18:59
 * @LastEditTime: 2022-09-01 19:23:35
 * @LastEditors: 白银
 * @Description: 使用 collect() 使整个操作失败 https://rustwiki.org/zh-CN/rust-by-example/error/iter_result.html
 */

fn main() {
    let strings = vec!["tofu", "93", "18"];
    let numbers: Result<Vec<_>, _> = strings.into_iter().map(|s| s.parse::<i32>()).collect();

    println!("Results: {:?}", numbers);
}
