/**
 * @Author: 白银
 * @Date: 2022-08-31 19:43:14
 * @LastEditTime: 2022-08-31 20:18:39
 * @LastEditors: 白银
 * @Description: 给 drop 函数增加了打印到控制台的功能，用于宣布它在什么时候被调用 https://rustwiki.org/zh-CN/rust-by-example/trait/drop.html
 */

struct Droppable {
    name: &'static str,
}

// 这个简单的 `drop` 实现添加了打印到控制台的功能
impl Drop for Droppable {
    fn drop(&mut self) {
        println!("> Dropping {}", self.name);
    }
}

fn main() {
    let _a = Droppable { name: "a" };

    // 代码块 A
    {
        let _b = Droppable { name: "b" };

        // 代码块 B
        {
            let _c = Droppable { name: "c" };
            let _d = Droppable { name: "d" };

            println!("Exiting block B");
        }

        println!("Just exited block B");
        println!("Exiting block A");
    }

    println!("Just exited block A");
    // 变量可以手动使用 `drop` 函数来销毁
    drop(_a);

    println!("end of the main function");

    // `_a` *不会*在这里再次销毁，因为它已经被（手动）销毁
}
