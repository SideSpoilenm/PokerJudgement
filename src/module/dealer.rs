use std::io::{BufReader, BufRead};
use std::fs::File;
use super::card::Card;
use super::super::range;

#[derive(Clone, Copy, PartialEq, Debug)]
#[allow(unused)]
pub enum Street {
    Preflop,
    Flop,
    Turn,
    River,
}

#[derive(Clone, Copy, PartialEq, Debug)]
#[allow(unused)]
pub enum Position {
    UTG,
    EP1,
    EP2,
    MP1,
    MP2,
    HJ,
    CO,
    BTN,
    SB,
    BB,
}

#[derive(Clone, Copy, PartialEq, Debug)]
#[allow(unused)]
pub enum Action {
    Fold,
    Check,
    Call,
    Raise,
    ThreeBet,
    FourBet,
}

#[allow(unused)]
pub struct Play {
    pub street: Street,
    pub position: Position,
    pub action: Action,
}

impl Play {
    pub fn range(&self) -> Vec<Vec<Card>> {
        let mut path:String = "src\\range\\".to_string();
        path += match self.position{
            Position::UTG => "UTG",
            Position::EP1 | Position::EP2 => "EP",
            Position::MP1 | Position::MP2 => "MP",
            Position::HJ => "HJ",
            Position::CO => "CO",
            Position::BTN => "BTN",
            Position::SB => "SB",
            Position::BB => "BB",
        };
        path += match self.action {
            Action::Fold => "",
            Action::Check => "_CHECK.txt",
            Action::Call => "_CALL.txt",
            Action::Raise => "_OPEN.txt",
            Action::ThreeBet => "_3BET.txt",
            Action::FourBet => "_4BET.txt",
        };
        let mut orders: Vec<String> = Vec::new();
        println!("{}", path);
        let file = File::open(path).unwrap();
        let reader = BufReader::new(file);
        for line in reader.lines() {
            let order: String = line.unwrap();
            orders.push(order);
        }
        range::make(&orders)
    }
}
