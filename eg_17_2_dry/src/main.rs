use std::ops::{Add, Mul, Sub};

/**
 * @Author: 白银
 * @Date: 2022-09-01 14:00:49
 * @LastEditTime: 2022-09-01 14:19:26
 * @LastEditors: 白银
 * @Description: 通过提取函数或测试集的公共部分，宏可以让你写出 DRY 的代码（DRY 是 Don't Repeat Yourself 的缩写，意思为 “不要写重复代码”） https://rustwiki.org/zh-CN/rust-by-example/macros/dry.html
 */

macro_rules! assert_equal_len {
    // `tt`（token tree，标记树）指示符表示运算符和标记
    ($a: ident, $b: ident, $func: ident, $op: tt) => {
        assert!(
            $a.len() == $b.len(),
            "{:?}: dimension mismatch: {:?} {:?} {:?}",
            stringify!($func),
            ($a.len(),),
            stringify!($op),
            ($b.len(),)
        );
    };
}

macro_rules! op {
    ($func: ident, $bound: ident, $op: tt, $method: ident) => {
        fn $func<T: $bound<T, Output = T> + Copy>(xs: &mut Vec<T>, ys: &Vec<T>) {
            assert_equal_len!(xs, ys, $func, $op);

            for (x, y) in xs.iter_mut().zip(ys.iter()) {
                *x = $bound::$method(*x, *y);
                // *x = x.$method(*y);
            }
        }
    };
}

// 实现 `add_assign`、`mul_assign` 和 `sub_assign` 等函数
op!(add_assign, Add, +=, add);
op!(mul_assign, Mul, *=, mul);
op!(sub_assign, Sub, -=, sub);

mod test {
    use std::iter;

    macro_rules! test {
        ($func: ident, $x: expr, $y: expr, $z: expr) => {
            #[test]

            fn $func() {
                for size in 0usize..10 {
                    let mut x: Vec<_> = iter::repeat($x).take(size).collect();
                    let y: Vec<_> = iter::repeat($y).take(size).collect();
                    let z: Vec<_> = iter::repeat($z).take(size).collect();

                    super::$func(&mut x, &y);

                    assert_eq!(x, y);
                }
            }
        };
    }

    // 测试 `add_assign`、`mul_assign` 和 `sub_assign`
    test!(add_assign, 1u32, 2u32, 3u32);
    test!(mul_assign, 2u32, 3u32, 6u32);
    test!(sub_assign, 3u32, 2u32, 1u32);
}
