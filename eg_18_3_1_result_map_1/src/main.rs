use std::num::ParseIntError;

/**
 * @Author: 白银
 * @Date: 2022-09-01 17:02:46
 * @LastEditTime: 2022-09-01 17:10:57
 * @LastEditors: 白银
 * @Description: 使用简单的 match 语句导致了更加繁琐的代码 https://rustwiki.org/zh-CN/rust-by-example/error/result/result_map.html
 */

// 修改了上一节中的返回类型，现在使用模式匹配而不是 `unwrap()`
fn multiply(first_number_str: &str, second_number_str: &str) -> Result<i32, ParseIntError> {
    match first_number_str.parse::<i32>() {
        Ok(first_number) => match second_number_str.parse::<i32>() {
            Ok(second_number) => Ok(first_number * second_number),
            Err(e) => Err(e),
        },
        Err(e) => Err(e),
    }
}

fn print(result: Result<i32, ParseIntError>) {
    match result {
        Ok(n) => println!("n is {}", n),
        Err(e) => println!("Error: {}", e),
    }
}

fn main() {
    // 这种情形下仍然会给出正确的答案
    let twenty = multiply("10", "2");
    print(twenty);

    // 这种情况下就会提供一条更有用的错误信息
    let tt = multiply("t", "2");
    print(tt);
}
