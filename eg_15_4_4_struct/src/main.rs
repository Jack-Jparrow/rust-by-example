/**
 * @Author: 白银
 * @Date: 2022-08-30 20:42:40
 * @LastEditTime: 2022-08-30 21:17:49
 * @LastEditors: 白银
 * @Description: 在结构体中标注生命周期也和函数的类似 https://rustwiki.org/zh-CN/rust-by-example/scope/lifetime/struct.html
 */

// 一个 `Borrowed` 类型，含有一个指向 `i32` 类型的引用
// 该引用必须比 `Borrowed` 寿命更长
#[derive(Debug)]
struct Borrowed<'a>(&'a i32);

// 和前面类似，这里的两个引用都必须比这个结构体长寿
#[derive(Debug)]
#[allow(dead_code)]
struct NamedBorrowed<'a> {
    x: &'a i32,
    y: &'a i32,
}

// 一个枚举类型，其取值不是 `i32` 类型就是一个指向 `i32` 的引用
#[derive(Debug)]
enum Either<'a> {
    Num(i32),
    Ref(&'a i32),
}

fn main() {
    let x = 18;
    let y = 15;
    let single = Borrowed(&x);
    let double = NamedBorrowed { x: &x, y: &y };
    let reference = Either::Ref(&x);
    let number = Either::Num(y);

    println!("x is borrowed in {:?}", single);
    println!("x and y are borrowed in {:?}", double);
    println!("x is borrowed in {:?}", reference);
    println!("y is *not* borrowed in {:?}", number);
}
