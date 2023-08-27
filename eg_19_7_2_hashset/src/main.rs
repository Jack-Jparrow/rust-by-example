use std::collections::HashSet;

/**
 * @Author: 白银
 * @Date: 2022-09-02 14:16:27
 * @LastEditTime: 2022-09-02 14:33:34
 * @LastEditors: 白银
 * @Description: 集合（set）拥有 4 种基本操作 https://rustwiki.org/zh-CN/rust-by-example/std/hash/hashset.html
 */

fn main() {
    let mut a: HashSet<i32> = vec![1i32, 2, 3].into_iter().collect();
    let mut b: HashSet<i32> = vec![2i32, 3, 4].into_iter().collect();

    assert!(a.insert(4));
    assert!(a.contains(&4));
    // 如果值已经存在，那么 `HashSet::insert()` 返回 false
    // assert!(b.insert(4), "Value 4 is already in set B!");
    b.insert(5);

    // 若一个集合（collection）的元素类型实现了 `Debug`，那么该集合也就实现了 `Debug`
    // 这通常将元素打印成这样的格式 `[elem1, elem2, ...]
    println!("A: {:?}", a);
    println!("B: {:?}", b);

    // 乱序打印 [1, 2, 3, 4, 5]。
    println!("Union: {:?}", a.union(&b).collect::<Vec<&i32>>());

    // 这将会打印出 [1]
    println!("Difference: {:?}", a.difference(&b).collect::<Vec<&i32>>());

    // 乱序打印 [2, 3, 4]。
    println!(
        "Intersection: {:?}",
        a.intersection(&b).collect::<Vec<&i32>>()
    );

    // 打印 [1, 5]
    println!(
        "Symmetric Difference: {:?}",
        a.symmetric_difference(&b).collect::<Vec<&i32>>()
    );
}
