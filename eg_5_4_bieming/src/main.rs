/**
 * @Author: Jack Jparrow
 * @Date: 2022-08-19 21:45:01
 * @LastEditTime: 2022-08-19 21:49:12
 * @LastEditors: Jack Jparrow
 * @Description: 可以用 type 语句给已有的类型取个新的名字。&类型的名字必须遵循驼峰命名法（像是 CamelCase 这样），否则编译器将给出警告。原生类型是例外 https://rustwiki.org/zh-CN/rust-by-example/types/alias.html
 */

// `NanoSecond` 是 `u64` 的新名字
type NanoSecond = u64;
type Inch = u64;

#[allow(non_camel_case_types)]
type u64_t = u64;

fn main() {
    // `NanoSecond` = `Inch` = `u64_t` = `u64`
    let nanoseconds: NanoSecond = 5 as u64_t;
    let inches: Inch = 2 as u64_t;

    // 注意类型别名*并不能*提供额外的类型安全，因为别名*并不是*新的类型
    println!(
        "{} nanoseconds + {} inches = {} unit?",
        nanoseconds,
        inches,
        nanoseconds + inches
    );
}
