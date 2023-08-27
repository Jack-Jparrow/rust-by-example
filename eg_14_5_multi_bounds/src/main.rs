use std::fmt::{Debug, Display};

/**
 * @Author: 白银
 * @Date: 2022-08-28 20:03:00
 * @LastEditTime: 2022-08-28 20:13:50
 * @LastEditors: 白银
 * @Description: 多重约束（multiple bounds）可以用 + 连接。和平常一样，类型之间使用 , 隔开 https://rustwiki.org/zh-CN/rust-by-example/generics/multi_bounds.html
 */

fn compare_prints<T: Debug + Display>(_t: &T) {
    println!("Debug: `{:?}`", _t);
    println!("Display: `{}`", _t);
}

fn compare_types<T: Debug, U: Debug>(t: &T, _u: &U) {
    println!("t: `{:?}", t);
    println!("u: `{:?}", _u);
}

fn main() {
    let string = "words";
    let array = [1, 2, 3];
    let vec = vec![1, 2, 3];

    compare_prints(&string);
    compare_types(&array, &vec);
}
