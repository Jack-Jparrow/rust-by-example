/**
 * @Author: Jack Jparrow
 * @Date: 2022-08-23 21:52:03
 * @LastEditTime: 2022-08-23 21:53:42
 * @LastEditors: Jack Jparrow
 * @Description: 虽然返回值中没有信息，但此函数会照常返回 https://rustwiki.org/zh-CN/rust-by-example/fn/diverging.html
 */

fn some_fn() {
    ()
}

fn main() {
    let a: () = some_fn();

    println!("This function returns and you can see this line.");
}
