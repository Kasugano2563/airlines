use std::io::{self, stdout, Write};
use crate::saveload::Config;

mod command; mod newgame; mod saveload; mod gameerr;

// 主菜单
fn main() {
    let mut game_state: Config = saveload::Config {
        company_name: String::from("Airlines 主菜单"),
        money: 0,
        years: 0,
        month: 0,
    };

    loop {
        print!("{} $ ", game_state.company_name);
        stdout().flush().unwrap();
        let mut input: String = String::new();

        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                let command: &str = input.trim();

                if !command.is_empty() {
                    command::menu_command(command, &mut game_state);
                }
            }
            Err(error) => {
                eprintln!("读取行错误: {}", error);
                continue;
            }
        }
    }
}

// 游戏循环
fn game_loop(state: &mut Config) {
    loop {
        print!("{} $ ", state.company_name);
        stdout().flush().unwrap();
        let mut input: String = String::new();

        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                let command: &str = input.trim();

                if !command.is_empty() {
                    command::game_command(command, state);
                }
            }
            Err(error) => {
                eprintln!("读取行错误: {}", error);
                continue;
            }
        }
    }
}