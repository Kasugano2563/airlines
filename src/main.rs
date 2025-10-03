use std::io::{self, stdout, Write};

mod command; mod newgame; mod saveload;

fn main() {
    loop {
        print!("MENU $ ");
        stdout().flush().unwrap();
        let mut input: String = String::new();

        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                let command: &str = input.trim();

                if !command.is_empty() {
                    command::menu_command(command);
                }
            }
            Err(error) => {
                eprintln!("读取行错误: {}", error);
                continue;
            }
        }
    }
}
