/**
 * @Author: 白银
 * @Date: 2022-09-01 20:54:33
 * @LastEditTime: 2022-09-02 13:53:20
 * @LastEditors: 白银
 * @Description: Result 枚举类型强调为什么一个操作会失败 https://rustwiki.org/zh-CN/rust-by-example/std/result.html
 */

mod checked {
    // 我们想要捕获的数学 “错误”
    #[derive(Debug)]
    pub enum MathError {
        DivisionByZero,
        NegativeLogarithm,
        NegativeSquareRoot,
    }

    pub type MathResult = Result<f64, MathError>;

    pub fn div(x: f64, y: f64) -> MathResult {
        if y == 0.0 {
            // 此操作将会失败，那么（与其让程序崩溃）不如把失败的原因包装在 `Err` 中并返回
            Err(MathError::DivisionByZero)
        } else {
            // 此操作是有效的，返回包装在 `Ok` 中的结果
            Ok(x / y)
        }
    }

    pub fn sqrt(x: f64) -> MathResult {
        if x < 0.0 {
            Err(MathError::NegativeSquareRoot)
        } else {
            Ok(x.sqrt())
        }
    }

    pub fn ln(x: f64) -> MathResult {
        if x < 0.0 {
            Err(MathError::NegativeLogarithm)
        } else {
            Ok(x.ln())
        }
    }
}

// `op(x, y)` === `sqrt(ln(x / y))`
fn op(x: f64, y: f64) -> f64 {
    // 这是一个三层的 match 金字塔！
    match checked::div(x, y) {
        Err(why) => panic!("{:?}", why),
        Ok(ratio) => match checked::ln(ratio) {
            Err(why) => panic!("{:?}", why),
            Ok(ln) => match checked::sqrt(ln) {
                Err(why) => panic!("{:?}", why),
                Ok(sqrt) => sqrt,
            },
        },
    }
}

fn main() {
    println!("{}", op(1.0, 10.0));
}
