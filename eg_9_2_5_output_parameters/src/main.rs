/**
 * @Author: Jack Jparrow
 * @Date: 2022-08-23 21:22:16
 * @LastEditTime: 2022-08-23 21:30:54
 * @LastEditors: Jack Jparrow
 * @Description: 闭包作为输入参数是可能的，所以返回闭包作为输出参数（output parameter）也应该是可能的 https://rustwiki.org/zh-CN/rust-by-example/fn/closures/output_parameters.html
 */

fn create_fn() -> impl Fn() {
    let text = "Fn".to_owned();

    move || println!("This is a: {}", text)
}

fn create_fnmut() -> impl FnMut() {
    let text = "FnMut".to_owned();

    move || println!("This is a: {}", text)
}

fn create_fnonce() -> impl FnOnce() {
    let text = "FnOnce".to_owned();

    move || println!("This is a: {}", text)
}

fn main() {
    let fn_plain = create_fn();
    let mut fn_mut = create_fnmut();
    let fn_once = create_fnonce();

    fn_plain();
    fn_mut();
    fn_once();
}
