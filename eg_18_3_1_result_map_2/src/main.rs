use std::num::ParseIntError;

/**
 * @Author: 白银
 * @Date: 2022-09-01 17:12:09
 * @LastEditTime: 2022-09-01 17:18:15
 * @LastEditors: 白银
 * @Description: Option 的 map、and_then、以及很多其他组合算子也为 Result 实现了 https://rustwiki.org/zh-CN/rust-by-example/error/result/result_map.html
 */

// 就像 `Option` 那样，我们可以使用 `map()` 之类的组合算子
// 除去写法外，这个函数与上面那个完全一致，它的作用是：如果值是合法的，计算其乘积，否则返回错误
fn multiply(first_number_str: &str, second_number_str: &str) -> Result<i32, ParseIntError> {
    first_number_str.parse::<i32>().and_then(|first_number| {
        second_number_str
            .parse::<i32>()
            .map(|second_number| first_number * second_number)
    })
}

fn print(result: Result<i32, ParseIntError>) {
    match result {
        Ok(n) => println!("n is {}", n),
        Err(e) => println!("Error: {}", e),
    }
}

fn main() {
    // 这种情况下仍然会给出正确的答案
    let twenty = multiply("10", "2");
    print(twenty);

    // 这种情况下就会提供一条更有用的错误信息
    let tt = multiply("t", "2");
    print(tt)
}
