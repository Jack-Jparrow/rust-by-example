/**
 * @Author: 白银
 * @Date: 2022-09-01 19:29:24
 * @LastEditTime: 2022-09-01 20:40:16
 * @LastEditors: 白银
 * @Description: vector 是大小可变的数组。和 slice（切片）类似，它们的大小在编译时是未知的，但 它们可以随时扩大或缩小 https://rustwiki.org/zh-CN/rust-by-example/std/vec.html
 */

fn main() {
    // 迭代器可以被收集到 vector 中
    let collected_iterator: Vec<i32> = (0..10).collect();
    println!("Collected (0..10) into: {:?}", collected_iterator);

    // `vec!` 宏可用来初始化一个 vector
    let mut xs = vec![1i32, 2, 3];
    println!("Initial vector: {:?}", xs);

    // 在 vector 的尾部插入一个新的元素
    println!("Push 4 into the vector");
    xs.push(4);
    println!("Vector: {:?}", xs);

    // 报错！不可变 vector 不可增长
    // collected_iterator.push(0);

    // `len` 方法获得一个 vector 的当前大小
    println!("Vector size: {}", xs.len());
    // 下标使用中括号表示（从 0 开始）
    println!("Second element: {}", xs[1]);
    // `pop` 移除 vector 的最后一个元素并将它返回
    println!("Pop last element: {:?}", xs.pop());
    // 超出下标范围将抛出一个 panic
    // println!("Fourth element: {}", xs[3]);
    // 迭代一个 `Vector` 很容易
    println!("Contents of xs:");
    for x in xs.iter() {
        println!("> {}", x);
    }

    // 可以在迭代 `Vector` 的同时，使用独立变量（`i`）来记录迭代次数
    for (i, x) in xs.iter().enumerate() {
        println!("In position {} we have value {}", i, x);
    }

    // 多亏了 `iter_mut`，可变的 `Vector` 在迭代的同时，其中每个值都能被修改
    for x in xs.iter_mut() {
        *x *= 3;
    }

    println!("Updated vector: {:?}", xs);
}
