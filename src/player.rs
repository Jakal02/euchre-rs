use std::fmt;
use crate::card::Card;
use crate::state::ScopedGameState;

pub trait Strategy {
    fn new(name: &'static str) -> Self;

    fn step(&self, state: &ScopedGameState) -> ();
}

pub struct Human {
    name: String,
}

pub struct Random {
    name: String,
}

impl fmt::Display for Human {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl fmt::Display for Random {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}


pub struct Player {
    pub hand: Vec<Card>,
}

impl fmt::Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // write!(f, "{} Player: {{ ", self.strategy).ok();
        write!(f, "{{").ok();
        for card in &self.hand {
            write!(f, "{}, ", card).ok();
        }
        write!(f, "}}")
    }
}