/**
 * @Author: Jack Jparrow
 * @Date: 2022-08-20 22:11:16
 * @LastEditTime: 2022-08-20 22:14:20
 * @LastEditors: Jack Jparrow
 * @Description: 表达式 https://rustwiki.org/zh-CN/rust-by-example/expression.html
 */

fn main() {
    let x = 5u32;
    let y = {
        let x_squared = x * x;
        let x_cube = x_squared * x;

        // 将此表达式赋给 `y`
        x_cube + x_squared + x
    };
    let z = {
        // 分号结束了这个表达式，于是将 `()` 赋给 `z`
        2 * x;
    };

    println!("x is {:?}", x);
    println!("y is {:?}", y);
    println!("z is {:?}", z);
}
