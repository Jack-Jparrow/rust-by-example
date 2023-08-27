/**
 * @Author: 白银
 * @Date: 2022-08-31 19:25:29
 * @LastEditTime: 2022-08-31 19:42:38
 * @LastEditors: 白银
 * @Description: dyn 关键字返回指向堆的 trait 指针 https://rustwiki.org/zh-CN/rust-by-example/trait/dyn.html
 */

struct Sheep {}
struct Cow {}

trait Animal {
    // 实例方法签名
    fn noise(&self) -> &'static str;
}

// 实现 `Sheep` 的 `Animal` trait
impl Animal for Sheep {
    fn noise(&self) -> &'static str {
        "baaaaah!"
    }
}

// 实现 `Cow` 的 `Animal` trait
impl Animal for Cow {
    fn noise(&self) -> &'static str {
        "moooooo!"
    }
}

// 返回一些实现 Animal 的结构体，但是在编译时我们不知道哪个结构体
fn random_animal(random_number: f64) -> Box<dyn Animal> {
    if random_number < 0.5 {
        Box::new(Sheep {})
    } else {
        Box::new(Cow {})
    }
}

fn main() {
    let random_number = 0.234;
    let animal = random_animal(random_number);

    println!(
        "You've randomly chosen an animal, and it says {}",
        animal.noise()
    );
}
