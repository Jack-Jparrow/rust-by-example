/**
 * @Author: 白银
 * @Date: 2022-09-01 19:18:59
 * @LastEditTime: 2022-09-01 19:28:12
 * @LastEditors: 白银
 * @Description: 当你看着这些结果时，你会发现所有东西还在 Result 中保存着。要取出它们，需要一些 模板化的代码 https://rustwiki.org/zh-CN/rust-by-example/error/iter_result.html
 */

fn main() {
    let strings = vec!["tofu", "93", "18"];
    let (numbers, errors): (Vec<_>, Vec<_>) = strings
        .into_iter()
        .map(|s| s.parse::<i32>()).partition(Result::is_ok);
    let numbers: Vec<_> = numbers.into_iter().map(Result::unwrap).collect();
    let errors: Vec<_> = errors.into_iter().map(Result::unwrap_err).collect();
    
    println!("Numbers: {:?}", numbers);
    println!("Errors: {:?}", errors);
}