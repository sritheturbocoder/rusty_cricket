mod utils;

pub struct Player {
    toss_selection : utils::TossSelection,
    wontoss :bool,
    runs :u16,
    wickets :u16,
    overs :u16,
}

impl Player {
    pub choose_toss() -> utils::TossSelection {
    let toss : String = get_player_input()
            .unwrap_or_else(|e| utils::exit_err(&e, e.raw_os_error().unwrap_or(-1)))
            .trim()
            .parse()
            .unwrap_or_else(|e| utils::exit_err(&e, 2));
    
    println!("Enter your toss number ");
    let user_toss_number :u16 = get_player_input()
            .unwrap_or_else(|e| utils::exit_err(&e, e.raw_os_error().unwrap_or(-1)))
            .trim()
            .parse()
            .unwrap_or_else(|e| utils::exit_err(&e, 2));
    }
}