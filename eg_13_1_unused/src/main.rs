/**
 * @Author: Jack Jparrow
 * @Date: 2022-08-25 20:15:04
 * @LastEditTime: 2022-08-25 20:18:42
 * @LastEditors: Jack Jparrow
 * @Description: 编译器提供了 dead_code（死代码，无效代码）lint，这会对未使用的函数 产生警告。可以用一个属性来禁用这个 lint https://rustwiki.org/zh-CN/rust-by-example/attribute/unused.html
 */

fn used_function() {}

// `#[allow(dead_code)]` 属性可以禁用 `dead_code` lint
#[allow(dead_code)]
fn unused_function() {}

#[allow(dead_code)]
fn noisy_unused_function() {}

fn main() {
    used_function();
}
