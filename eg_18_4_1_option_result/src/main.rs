use std::num::ParseIntError;

/**
 * @Author: 白银
 * @Date: 2022-09-01 17:35:11
 * @LastEditTime: 2022-09-01 17:49:15
 * @LastEditors: 白银
 * @Description: 处理混合错误类型的最基本的手段就是让它们互相包含
 */

fn double_first(vec: Vec<&str>) -> Option<Result<i32, ParseIntError>> {
    vec.first().map(|first| first.parse::<i32>().map(|n| 2 * n))
}

fn main() {
    let numbers = vec!["42", "93", "18"];
    let empty = vec![];
    let strings = vec!["tofu", "93", "18"];

    println!("The first doubled is {:?}", double_first(numbers));
    println!("The first doubled is {:?}", double_first(empty));
    // 错误1：输入 vector 为空
    println!("The first doubled is {:?}", double_first(strings));
    // 错误2：此元素不能解析成数字
}
