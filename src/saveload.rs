use std::{self, fs::File, io::{self, stdout, BufReader, Write}};
use serde::{Deserialize, Serialize};
use crate::gameerr;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Config {
    pub company_name: String,
    pub money: i64,
    pub years: i64,
    pub month: i64,
}

// 导入/导出文件内部逻辑
fn export_save(config: &Config, filename: &str, ingame: i64) -> Result<(), Box<dyn std::error::Error>> {
    gameerr::broken_save(config.month, ingame);
    let yaml_string: String = serde_yaml::to_string(config)?;

    let mut file = File::create(filename)?;
    file.write_all(yaml_string.as_bytes())?;

    println!("存档 {} 已保存", filename);
    Ok(())
}

fn load_save(filename: &str, ingame: i64) -> Result<Config, Box<dyn std::error::Error>> {
    let filename = format!("{}.yaml", filename);
    let file = File::open(&filename)?;
    let reader = BufReader::new(file);
    let config: Config = serde_yaml::from_reader(reader)?;

    println!("存档 {} 已加载, 正在校验存档", filename);
    gameerr::broken_save(config.month, ingame);
    Ok(config)
}

// 导入/导出文件前端逻辑
pub fn game_save(config: &Config, ingame: i64) {
    let save_name: String;
    
    loop {
        println!("保存存档");
        print!("请输入存档文件名称（.yaml）：");
        stdout().flush().unwrap();

        let mut save_input: String = String::new();
        match io::stdin().read_line(&mut save_input) {
            Ok(_) => {
                let trimmed_save_name: &str = save_input.trim();
                if trimmed_save_name.is_empty() {
                    println!("存档文件名称不能为空");
                } else {
                    save_name = trimmed_save_name.to_string();
                    break;
                }
            }
            Err(error) => {
                eprintln!("读取行错误: {}", error);
                continue;
            }
        }
    }
    
    let filename: String = format!("{}.yaml", save_name);
    if let Err(e) = export_save(config, &filename, ingame) {
        eprintln!("导出错误: {}", e);
    }
}

pub fn game_load(ingame: i64) -> Option<Config> {
    loop {
        println!("加载存档");
        print!("请输入存档文件名称(.yaml）：");
        stdout().flush().unwrap();

        let mut load_input: String = String::new();
        match io::stdin().read_line(&mut load_input) {
            Ok(_) => {
                let save_name = load_input.trim();
                if save_name.is_empty() {
                    println!("存档文件名称不能为空");
                } else {
                    match load_save(save_name, ingame) {
                        Ok(config) => {
                            println!("加载成功，公司: {}，年份: {}.{}，金钱：{}", config.company_name, config.years, config.month, config.money);
                            return Some(config);
                        }
                        Err(e) => {
                            eprintln!("加载错误: {}", e);
                        }
                    }
                }
            }
            Err(error) => {
                eprintln!("读取输入时发生错误: {}", error);
                continue;
            }
        }
    }
}