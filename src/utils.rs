use std::io::{self,Write};
use std::fmt::Display;
use std::process;
use rand::Rng;

pub fn generate_odd_number() -> u16 {
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
    
pub fn generate_even_number() -> u16 {
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
    
pub fn generate_toss() -> u16 {
    rand::thread_rng().gen_range(0, 1)
}

pub fn exit_err<T: Display>(msg: T, code: i32) -> !{
    let _ = writeln!(&mut io::stderr(), "Error: {}", msg);
    process::exit(code)
}

