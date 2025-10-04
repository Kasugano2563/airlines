use std::process::exit;
use rust_i18n::t;

use crate::{get_system_locale, main};

pub fn broken_save(month: i64, ingame: i64) {
    let locale = get_system_locale();
    rust_i18n::set_locale(&locale);   
    if month == 2 || !month == 3 || month == 5 || month == 6 || month == 8 || month == 9 || month == 11 || month ==12 {
        if ingame == 0 {
            eprintln!("{}", t!("broken_save_1"));
            main();
        } else {
            eprintln!("{}", t!("broken_save_2"));
            exit(1);
        }
    }
}