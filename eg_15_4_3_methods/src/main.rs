/**
 * @Author: 白银
 * @Date: 2022-08-30 20:08:04
 * @LastEditTime: 2022-08-30 21:12:17
 * @LastEditors: 白银
 * @Description: 方法的标注和函数类似 https://rustwiki.org/zh-CN/rust-by-example/scope/lifetime/methods.html
 */

struct Owner(i32);

impl Owner{
    // 标注生命周期，就像独立的函数一样
    fn add_one<'a>(&'a mut self){
        self.0 += 1;
    }

    fn print<'a>(&'a self) {
        println!("`print`: {}", self.0);
    }
}

fn main() {
    let mut owner = Owner(18);

    owner.add_one();
    owner.print();
}