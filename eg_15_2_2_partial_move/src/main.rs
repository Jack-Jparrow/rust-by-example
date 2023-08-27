/**
 * @Author: 白银
 * @Date: 2022-08-29 20:36:47
 * @LastEditTime: 2022-08-29 20:43:16
 * @LastEditors: 白银
 * @Description: 在单个变量的解构内，可以同时使用 by-move 和 by-reference 模式绑定 https://rustwiki.org/zh-CN/rust-by-example/scope/move/partial_move.html
 */

fn main() {
    struct Person {
        name: String,
        age: u8,
    }

    let person = Person {
        name: String::from("Alice"),
        age: 20,
    };
    // `name` 从 person 中移走，但 `age` 只是引用
    let Person { name, ref age } = person;

    println!("The person's age is {}", age);
    println!("The person's name is {}", name);

    // 报错！部分移动值的借用：`person` 部分借用产生
    //println!("The person struct is {:?}", person);

    // `person` 不能使用，但 `person.age` 因为没有被移动而可以继续使用
    println!("The person's age from person struct is {}", person.age);
}
