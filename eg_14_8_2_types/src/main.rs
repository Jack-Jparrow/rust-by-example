/**
 * @Author: 白银
 * @Date: 2022-08-28 21:31:48
 * @LastEditTime: 2022-08-28 21:46:14
 * @LastEditors: 白银
 * @Description: 通过把容器内部的类型放到 trait 中作为输出类型，使用 “关联类型” 增加了代码 的可读性 https://rustwiki.org/zh-CN/rust-by-example/generics/assoc_items/types.html
 */

struct Container(i32, i32);

// 这个 trait 检查给定的 2 个项是否储存于容器中并且能够获得容器的第一个或最后一个值
trait Contains {
    // 在这里定义可以被方法使用的泛型类型
    type A;
    type B;

    fn contains(&self, _: &Self::A, _: &Self::B) -> bool; // 显式地要求 `A` 和 `B`
    fn first(&self) -> i32; // 未显式地要求 `A` 或 `B`
    fn last(&self) -> i32; // 未显式地要求 `A` 或 `B`
}

impl Contains for Container {
    // 指出 `A` 和 `B` 是什么类型。如果 `input`（输入）类型为 `Container(i32, i32)`，那么 `output`（输出）类型会被确定为 `i32` 和 `i32`
    type A = i32;
    type B = i32;

    // 如果存储的数字和给定的相等则为真
    // `&Self::A` 和 `&Self::B` 在这里也是合法的类型
    fn contains(&self, number_1: &i32, number_2: &i32) -> bool {
        (&self.0 == number_1) && (&self.1 == number_2)
    }

    // 得到第一个数字
    fn first(&self) -> i32 {
        self.0
    }

    // 得到最后一个数字
    fn last(&self) -> i32 {
        self.1
    }
}

// 容器 `C` 就包含了 `A` 和 `B` 类型。鉴于此，必须指出 `A` 和 `B` 显得很麻烦
fn difference<C: Contains>(container: &C) -> i32 {
    container.last() - container.first()
}

fn main() {
    let number_1 = 3;
    let number_2 = 10;
    let container = Container(number_1, number_2);

    println!(
        "Does container contain {} and {}: {}",
        &number_1,
        &number_2,
        container.contains(&number_1, &number_2)
    );
    println!("First number: {}", container.first());
    println!("Last number: {}", container.last());
    println!("The difference is: {}", difference(&container));
}
