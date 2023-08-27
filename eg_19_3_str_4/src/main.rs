use std::str;

/**
 * @Author: 白银
 * @Date: 2022-09-01 21:29:16
 * @LastEditTime: 2022-09-01 21:40:47
 * @LastEditors: 白银
 * @Description: 想要非 UTF-8 字符串（记住，&str 和 String 都必须是合法的 UTF-8 序列），或者 需要一个字节数组，其中大部分是文本？请使用字节串（byte string） https://rustwiki.org/zh-CN/rust-by-example/std/str.html
 */

fn main() {
    // 注意这并不是一个 &str
    let bytestring: &[u8; 20] = b"this is a bytestring";
    // 字节串没有实现 Display，所以它们的打印功能有些受限
    println!("A bytestring: {:?}", bytestring);

    // 字节串可以使用单字节的转义字符...
    let escaped = b"\x52\x75\x73\x74 as bytes";
    // ...但不能使用 Unicode 转义字符
    // let escaped = b"\u{211D} is not allowed";
    println!("Some escaped bytes: {:?}", escaped);

    // 原始字节串和原始字符串的写法一样
    let raw_bytestring = br"\u{211D} is not escaped here";
    println!("{:?}", raw_bytestring);

    // 把字节串转换为 &str 可能失败
    if let Ok(my_str) = str::from_utf8(raw_bytestring) {
        println!("And the same as text: '{}'", my_str);
    }

    let quotes = br#"You can also use "fancier" formatting, \
                                like with normal raw strings"#;

    // 字节串可以不使用 UTF-8 编码
    let shift_jis = b"\x82\xe6\x82\xa8\x82\xb1\x82"; // SHIFT-JIS 编码的 "ようこそ"

    // 但这样的话它们就无法转换成 &str 了
    match str::from_utf8(shift_jis) {
        Ok(my_str) => println!("Conversion successful: '{}'", my_str),
        Err(e) => println!("Conversion failed: {:?}", e),
    }
}
