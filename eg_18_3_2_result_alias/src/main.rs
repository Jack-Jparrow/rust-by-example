use std::num::ParseIntError;

/**
 * @Author: 白银
 * @Date: 2022-09-01 16:33:28
 * @LastEditTime: 2022-09-01 17:25:09
 * @LastEditors: 白银
 * @Description: 同一模块中的错误常常会有相同的 Err 类 型，所以单个别名就能简便地定义所有相关的 Result https://rustwiki.org/zh-CN/rust-by-example/error/result/result_alias.html
 */

// 为带有错误类型 `ParseIntError` 的 `Result` 定义一个泛型别名
type AliasedResult<T> = Result<T, ParseIntError>;

// 使用上面定义过的别名来表示上一节中的 `Result<i32,ParseIntError>` 类型
fn multiply(first_number_str: &str, second_number_str: &str) -> AliasedResult<i32> {
    first_number_str.parse::<i32>().and_then(|first_number| {
        second_number_str
            .parse::<i32>()
            .map(|second_number| first_number * second_number)
    })
}

// 在这里使用别名又让我们节省了一些代码量
fn print(result: AliasedResult<i32>) {
    match result {
        Ok(n) => println!("n is {}", n),
        Err(e) => println!("Error: {}", e),
    }
}

fn main() {
    print(multiply("10", "2"));
    print(multiply("t", "2"));
}
