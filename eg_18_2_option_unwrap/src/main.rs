/**
 * @Author: 白银
 * @Date: 2022-09-01 14:34:52
 * @LastEditTime: 2022-09-01 15:29:07
 * @LastEditors: 白银
 * @Description: 显式处理将举出更受控制的 结果，同时如果需要的话，仍然可以使程序 panic https://rustwiki.org/zh-CN/rust-by-example/error/option_unwrap.html
 */

// 平民（commoner）们见多识广，收到什么礼物都能应对
// 所有礼物都显式地使用 `match` 来处理
fn given_commoner(gift: Option<&str>) {
    match gift {
        // 指出每种情况下的做法
        Some("snake") => println!("Yuck! I'm throwing that snake in a fire."),
        Some(inner) => println!("{}? How nice.", inner),
        None => println!("No gift? Oh well."),
    }
}

// 养在深闺人未识的公主见到蛇就会 `panic`（恐慌）
// 这里所有的礼物都使用 `unwrap` 隐式地处理
fn given_princess(gift: Option<&str>) {
    let inside = gift.unwrap();

    if inside == "snake" {
        panic!("AAAaaaaa!!!!");
    }

    println!("I love {}s!!!!!", inside);
}

fn main() {
    let food = Some("chicken");
    let snake = Some("snake");
    let void = None;

    given_commoner(food);
    given_commoner(snake);
    given_commoner(void);

    let bird = Some("robin");
    let nothing = None;

    given_commoner(bird);
    given_princess(nothing);
}
