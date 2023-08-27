/**
 * @Author: Jack Jparrow
 * @Date: 2022-08-24 20:59:12
 * @LastEditTime: 2022-08-24 20:59:40
 * @LastEditors: Jack Jparrow
 * @Description: 让我们创建一个库，然后看看如何把它链接到另一个 crate https://rustwiki.org/zh-CN/rust-by-example/crates/lib.html
 */

pub fn public_function() {
    println!("called rary's `public_function()`");
}

fn private_function() {
    println!("called rary's `private_function()`");
}

pub fn indirect_access() {
    print!("called rary's `indirect_access()`, that\n> ");

    private_function();
}
