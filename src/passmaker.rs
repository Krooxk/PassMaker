use std::env;
use rand::prelude::*;
use rand::distributions::Alphanumeric;

fn create_pass(chars: String, nums: String, length: usize) {
    let mut rng = thread_rng();

    let password: String = (0..length).map(|_| {
        if rng.gen_bool(0.5) {
            chars.chars().choose(&mut rng).unwrap()
        } else {
            nums.chars().choose(&mut rng).unwrap()
        }
    }).collect();

    println!("\n[+] Password: {}", password);
}

fn main() {
    let mut chars = String::from("ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz");
    let mut nums = String::from("0123456789");

    let mut args = env::args();
    args.next();

    let mut length = 8;
    let mut lowercase = false;
    let mut uppercase = false;
    let mut symbols = false;
    let mut digits = false;

    while let Some(arg) = args.next() {
        match arg.as_str() {
            "-h" | "--help" => {
                println!("Usage: passmaker [OPTION]...");
                println!("Generates a random password with the specified options. If no options are specified, a default password will be generated.\n");
                println!("Options:");
                println!("  -l, --length <length>     specify the length of the password (default: 8)");
                println!("  -s, --lowercase          include lowercase letters in the password");
                println!("  -p, --uppercase          include uppercase letters in the password");
                println!("  -S, --symbols            include symbols in the password");
                println!("  -d, --digits             include digits in the password");
                println!("  -h, --help               show this help message and exit");
                return;
            },
            "-l" | "--length" => {
                length = args.next().unwrap().parse().unwrap();
            },
            "-s" | "--lowercase" => {
                lowercase = true;
            },
            "-p" | "--uppercase" => {
                uppercase = true;
            },
            "-S" | "--symbols" => {
                symbols = true;
            },
            "-d" | "--digits" => {
                digits = true;
            },
            _ => {
                eprintln!("Invalid argument: {}", arg);
                return;
            }
        }
    }

    if lowercase {
        chars = String::from("abcdefghijklmnopqrstuvwxyz");
    } else if uppercase {
        chars = String::from("ABCDEFGHIJKLMNOPQRSTVUWXYZ");
    }
    if symbols {
        chars.extend(Alphanumeric.sample_iter(&mut rand::thread_rng()));
    }
    if digits {
        nums = String::from("0123456789");
    }

    create_pass(chars, nums, length);
}
