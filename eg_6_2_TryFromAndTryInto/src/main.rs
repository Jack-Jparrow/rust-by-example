/**
 * @Author: Jack Jparrow
 * @Date: 2022-08-20 21:32:56
 * @LastEditTime: 2022-08-20 21:40:17
 * @LastEditors: Jack Jparrow
 * @Description: TryFrom 和 TryInto trait 用于易出错的转换，也正因如此，其返回值是 Result 型 https://rustwiki.org/zh-CN/rust-by-example/conversion/try_from_try_into.html
 */

#[derive(Debug, PartialEq)]
struct EveNumber(i32);

impl TryFrom<i32> for EveNumber {
    type Error = ();

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        if value % 2 == 0 {
            Ok(EveNumber(value))
        } else {
            Err(())
        }
    }
}

fn main() {
    // TryFrom
    assert_eq!(EveNumber::try_from(8), Ok(EveNumber(8)));
    assert_eq!(EveNumber::try_from(5), Err(()));

    // TryInto
    let result: Result<EveNumber, ()> = 8i32.try_into();
    assert_eq!(result, Ok(EveNumber(8)));
    let result: Result<EveNumber, ()> = 5i32.try_into();
    assert_eq!(result, Err(()));
}
