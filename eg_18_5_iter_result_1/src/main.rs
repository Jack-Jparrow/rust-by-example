/**
 * @Author: 白银
 * @Date: 2022-09-01 19:15:01
 * @LastEditTime: 2022-09-01 19:18:03
 * @LastEditors: 白银
 * @Description: Iter::map 操作可能失败 https://rustwiki.org/zh-CN/rust-by-example/error/iter_result.html
 */

fn main() {
    let strings = vec!["tofu", "93", "18"];
    let numbers: Vec<_> = strings.into_iter().map(|s| s.parse::<i32>()).collect();

    println!("Results: {:?}", numbers);
}
