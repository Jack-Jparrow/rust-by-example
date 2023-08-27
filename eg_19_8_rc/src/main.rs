use std::rc::Rc;

/**
 * @Author: 白银
 * @Date: 2022-09-02 14:27:17
 * @LastEditTime: 2022-09-02 14:43:18
 * @LastEditors: 白银
 * @Description: 克隆 Rc 从不执行深拷贝。克隆只创建另一个指向包裹值的指针，并增加计数 https://rustwiki.org/zh-CN/rust-by-example/std/rc.html
 */

fn main() {
    let rc_examples = "Rc examples".to_string();

    {
        println!("--- rc_a is created ---");

        let rc_a: Rc<String> = Rc::new(rc_examples);

        println!("Reference Count of rc_a: {}", Rc::strong_count(&rc_a));

        {
            println!("--- rc_a is cloned to rc_b ---");

            let rc_b: Rc<String> = Rc::clone(&rc_a);
            println!("Reference Count of rc_b: {}", Rc::strong_count(&rc_b));
            println!("Reference Count of rc_a: {}", Rc::strong_count(&rc_a));
            
            // 如果两者内部的值相等的话，则两个 `Rc` 相等。
            println!("rc_a and rc_b are equal: {}", rc_a.eq(&rc_b));
                        
            // 我们可以直接使用值的方法
            println!("Length of the value inside rc_a: {}", rc_a.len());
            println!("Value of rc_b: {}", rc_b);
            
            println!("--- rc_b is dropped out of scope ---");
        }

        println!("Reference Count of rc_a: {}", Rc::strong_count(&rc_a));
        
        println!("--- rc_a is dropped out of scope ---");
    }

    // 报错！`rc_examples` 已经移入 `rc_a`
    // 而且当 `rc_a` 被删时，`rc_examples` 也被一起删除
    // println!("rc_examples: {}", rc_examples);
}