use std::io::{self, Write};
use std::fmt::Display;
use std::process;

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
    check_for_user_input(toss, user_toss_numberstr);
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

fn check_for_user_input(toss : String, user_toss_number: u16) {
    match toss.chars().next().unwrap()  {
        'o' | 'O' => {
            match user_toss_number % 2 {
                0 => panic!("You promised to enter odd..I am smart you can't cheat me !!"),
                _ => println!("Thanks, Now let me show my number")
            }
        },
        'e' | 'E' => {
            match user_toss_number % 2 {
                0 => println!("Thanks, Now let me show my number"),
                _ => panic!("You promised to enter even but you cheated !!")
            }
        },
        _ => {
            panic!("Invalid input...please enter O or E")
        }
    }
}

fn exit_err<T: Display>(msg: T, code: i32) -> !{
    let _ = writeln!(&mut io::stderr(), "Error: {}", msg);
    process::exit(code)
}