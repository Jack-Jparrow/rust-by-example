use std::num::ParseIntError;

/**
 * @Author: 白银
 * @Date: 2022-09-01 17:19:55
 * @LastEditTime: 2022-09-01 17:31:00
 * @LastEditors: 白银
 * @Description: 使用 match 语句和提前返回（early return）的结合 https://rustwiki.org/zh-CN/rust-by-example/error/result/early_returns.html
 */

fn multiply(first_number_str: &str, second_number_str: &str) -> Result<i32, ParseIntError> {
    let first_number = match first_number_str.parse::<i32>() {
        Ok(first_number) => first_number,
        Err(e) => return Err(e),
    };
    let second_number = match second_number_str.parse::<i32>() {
        Ok(second_number) => second_number,
        Err(e) => return Err(e),
    };

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
