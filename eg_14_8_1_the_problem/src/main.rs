/**
 * @Author: 白银
 * @Date: 2022-08-28 21:19:55
 * @LastEditTime: 2022-08-28 21:29:42
 * @LastEditors: 白银
 * @Description: trait 如果对实现了它的容器类型是泛型的，则须遵守类型规范要求——trait 的 使用者必须指出 trait 的全部泛型类型 https://rustwiki.org/zh-CN/rust-by-example/generics/assoc_items/the_problem.html
 */

struct Container(i32, i32);

// 这个 trait 检查给定的 2 个项是否储存于容器中并且能够获得容器的第一个或最后一个值
trait Contains<A, B> {
    fn contains(&self, _: &A, _: &B) -> bool; // 显式地要求 `A` 和 `B`
    fn first(&self) -> i32; // 未显式地要求 `A` 或 `B`
    fn last(&self) -> i32; // 未显式地要求 `A` 或 `B`
}

impl Contains<i32, i32> for Container {
    // 如果存储的数字和给定的相等则为真
    fn contains(&self, number_1: &i32, number_2: &i32) -> bool {
        (&self.0 == number_1) && (&self.1 == number_2)
    }

    fn first(&self) -> i32 {
        self.0
    }

    fn last(&self) -> i32 {
        self.1
    }
}

// 容器 `C` 就包含了 `A` 和 `B` 类型。鉴于此，必须指出 `A` 和 `B` 显得很麻烦
fn difference<A, B, C>(container: &C) -> i32
where
    C: Contains<A, B>,
{
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
