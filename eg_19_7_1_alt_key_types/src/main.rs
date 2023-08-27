use std::collections::HashMap;

/**
 * @Author: 白银
 * @Date: 2022-09-02 14:09:53
 * @LastEditTime: 2022-09-02 14:26:09
 * @LastEditors: 白银
 * @Description: 为了试验 HashMap 中的 struct，让我们试着做一个非常简易的用户登录系统 https://rustwiki.org/zh-CN/rust-by-example/std/hash/alt_key_types.html
 */

// Eq 要求你对此类型推导 PartiaEq
#[derive(Eq, Hash, PartialEq)]
struct Account<'a> {
    username: &'a str,
    password: &'a str,
}

struct AccountInfo<'a> {
    name: &'a str,
    email: &'a str,
}

type Accounts<'a> = HashMap<Account<'a>, AccountInfo<'a>>;

fn try_logon<'a>(accounts: &Accounts<'a>, username: &'a str, password: &'a str) {
    println!("Username: {}", username);
    println!("Password: {}", password);
    println!("Attempting logon...");

    let logon = Account {
        username: username,
        password: password,
    };

    match accounts.get(&logon) {
        Some(account_info) => {
            println!("Successful logon!");
            println!("Name: {}", account_info.name);
            println!("Email: {}", account_info.email);
        }
        _ => println!("Login failed!"),
    }
}

fn main() {
    let mut accounts: Accounts = HashMap::new();
    let account = Account {
        username: "j.everyman",
        password: "password123",
    };
    let account_info = AccountInfo {
        name: "John Everyman",
        email: "j.everyman@email.com",
    };

    accounts.insert(account, account_info);

    try_logon(&accounts, "j.everyman", "psasword123");
    try_logon(&accounts, "j.everyman", "password123");
}
