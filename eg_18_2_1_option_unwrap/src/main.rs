/**
 * @Author: 白银
 * @Date: 2022-09-01 14:34:52
 * @LastEditTime: 2022-09-01 15:19:27
 * @LastEditors: 白银
 * @Description: 可以将多个 ? 链接在一起，以使代码更具可读性 https://rustwiki.org/zh-CN/rust-by-example/error/option_unwrap/question_mark.html#%E4%BD%BF%E7%94%A8--%E8%A7%A3%E5%BC%80-option
 */

struct Person {
    job: Option<Job>,
}
#[derive(Clone, Copy)]
struct Job {
    phone_number: Option<PhoneNumber>,
}
#[derive(Clone, Copy)]
struct PhoneNumber {
    area_code: Option<u8>,
    number: u32,
}

impl Person {
    // 获取此人的工作电话号码的区号（如果存在的话）
    fn work_phone_area_code(&self) -> Option<u8> {
        // 没有`？`运算符的话，这将需要很多的嵌套的 `match` 语句
        self.job?.phone_number?.area_code
    }
}

fn main() {
    let p = Person {
        job: Some(Job {
            phone_number: Some(PhoneNumber {
                area_code: Some(61),
                number: 439222222,
            }),
        }),
    };

    assert_eq!(p.work_phone_area_code(), Some(61));
}
