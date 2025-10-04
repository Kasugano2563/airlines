use std::{io::{self, stdout, Write}, process::exit};
use crate::{game_loop, newgame, saveload::{game_load, game_save, Config}};

// 主菜单命令
pub fn menu_command(command: &str, state: &mut Config) {
    // 命令处理逻辑
    match command {
        "help" => {
            println!("Airlines 游戏, 版本 0.0.1a\n");
            println!("----命令列表----\n");
            println!("help  显示该列表");
            println!("new   新建游戏");
            println!("load  加载游戏");
            println!("exit  退出游戏");
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
            println!("未知命令: {}", command);
            println!("输入 'help' 查看可用命令");
        }
    }
}

// 游戏内命令
pub fn game_command(command: &str, state: &mut Config) {
    // 命令处理逻辑
    match command {
        "help" => {
            println!("Airlines 游戏, 版本 0.0.1a\n");
            println!("----命令列表----\n");
            println!("help  显示该列表");
            println!("info  显示游戏信息");
            println!("new   新建游戏");
            println!("load  加载游戏");
            println!("save  保存游戏");
            println!("next  进入下一回合");
            println!("exit  退出游戏");
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
            println!("未知命令: {}", command);
            println!("输入 'help' 查看可用命令");
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
                eprintln!("读取输入时发生错误: {}", error);
                return false;
            }
        }
    }
}