/**
 * @Author: 白银
 * @Date: 2022-09-01 17:31:45
 * @LastEditTime: 2022-09-01 17:39:18
 * @LastEditors: 白银
 * @Description: unwrap 的两个实例生成了不同的错误类型。Vec::first 返回一个 Option，而 parse::<i32> 返回一个 Result<i32, ParseIntError> https://rustwiki.org/zh-CN/rust-by-example/error/multiple_error_types.html
 */

fn double_first(vec: Vec<&str>) -> i32 {
    let first = vec.first().unwrap(); // 生成错误 1

    2 * first.parse::<i32>().unwrap() // 生成错误 2
}

fn main() {
    let numbers = vec!["42", "93", "18"];
    let empty = vec![];
    let strings = vec!["tofu", "93", "18"];

    println!("The first doubled is {}", double_first(numbers));
    println!("The first doubled is {}", double_first(empty));
    // 错误1：输入 vector 为空
    println!("The first doubled is {}", double_first(strings));
    // 错误2：此元素不能解析成数字
}
