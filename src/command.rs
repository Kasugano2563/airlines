use std::{io::{self, stdout, Write}, process::exit};
use rust_i18n::t;

use crate::{game_loop, get_system_locale, newgame, saveload::{game_load, game_save, Config}};

rust_i18n::i18n!("locales", fallback = "en");

// Menu commmand
pub fn menu_command(command: &str, state: &mut Config) {
    // Set locale
    let locale = get_system_locale();
    rust_i18n::set_locale(&locale);

    // Command processing
    match command {
        "help" => {
            println!("{}", t!("game_info"));
            println!("{}", t!("command_list_title"));
            println!("{}", t!("help_help"));
            println!("{}", t!("help_new"));
            println!("{}", t!("help_load"));
            println!("{}", t!("help_exit"));
        }
        "new" => {
            let ingame: i64 = 0;
            let new_state = newgame::newgame(ingame);
            *state = new_state;
            game_loop(state);
            }
        "load" => {
            let ingame: i64 = 0;
            if let Some(new_state) = game_load(ingame) {
                *state = new_state;
                game_loop(state);
            }
        }
        "exit" => {
            exit(0);
        }
        _ => {
            println!("{}: {}", t!("unknown_command_1"), command);
            println!("{}", t!("unknown_command_2"));
        }
    }
}

// In game command
pub fn game_command(command: &str, state: &mut Config) {
    // Set locale
    let locale = get_system_locale();
    rust_i18n::set_locale(&locale);
    
    // Command processing
    match command {
        "help" => {
            println!("{}", t!("game_info"));
            println!("{}", t!("command_list_title"));
            println!("{}", t!("help_help"));
            println!("{}", t!("help_info"));
            println!("{}", t!("help_new"));
            println!("{}", t!("help_load"));
            println!("{}", t!("help_save"));
            println!("{}", t!("help_next"));
            println!("{}", t!("help_exit"));
        }
        "info" => {
            println!("现在是 {}.{} , 金钱 {}", state.years, state.month, state.money);
        }
        "new" => {
            if confirm("确认要开始新游戏吗? 未保存的数据将会丢失") {
                let ingame: i64 = 1;
                let new_state = newgame::newgame(ingame);
                *state = new_state;
                game_loop(state);
            }
        }
        "load" => {
            if confirm("确认要加载其他存档吗? 未保存的数据将会丢失") {
                let ingame: i64 = 1;
                if let Some(new_state) = game_load(ingame) {
                    *state = new_state;
                    game_loop(state);
                }
            }
        }
        "save" => {
            let ingame: i64 = 1;
            game_save(&state, ingame);
        }
        "next" => {
            if confirm("确认要进入下一回合吗?") {
                state.month += 3;
                if state.month > 12 {
                    state.years += 1;
                    state.month = 1;
                }
                println!("现在是 {}.{} , 金钱 {}", state.years, state.month, state.money);
            }
        }
        "exit" => {
            exit(0);
        }
        _ => {
            println!("{}: {}", t!("unknown_command_1"), command);
            println!("{}", t!("unknown_command_2"));
        }
    }
}

fn confirm(prompt: &str) -> bool {
    loop {
        print!("{} (y/n): ", prompt);
        stdout().flush().unwrap();

        let mut ori_input = String::new();
        match io::stdin().read_line(&mut ori_input) {
            Ok(_) => {
                let input = ori_input.trim().to_lowercase();
                if input == "y" || input == "yes" {
                    return true;
                } else if input == "n" || input == "no" {
                    println!("操作已取消");
                    return false;
                } else {
                    println!("输入无效, 请输入 'y' 或 'n'");
                }
            }
            Err(error) => {
                eprintln!("{}: {}", t!("error_reading_row"), error);
                return false;
            }
        }
    }
}