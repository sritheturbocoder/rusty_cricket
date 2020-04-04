use crate::errors::errorhandler::{ErrorCode, BASE_COIN_TOSS_ERROR_CODE};

use core::fmt;
use rand::Rng;

/// Error raised by Coin Toss
#[repr(C)]
#[derive(
    Debug,
    ::failure::Fail,
    Copy,
    Clone,
    serde::Serialize,
    serde::Deserialize,
    Eq,
    PartialEq,
    PartialOrd,
    Ord,
    Hash,
)]
pub enum CoinError {
    /// Failed to parse coin side (5001)
    #[fail(display = "Not a valid side")]
    InvalidSideError,
}

impl ErrorCode for CoinError {
    fn error_code(&self) -> i32 {
        BASE_COIN_TOSS_ERROR_CODE
            + match *self {
                CoinError::InvalidSideError => 1,
            }
    }
}


/// Enum representing a coin
#[derive(
    Clone, Copy, Debug, Eq, Hash, PartialEq, PartialOrd, Ord, serde::Serialize, serde::Deserialize,
)]
pub enum Coin {
    /// Head side of the coin
    Heads,
    /// Tails side of the coin
    Tails,
}

impl Coin {
    /// Flips a coin, returning a Coin with its current Side up
    pub fn flip() -> Coin {
        Coin::from(rand::thread_rng().gen::<bool>())
    }
}

impl fmt::Display for Coin {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Coin::Heads => f.write_str("Heads"),
            Coin::Tails => f.write_str("Tails"),
        }
    }
}

/// Coin toss struct
#[derive(
    Copy, Clone, Debug, serde::Serialize, serde::Deserialize, Ord, PartialOrd, PartialEq, Eq, Hash,
)]
pub struct CoinToss {
    /// The coin's state that the player guessed
    pub guess: Coin,
    /// The coin's state that happened after the toss
    pub real: Coin,
}

impl Into<bool> for CoinToss {
    fn into(self) -> bool {
        self.guess == self.real
    }
}

impl core::str::FromStr for Coin {
    type Err = crossterm::ErrorKind;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let guess = s.to_ascii_lowercase();
        if ["heads", "h", "t", "tails"].contains(&guess.as_str()) {
            Ok(Coin::from(guess.starts_with('h')))
        } 
        else
        {
            Err(crossterm::ErrorKind::__Nonexhaustive)
        }
    }
}

impl From<bool> for Coin {
    fn from(b: bool) -> Coin {
        if b {
            Coin::Heads
        } else {
            Coin::Tails
        }
    }
}

impl fmt::Display for CoinToss {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Guess: {}\nResult: {}", self.guess, self.real)
    }
}

impl CoinToss {
    /// Make a guess on what side the coin will land on
    pub fn guess(guess: Coin) -> Self {
        CoinToss {
            guess,
            real: Coin::flip(),
        }
    }
    /// Returns if the guess was correct
    pub fn is_correct(self) -> bool {
        self.real == self.guess
    }
}
