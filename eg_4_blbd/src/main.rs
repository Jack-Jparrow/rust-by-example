/**
 * @Author: Jack Jparrow
 * @Date: 2022-08-18 20:31:32
 * @LastEditTime: 2022-08-18 21:17:23
 * @LastEditors: Jack Jparrow
 * @Description: 使用 let 绑定操作可以将值（比如字面量）绑定（bind）到变量 https://rustwiki.org/zh-CN/rust-by-example/variable_bindings.html
 */

fn main() {
    let an_integer = 1u32;
    let a_boolean = true;
    let unit = ();
    // 将 `an_integer` 复制到 `copied_integer`
    let copied_integer = an_integer;

    println!("An integer: {:?}", copied_integer);
    println!("A boolean: {:?}", a_boolean);
    println!("Meet the unit value: {:?}", unit);

    // 编译器会对未使用的变量绑定产生警告；可以给变量名加上下划线前缀来消除警告
    let _unused_variable = 3u32;
}
