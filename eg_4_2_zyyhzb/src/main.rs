/**
 * @Author: Jack Jparrow
 * @Date: 2022-08-18 21:43:19
 * @LastEditTime: 2022-08-18 21:51:18
 * @LastEditors: Jack Jparrow
 * @Description: 作用域和遮蔽 https://rustwiki.org/zh-CN/rust-by-example/variable_bindings/scope.html
 */

fn main() {
    // 此绑定生存于 main 函数中
    let long_lived_binding = 2;

    // ↓这是一个代码块，比 main 函数拥有更小的作用域
    {
        // 此绑定只存在于本代码块
        let short_lived_binding = 2;

        println!("inner short: {}", short_lived_binding);

        // 此绑定*遮蔽*了外面的绑定
        let long_lived_binding = 5_f32;

        println!("inner long: {}", long_lived_binding);
    }
    // ↑代码块结束

    // 报错！`short_lived_binding` 在此作用域上不存在
    // println!("outer short: {}", short_lived_binding);

    println!("outer long: {}", long_lived_binding);

    // 此绑定同样*遮蔽*了前面的绑定
    let long_lived_binding = 'a';

    println!("outer long: {}", long_lived_binding);
}
