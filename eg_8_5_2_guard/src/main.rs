/**
 * @Author: Jack Jparrow
 * @Date: 2022-08-22 21:52:14
 * @LastEditTime: 2022-08-22 21:54:56
 * @LastEditors: Jack Jparrow
 * @Description: 可以加上 match 卫语句（guard） 来过滤分支 https://rustwiki.org/zh-CN/rust-by-example/flow_control/match/guard.html
 */

fn main() {
    let pair = (2, -2);

    println!("Tell me about {:?}", pair);

    match pair {
        (x, y) if x == y => println!("These are twins"),
        (x, y) if x + y == 0 => println!("The first one is odd"),
        _ => println!("No correlation..."),
    }
}
