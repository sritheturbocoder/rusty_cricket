use std::io::{self, Write};
use std::fmt::Display;
use std::process;

fn main() {
    println!("Fun with HAND CRICKET!");
    let mut toss = String::new();
    println!("Choose Odd / Even ? (O/E)");
    match io::stdin().read_line(&mut toss) {
        Ok(n) => {
            println!("{} bytes read", n);
            println!("You enterted {} ", toss);
        },
        Err(error) => println!("error: {}", error),
    }
    
    let mut user_toss_numberstr = String::new();
    match toss.chars().next().unwrap()  {
        'o' | 'O' => {
            println!("You choose Odd so I will take Even !");
            println!("Let's toss now !!!");
    
        },
        'e' | 'E' => {
            println!("You choose Even so I will take Odd !");
            println!("Let's toss now !!!");
        },
        _ => {
            panic!("Invalid input...please enter O or E")
        }
    }

    
    match io::stdin().read_line(&mut user_toss_numberstr) {
        Ok(n) => {
            println!("{} bytes read", n);
            println!("You entered {}", user_toss_numberstr);
        },
        Err(error) => println!("error: {}", error),
    }

    let user_entered_number : u32 = match user_toss_numberstr.trim().parse(){
        Ok(num) => num,
        Err(err) => {
            panic!("Invalid input please input a valid number. Your input resulted in {}", err)
        }
    };

    match toss.chars().next().unwrap()  {
        'o' | 'O' => {
            match user_entered_number % 2 {
                0 => panic!("You promised to enter odd but you cheated !!"),
                _ => println!("Thanks, Now let me show my number")
            }
        },
        'e' | 'E' => {
            match user_entered_number % 2 {
                0 => println!("Thanks, Now let me show my number"),
                _ => panic!("You promised to enter even but you cheated !!")
            }
        },
        _ => {
            panic!("Invalid input...please enter O or E")
        }
    }
}

fn GetPlayerInput(msg : &str) -> io::Result<String> {
    let mut buf = String::new();
    print!("{}", msg);
    //let mut file = try!(io::stdout().flush());
    io::stdout().flush().ok().expect("Could not flush stdout");
    io::stdin().read_line(&mut buf).ok().expect("Could not read line");

    match io::stdin().read_line(&mut buf) {
        Ok(n) => {
            println!("{} bytes read", n);
            println!("You enterted {} ", buf);
        },
        Err(error) => println!("error: {}", error),
    }

    Ok(buf)
}