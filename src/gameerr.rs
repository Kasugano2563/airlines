use crate::main;

pub fn broken_save(month: i64) {
    if !month == 1 || month == 4 || month == 7 || month == 10 {
        eprintln!("该存档已损坏!");
        main();
    }
}