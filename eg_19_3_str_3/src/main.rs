/**
 * @Author: 白银
 * @Date: 2022-09-01 21:11:55
 * @LastEditTime: 2022-09-01 21:32:17
 * @LastEditors: 白银
 * @Description: 有时会有太多需要转义的字符，或者是直接原样写出会更便利。这时可以使用原始字符 串（raw string） https://rustwiki.org/zh-CN/rust-by-example/std/str.html
 */

fn main() {
    let raw_str = r"Escapes don't work here: \x3F \u{211D}";
    println!("{}", raw_str);

    // 如果你要在原始字符串中写引号，请在两边加一对 #
    let quotes = r#"And then I said: "There is no escape!""#;
    println!("{}", quotes);

    // 如果字符串中需要写 "#，那就在定界符中使用更多的 #
    // 可使用的 # 的数目没有限制
    let longer_delimiter = r###"A string with "# in it. And even "##!"###;
    println!("{}", longer_delimiter);
}
