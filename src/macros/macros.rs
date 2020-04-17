#[macro_use]
mod macros{
    #[macro_export]
    macro_rules! start_game {
        ($a:expr,$b:expr, $c:ident) =>{
            match CricketGame::new($a, $b).start($c){
                Ok(()) => {
                    break;
                },
                Err(e) => {
                    panic!(e);
                }
            }
        }
    }
}
