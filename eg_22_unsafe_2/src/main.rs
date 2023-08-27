use std::slice;

/**
 * @Author: 白银
 * @Date: 2022-09-02 17:29:48
 * @LastEditTime: 2022-09-02 18:37:36
 * @LastEditors: 白银
 * @Description: 一些函数可以声明为不安全的（unsafe），这意味着在使用它时保证正确性不再是编译器 的责任，而是程序员的 https://rustwiki.org/zh-CN/rust-by-example/unsafe.html
 */

fn main() {
    let some_vector = vec![1, 2, 3, 4];
    let pointer = some_vector.as_ptr();
    let length = some_vector.len();

    unsafe {
        let my_slice: &[u32] = slice::from_raw_parts(pointer, length);

        assert_eq!(some_vector.as_slice(), my_slice);
    }
}
