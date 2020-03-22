use std::io::{self, Write};
use std::fmt::Display;
use std::process;
use rand::Rng;

fn main() {
    println!("Fun with HAND CRICKET!");
    println!("Choose Odd / Even ? (O/E)");
    let toss : String = get_player_input()
            .unwrap_or_else(|e| exit_err(&e, e.raw_os_error().unwrap_or(-1)))
            .trim()
            .parse()
            .unwrap_or_else(|e| exit_err(&e, 2));
    
    println!("Enter your toss number ");
    let user_toss_numberstr :u16 = get_player_input()
            .unwrap_or_else(|e| exit_err(&e, e.raw_os_error().unwrap_or(-1)))
            .trim()
            .parse()
            .unwrap_or_else(|e| exit_err(&e, 2));
    let my_number : u16 = process_user_input(toss, user_toss_numberstr);
    println!("My number is {}", my_number);
}

fn get_player_input() -> io::Result<String> {
    let mut buf = String::new();
    match io::stdin().read_line(&mut buf) {
        Ok(n) => {
            println!("{} bytes read", n);
            println!("You enterted {} ", buf);
        },
        Err(error) => println!("error: {}", error),
    }

    Ok(buf)
}

fn process_user_input(toss : String, user_toss_number: u16) -> u16 {
    match toss.chars().next().unwrap()  {
        'o' | 'O' => {
            match user_toss_number % 2 {
                0 => panic!("You promised to enter odd..I am smart you can't cheat me !!"),
                _ => generate_even_number(), // user selected odd so I select some even
            }
        },
        'e' | 'E' => {
            match user_toss_number % 2 {
                0 => generate_odd_number(), // user selected even so I select some odd
                _ => panic!("You promised to enter even but you cheated !!"),
            }
        },
        _ => {
            panic!("Invalid input...please enter O or E")
        }
    }
}

fn generate_odd_number() -> u16 {
    
    let generated_odd_number :u16;

    loop{
        let odd_number : u16 = rand::thread_rng().gen_range(1, 10);
        match odd_number % 2 {
            0 => continue,
            1 => {
                   generated_odd_number = odd_number;
                   break;
            },
            _ => panic! ("Discovered something new in Mathematics")
        }
    }

    generated_odd_number
}

fn generate_even_number() -> u16 {
    
    let generated_even_number :u16;

    loop{
        let even_number : u16 = rand::thread_rng().gen_range(1, 10);
        match even_number % 2 {
            0 => {
                generated_even_number = even_number;
                break;
            },
            1 => continue,
            _ => panic! ("Discovered something new in Mathematics")
        }
    }

    generated_even_number
}

fn exit_err<T: Display>(msg: T, code: i32) -> !{
    let _ = writeln!(&mut io::stderr(), "Error: {}", msg);
    process::exit(code)
}