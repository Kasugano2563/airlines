use std::process::exit;
use crate::main;

pub fn broken_save(month: i64, ingame: i64) {
    if month == 2 || !month == 3 || month == 5 || month == 6 || month == 8 || month == 9 || month == 11 || month ==12 {
        if ingame == 0 {
            eprintln!("该存档文件已损坏! 无法加载!");
            main();
        } else {
            eprintln!("该游戏状态已损坏! 无法保存!");
            exit(1);
        }
    }
}