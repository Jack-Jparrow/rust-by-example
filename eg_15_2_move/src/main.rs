/**
 * @Author: 白银
 * @Date: 2022-08-29 19:45:46
 * @LastEditTime: 2022-08-29 20:29:44
 * @LastEditors: 白银
 * @Description: 在移动资源之后，原来的所有者不能再被使用，这可避免悬挂指针（dangling pointer）的产生 https://rustwiki.org/zh-CN/rust-by-example/scope/move.html
 */

fn destroy_box(c: Box<i32>) {
    println!("Destroying a box that contains {}", c);

    // `c` 被销毁且内存得到释放
}

fn main() {
    // 栈分配的整型
    let x = 5u32;
    // 将 `x` *复制*到 `y`——不存在资源移动
    let y = x;

    // 两个值各自都可以使用
    println!("x is {}, and y is {}", x, y);

    // `a` 是一个指向堆分配的整数的指针
    let a = Box::new(5i32);

    println!("a contains: {}", a);

    // *移动* `a` 到 `b`
    let b = a;
    // 把 `a` 的指针地址（而非数据）复制到 `b`。现在两者都指向同一个堆分配的数据，但是现在是 `b` 拥有它

    // 报错！`a` 不能访问数据，因为它不再拥有那部分堆上的内存
    //println!("a contains: {}", a);

    // 此函数从 `b` 中取得堆分配的内存的所有权
    destroy_box(b);

    // 此时堆内存已经被释放，这个操作会导致解引用已释放的内存，而这是编译器禁止的
    // 报错！和前面出错的原因一样
    //println!("b contains: {}", b);
}