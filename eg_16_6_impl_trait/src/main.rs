use std::iter;
use std::vec::IntoIter;

/**
 * @Author: 白银
 * @Date: 2022-08-31 20:19:08
 * @LastEditTime: 2022-08-31 20:57:42
 * @LastEditors: 白银
 * @Description: 如果函数返回实现了 MyTrait 的类型，可以将其返回类型编写为 -> impl MyTrait https://rustwiki.org/zh-CN/rust-by-example/trait/impl_trait.html
 */

// 该函数组合了两个 `Vec <i32>` 并在其上返回一个迭代器
fn combine_vecs_explicit_return_type(
    v: Vec<i32>,
    u: Vec<i32>,
) -> iter::Cycle<iter::Chain<IntoIter<i32>, IntoIter<i32>>> {
    v.into_iter().chain(u.into_iter()).cycle()
}

// 这是完全相同的函数，但其返回类型使用 `impl Trait`
fn combine_vecs(v: Vec<i32>, u: Vec<i32>) -> impl Iterator<Item = i32> {
    v.into_iter().chain(u.into_iter()).cycle()
}

fn main() {
    let v1 = vec![1, 2, 3];
    let v2 = vec![4, 5];
    let mut v3 = combine_vecs(v1, v2);

    assert_eq!(Some(1), v3.next());
    assert_eq!(Some(2), v3.next());
    assert_eq!(Some(3), v3.next());
    assert_eq!(Some(4), v3.next());
    assert_eq!(Some(5), v3.next());

    println!("all done");
}
