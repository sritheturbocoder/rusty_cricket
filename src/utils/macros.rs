#[macro_export]
macro_rules! start_game {
    ($a:expr,$b:expr, $c:ident) => {
            match CricketGame::new($a, $b).start($c) {
                    Ok(()) => {
                            break;
                    }
                    Err(e) => {
                            panic!(e);
                    }
            }
    };
}

#[macro_export]
macro_rules! toss {
    ($a:expr, $w:ident) => {
            queue!($w, style::Print("You choose "))?;
            queue!($w, style::Print($a), cursor::MoveToNextLine(1))?;
            $w.flush()?;
            if CoinToss::guess($a.parse()?).is_correct() {
                    for line in TOSSCHOICE.split('\n') {
                            queue!($w, style::Print(line), cursor::MoveToNextLine(1))?;
                    }
                    $w.flush()?;

                    let toss_decision = read()?;

                    if toss_decision == Event::Key(KeyCode::Char('1').into()) {
                            start_game!(TossWonBy::Human, PlayerStatus::Batting, $w);
                    } else if toss_decision == Event::Key(KeyCode::Char('2').into()) {
                            start_game!(TossWonBy::Human, PlayerStatus::Bowling, $w);
                    } else if toss_decision == Event::Key(KeyCode::Char('q').into()) {
                            break;
                    } else {
                            queue!(
                                    $w,
                                    style::Print("Invaid input, Choose h or t"),
                                    cursor::MoveToNextLine(1)
                            )?;
                            $w.flush()?;
                            continue;
                    }
            } else {
                    queue!($w, style::Print("You lost toss"), cursor::MoveToNextLine(1))?;
                    $w.flush()?;
                    match rand::thread_rng().gen::<bool>() {
                            true => {
                                    start_game!(TossWonBy::Genie, PlayerStatus::Batting, $w);
                            }
                            _ => {
                                    start_game!(TossWonBy::Genie, PlayerStatus::Bowling, $w);
                            }
                    }
            }
    };
}