use std::error::Error;
use std::io::{self, Write};
use std::{process::exit, env};
use {rand, colored::Colorize};

// define a vcersion
const VERSION: &str = "1.1";

fn input() -> Result<String, Box<dyn Error>> {
    let mut i = String::new();
    io::stdin().read_line(&mut i)?;
    Ok(i)
}

fn args() -> Vec<String> {
    let args: Vec<String> = env::args().skip(1).collect();
    if args.is_empty() {
        eprintln!("ERROR: Программе не переданы аргументы! Как я могу узнать кто из вас гей? :(");
        eprintln!("Пример: {} {} {} ..", "gaynote".yellow(), "<Имя>".green(), "<Второе имя>".green());
        eprintln!("\nЧтобы узнать больше, напишите --help.");
        exit(1);
    }

    // find flags
    for flag in &args {
        match flag.as_str() {
            "--help" | "-h" => {
                println!("{}", "Справка:".yellow());
                println!("{}", "-----------".yellow());
                println!("\n{} - {}я, ОтечестВеннАя прОгрАммА, которая позволяет вычислить гея, который отправится на СВО следующим!", "gaynote".green(), "СВО".red());
                println!("Просто напишите имена пидорков через пробел.");
                println!("Чтобы узнать версию, используйте флаг '--version' или '-ZV'");
                exit(0);
            },

            "--version" | "-ZV" => {
                println!("Версия {}ей прОгрАммы - {}", "СВО".red(), VERSION.yellow());
                exit(0);
            },

            s if s.starts_with('-') | s.starts_with("--") => {
                eprintln!("ОШИБКА: Неизвестный флаг!");
                exit(1);
            },

            _ => {}
        };
    }
    args
}

fn rm_nlist(vec: &mut Vec<String>, index: usize) {
    vec.remove(index);
    for i in 0..vec.len() {
        println!("| {}: {}", (i + 1).to_string().yellow(), vec[i]);
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut gays = args();

    let r = rand::random_range(0..gays.len());
    println!("\n{}", "- Геи:".red());
    for i in 0..gays.len() {
        println!("| {}: {}", (i + 1).to_string().yellow(), gays[i]);
    }

    print!("Вы готовы? [Y/n]: ");
    io::stdout().flush()?;

    let sex = input()?.trim().parse::<char>();
    let sex = match sex {
        Ok(sex) => sex,
        Err(_) => 'y'
    };

    if let 'y' = sex.to_ascii_lowercase() {
        println!("{}", "-------------------------".yellow());
        println!("\nГей: {}{}", gays[r].green(), "✅".green());
        println!("\n{}", "-------------------------".yellow());
        println!("{}", "- Натуралы:".red());

        // list straights
        rm_nlist(&mut gays, r);
    } else {
        eprintln!("лох ебаный");
        exit(1);
    }

    Ok(())
}
