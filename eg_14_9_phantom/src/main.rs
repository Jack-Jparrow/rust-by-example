use std::marker::PhantomData;

/**
 * @Author: 白银
 * @Date: 2022-08-28 21:46:38
 * @LastEditTime: 2022-08-28 21:59:15
 * @LastEditors: 白银
 * @Description: 使用 std::marker::PhantomData 作为虚类型参数的类型，创建 包含不同数据类型的元组 https://rustwiki.org/zh-CN/rust-by-example/generics/phantom.html
 */

// 这个虚元组结构体对 `A` 是泛型的，并且带有隐藏参数 `B`
#[derive(PartialEq)] // 允许这种类型进行相等测试（equality test）
struct PhantomTuple<A, B>(A, PhantomData<B>);
// 这个虚类型结构体对 `A` 是泛型的，并且带有隐藏参数 `B`
#[derive(PartialEq)] // 允许这种类型进行相等测试
struct PhantomStruct<A, B> {
    first: A,
    phantom: PhantomData<B>,
}

// 注意：对于泛型 `A` 会分配存储空间，但 `B` 不会，因此，`B` 不能参与运算

fn main() {
    // 这里的 `f32` 和 `f64` 是隐藏参数

    // 被指定为 `<char, f32>` 的 `PhantomTuple` 类型
    let _tuple1: PhantomTuple<char, f32> = PhantomTuple('Q', PhantomData);
    // 被指定为 `<char, f64>` `PhantomTuple` 类型
    let _tuple2: PhantomTuple<char, f64> = PhantomTuple('Q', PhantomData);
    // 被指定为 `<char, f32>` 的类型
    let _struct1: PhantomStruct<char, f32> = PhantomStruct {
        first: 'Q',
        phantom: PhantomData,
    };
    // 被指定为 `<char, f64>` 的类型
    let _struct2: PhantomStruct<char, i64> = PhantomStruct {
        first: 'Q',
        phantom: PhantomData,
    };

    // 编译期错误！类型不匹配，所以这些值不能够比较：
    // println!("_tuple1 == _tuple2 yields: {}", _tuple1 == _tuple2);

    // 编译期错误！类型不匹配，所以这些值不能够比较：
    // println!("_struct1 == _struct2 yields: {}", _struct1 == _struct2);
}
