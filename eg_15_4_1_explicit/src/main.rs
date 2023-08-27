/**
 * @Author: 白银
 * @Date: 2022-08-30 19:59:11
 * @LastEditTime: 2022-08-30 20:06:30
 * @LastEditors: 白银
 * @Description: 显式生命周期标注的运用 https://rustwiki.org/zh-CN/rust-by-example/scope/lifetime/explicit.html
 */

// `print_refs` 接受两个 `i32` 的引用，它们有不同的生命周期 `'a` 和 `'b`
// 这两个生命周期都必须至少要和 `print_refs` 函数一样长
fn print_refs<'a, 'b>(x: &'a i32, y: &'b i32) {
    println!("x is {} and y is {}", x, y);
}

// 不带参数的函数，不过有一个生命周期参数 `'a`
fn failed_borrow<'a>() {
    let _x = 12;

    // 报错：`_x` 的生命周期不够长
    //let y: &'a i32 = &_x;
    // 在函数内部使用生命周期 `'a` 作为显式类型标注将导致失败，因为 `&_x` 的生命周期比 `y` 的短。短生命周期不能强制转换成长生命周期
}

fn main() {
    // 创建变量，稍后用于借用
    let (four, nine) = (4, 9);

    // 两个变量的借用（`&`）都传进函数
    print_refs(&four, &nine);
    // 任何被借用的输入量都必须比借用者生存得更长
    // 也就是说，`four` 和 `nine` 的生命周期都必须比 `print_refs` 的长

    failed_borrow();
    // `failed_borrow` 未包含引用，因此不要求 `'a` 长于函数的生命周期，但 `'a` 寿命确实更长。因为该生命周期从未被约束，所以默认为 `'static`
}
