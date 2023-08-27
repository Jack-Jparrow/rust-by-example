use List::*;

/**
 * @Author: Jack Jparrow
 * @Date: 2022-08-17 22:00:44
 * @LastEditTime: 2022-08-17 22:23:17
 * @LastEditors: Jack Jparrow
 * @Description: enum 的一个常见用法就是创建链表（linked-list） https://rustwiki.org/zh-CN/rust-by-example/custom_types/enum/testcase_linked_list.html
 */

enum List {
    // Cons：元组结构体，包含链表的一个元素和一个指向下一节点的指针
    Cons(u32, Box<List>),
    // Nil：末结点，表明链表结束
    Nil,
}

// 可以为 enum 定义方法
impl List {
    // 创建一个空的 List 实例
    fn new() -> List {
        // `Nil` 为 `List` 类型（译注：因 `Nil` 的完整名称是 `List::Nil`）
        Nil
    }

    // 处理一个 List，在其头部插入新元素，并返回该 List
    fn prepend(self, elem: u32) -> List {
        // `Cons` 同样为 List 类型
        Cons(elem, Box::new(self))
    }

    fn len(&self) -> u32 {
        match *self {
            // 必须对 `self` 进行匹配（match），因为这个方法的行为取决于 `self` 的取值种类
            // `self` 为 `&List` 类型，`*self` 为 `List` 类型，匹配一个具体的 `T` 类型要好过匹配引用 `&T`
            Cons(_, ref tail) => 1 + tail.len(),
            // （递归的）基准情形（base case）：一个长度为 0 的空列表
            Nil => 0
        }
    }
    
    // 返回列表的字符串表示（该字符串是堆分配的）
    fn stringify(&self) -> String {
        match *self {
            Cons(head, ref tail) => {
                // `format!` 和 `print!` 类似，但返回的是一个堆分配的字符串，而不是打印结果到控制台上
                format!("{}, {}", head, tail.stringify())
            },
            Nil => {
                format!("Nil")
            },
        }
    }
}

fn main() {
    // 创建一个空链表
    let mut list = List::new();

    // 追加一些元素
    list = list.prepend(1);
    list = list.prepend(2);
    list = list.prepend(3);

    // 显示链表的最后状态
    println!("linked list has length: {}", list.len());
    println!("{}", list.stringify());
}