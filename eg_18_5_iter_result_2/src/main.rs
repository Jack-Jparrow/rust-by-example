/**
 * @Author: 白银
 * @Date: 2022-09-01 19:15:36
 * @LastEditTime: 2022-09-01 19:21:34
 * @LastEditors: 白银
 * @Description: filter_map 会调用一个函数，过滤掉为 None 的所有结果 https://rustwiki.org/zh-CN/rust-by-example/error/iter_result.html
 */

fn main() {
    let strings = vec!["tofu", "93", "18"];
    let numbers: Vec<_> = strings
        .into_iter()
        .filter_map(|s| s.parse::<i32>().ok())
        .collect();

    println!("Results: {:?}", numbers);
}
