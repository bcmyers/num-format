use std::env;
use std::process::exit;

use num_format_system::SystemLocale;

fn main() {
    let mut args = env::args();
    match args.len() {
        0 => unreachable!(),
        1 => {
            let mut names = SystemLocale::available_names()
                .unwrap()
                .into_iter()
                .collect::<Vec<String>>();
            names.sort();
            for name in &names {
                println!("{}", name);
            }
        }
        2 => {
            let arg = args.nth(1).unwrap();
            if &arg == "all" {
                let mut names = SystemLocale::available_names()
                    .unwrap()
                    .into_iter()
                    .collect::<Vec<String>>();
                names.sort();
                for name in &names {
                    let locale = SystemLocale::from_name(name.to_string()).unwrap();
                    println!("{:?}", locale);
                }
            } else {
                let locale = SystemLocale::from_name(arg).unwrap();
                println!("{:?}", &locale);

            }
        }
        _ => {
            eprintln!("USAGE: test [locale name]");
            exit(1);
        }
    }
}
