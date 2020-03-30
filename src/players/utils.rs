use std::fmt;

pub enum PlayerStatus {
    Batting,
    Bowling
}

impl fmt::Display for PlayerStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

        let printable = match *self{
            PlayerStatus::Batting => "Batting",
            PlayerStatus::Bowling => "Bowling",
        };

        write!(f, "{}", printable)
    }
}