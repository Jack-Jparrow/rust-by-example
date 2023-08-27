/**
 * @Author: Jack Jparrow
 * @Date: 2022-08-20 21:06:26
 * @LastEditTime: 2022-08-20 21:31:48
 * @LastEditors: Jack Jparrow
 * @Description: 如果你为你的类型实现了 From，那么同时你也就免费获得了 Into https://rustwiki.org/zh-CN/rust-by-example/conversion/from_into.html
 */

#[derive(Debug)]
struct Number{
    value: i32,
}

impl From<i32> for Number {
    fn from(item: i32) -> Self {
        Number { value: item }
    }
}

fn main() {
    let int = 5;
    let num: Number = int.into();
    
    println!("My number is {:?}", num);
}