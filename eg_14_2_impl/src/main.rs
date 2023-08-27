/**
 * @Author: 白银
 * @Date: 2022-08-27 21:56:56
 * @LastEditTime: 2022-08-27 21:59:43
 * @LastEditors: 白银
 * @Description: 和函数类似，impl 块要想实现泛型，也需要很仔细 https://rustwiki.org/zh-CN/rust-by-example/generics/impl.html
 */

struct Val {
    val: f64,
}

struct GenVal<T> {
    gen_val: T,
}

// Val 的 `impl`
impl Val {
    fn value(&self) -> &f64 {
        &self.val
    }
}

// GenVal 的 `impl`，指定 `T` 是泛型类型
impl<T> GenVal<T> {
    fn value(&self) -> &T {
        &self.gen_val
    }
}

fn main() {
    let x = Val { val: 3.0 };
    let y = GenVal { gen_val: 3i32 };

    println!("{}, {}", x.value(), y.value());
}
