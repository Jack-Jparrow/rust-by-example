use std::num::ParseIntError;

/**
 * @Author: 白银
 * @Date: 2022-09-01 17:25:52
 * @LastEditTime: 2022-09-01 17:34:06
 * @LastEditors: 白银
 * @Description: ? 几乎就等于一个会返回 Err 而不是 panic 的 unwrap https://rustwiki.org/zh-CN/rust-by-example/error/result/enter_question_mark.html
 */

fn multiply(first_number_str: &str, second_number_str: &str) -> Result<i32, ParseIntError> {
    let first_number = first_number_str.parse::<i32>()?;
    let second_number = second_number_str.parse::<i32>()?;

    Ok(first_number * second_number)
}

fn print(result: Result<i32, ParseIntError>) {
    match result {
        Ok(n) => println!("n is {}", n),
        Err(e) => println!("Error: {}", e),
    }
}

fn main() {
    print(multiply("10", "2"));
    print(multiply("t", "2"));
}
