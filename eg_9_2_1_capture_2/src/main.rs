/**
 * @Author: Jack Jparrow
 * @Date: 2022-08-23 20:48:07
 * @LastEditTime: 2022-08-23 20:51:12
 * @LastEditors: Jack Jparrow
 * @Description: 在竖线 | 之前使用 move 会强制闭包取得被捕获变量的所有权 https://rustwiki.org/zh-CN/rust-by-example/fn/closures/capture.html
 */

fn main() {
    // `Vec` 在语义上是不可复制的
    let haystack = vec![1, 2, 3];
    let contains = move |needle| haystack.contains(needle);

    println!("{}", contains(&1));
    println!("{}", contains(&4));

    //println!("There're {} elements in vec", haystack.len());
    // ^ 取消上面一行的注释将导致编译时错误，因为借用检查不允许在变量被移动走之后继续使用它

    // 在闭包的签名中删除 `move` 会导致闭包以不可变方式借用 `haystack`，因此之后 `haystack` 仍然可用，取消上面的注释也不会导致错误
}
