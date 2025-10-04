use std::{env, io::{self, stdout, Write}};
use rust_i18n::t;
use crate::saveload::Config;

mod command; mod newgame; mod saveload; mod gameerr;

rust_i18n::i18n!("locales", fallback = "en");

pub fn get_system_locale() -> String {
    let lang = env::var("LANG").unwrap_or_else(|_| "en".to_string()); 
    lang.split('.') .next().unwrap_or("en").to_string()
}

// 主菜单
fn main() {
    let locale = get_system_locale();
    rust_i18n::set_locale(&locale);
    
    let mut game_state: Config = saveload::Config {
        company_name: String::from("Airlines Menu"),
        money: 0,
        years: 0,
        month: 0,
    };

    loop {
        print!("{} $ ", t!("menu"));
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
                eprintln!("{}: {}", t!("error_reading_row"), error);
                continue;
            }
        }
    }
}

// 游戏循环
fn game_loop(state: &mut Config) {
    let locale = get_system_locale();
    rust_i18n::set_locale(&locale);

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
                eprintln!("{}: {}", t!("error_reading_row"), error);
                continue;
            }
        }
    }
}