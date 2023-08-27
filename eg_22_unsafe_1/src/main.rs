/**
 * @Author: 白银
 * @Date: 2022-09-02 17:29:20
 * @LastEditTime: 2022-09-02 18:33:11
 * @LastEditors: 白银
 * @Description: 解引用一个裸指针只能通过不安全 代码块执行
 */

fn main() {
    let raw_p: *const u32 = &10;

    unsafe{
        assert!(*raw_p == 10);
    }
}