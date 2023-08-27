/**
 * @Author: Jack Jparrow
 * @Date: 2022-08-21 20:26:34
 * @LastEditTime: 2022-08-21 20:49:43
 * @LastEditors: Jack Jparrow
 * @Description: 从 loop 循环中返回 https://rustwiki.org/zh-CN/rust-by-example/flow_control/loop/return.html
 */

fn main() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    assert_eq!(result, 20);
}
