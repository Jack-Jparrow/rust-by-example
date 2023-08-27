/**
 * @Author: 白银
 * @Date: 2022-09-01 14:21:39
 * @LastEditTime: 2022-09-01 14:34:14
 * @LastEditors: 白银
 * @Description: 我们可以把之前的 calculate! 宏改写成可变参数接口 https://rustwiki.org/zh-CN/rust-by-example/macros/variadics.html
 */

macro_rules! calculate {
    // 单个 `eval` 的模式
    (eval $e: expr) => {
        {
            let val: usize = $e;// 强制类型转换为int

            println!("{} = {}", stringify!{$e}, val);
        }
    };

    // 递归地拆解多重的 `eval`
    (eval $e: expr, $(eval $es: expr), +) => {
        {
            calculate!{eval $e}
            calculate!{$(eval $es), +}
        }
    }
}

fn main() {
    calculate!(
        eval 1 + 2,
        eval 3 + 4,
        eval (2 * 3) + 1
    )
}
