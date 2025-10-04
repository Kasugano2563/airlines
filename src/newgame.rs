use std::io::{self, stdout, Write};
use crate::saveload::{self, Config};

pub fn newgame(ingame: i64) -> Config {
    let mut company_name: String;
    let years: i64;
    let month: i64 = 1;
    let money: i64 = 1000;
    
    loop {
        print!("请输入新航空公司名称: ");
        stdout().flush().unwrap();
        let mut ori_company_name: String = String::new();
        match io::stdin().read_line(&mut ori_company_name) {
            Ok(_) => {
                company_name = ori_company_name.trim().to_string();
                if company_name.is_empty() {
                    println!("航空公司名称不能为空");
                } else {
                    println!("您的航空公司名称为: {}", company_name);
                    break;
                }
            }
            Err(error) => {
                eprintln!("读取行错误: {}", error);
            }
        }
    }

    loop {
        print!("请输入游戏开始年份: ");
        stdout().flush().unwrap();
        let mut ori_years: String = String::new();
        match io::stdin().read_line(&mut ori_years) {
            Ok(_) => {
                match ori_years.trim().parse::<i64>() {
                    Ok(y) => {
                        years = y;
                        println!("您的游戏开始年份为: {}", years);
                        break;
                    }
                    Err(_) => {
                        eprintln!("必须输入数字");
                    }
                }
            }
            Err(error) => {
                eprintln!("读取行错误: {}", error);
            }
        }
    }

    let state = Config {
        company_name: company_name.clone(),
        years,
        money,
        month,
    };
    
    saveload::game_save(&state, ingame);
    state
}
