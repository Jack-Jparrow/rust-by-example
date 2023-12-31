// 将 `deeply::nested::function` 路径绑定到 `other_function`
use deeply::nested::function as other_function;

/**
 * @Author: Jack Jparrow
 * @Date: 2022-08-24 20:33:46
 * @LastEditTime: 2022-08-24 20:37:57
 * @LastEditors: Jack Jparrow
 * @Description: use 声明可以将一个完整的路径绑定到一个新的名字，从而更容易访问 https://rustwiki.org/zh-CN/rust-by-example/mod/use.html
 */

fn function() {
    println!("called `function()`");
}

mod deeply {
    pub mod nested {
        pub fn function() {
            println!("called `deeply::nested::function()`");
        }
    }
}

fn main() {
    // 更容易访问 `deeply::nested::funcion`
    other_function();

    println!("Entering block");

    {
        // 这和 `use deeply::nested::function as function` 等价
        // 此 `function()` 将遮蔽外部的同名函数
        use deeply::nested::function;

        function();

        // `use` 绑定拥有局部作用域。在这个例子中，`function()` 的遮蔽只存在这个代码块中
        println!("Leaving block");
    }

    function();
}
