use std::process::exit;
use crate::newgame;
// use crate::saveload;

pub fn menu_command(command: &str) {
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
            newgame::newgame();
            }
        "load" => {
            //if let Some(new_state) = saveload::game_load() {
            //    *state = new_state.clone();
            //}
            println!("没做");
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