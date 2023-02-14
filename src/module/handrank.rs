use super::card::{Card, Rank, Suit};

#[allow(dead_code)]
#[derive(PartialEq, Default, Debug)]
pub enum HandRank {
    #[default]
    High,
    OnePair { pair: Rank, },
    TwoPair { pair1: Rank, pair2: Rank, },
    Trips   { set: Rank, },
    Straight { initial: Rank, },
    Flush   { suit: Suit, },
    Fullhouse { set: Rank, pair: Rank, },
    Quads   { set: Rank, },
    StFlush { initial: Rank, suit: Suit, },
    Royal   { suit: Suit, },
}

impl HandRank {
    pub fn value(&self) -> u8 {
        let val: u8 = match self {
            HandRank::High => 0_u8,
            HandRank::OnePair { pair:_ } => 1_u8,
            HandRank::TwoPair { pair1:_, pair2:_ } => 2_u8,
            HandRank::Trips { set:_ } => 3_u8,
            HandRank::Straight { initial:_ } => 4_u8,
            HandRank::Flush { suit:_ } => 5_u8,
            HandRank::Fullhouse { set:_, pair:_ } => 6_u8,
            HandRank::Quads { set:_ } => 7_u8,
            HandRank::StFlush { initial:_, suit:_ } => 8_u8,
            HandRank::Royal { suit:_ } => 9_u8,
        };
        val
    }
    pub fn info(&self) -> Vec<Card> {
        let item: Vec<Card> = match self {
            HandRank::High => vec![Card { suit: Suit::Joker, rank: Rank::X } ],
            HandRank::OnePair { pair } => vec![Card { suit: Suit::Joker, rank: *pair } ],
            HandRank::TwoPair { pair1, pair2 } => vec![
                Card { suit: Suit::Joker, rank: *pair1 },
                Card { suit: Suit::Joker, rank: *pair2 },
            ],
            HandRank::Trips { set } => vec![ Card{ suit:Suit::Joker, rank:*set } ],
            HandRank::Straight { initial } => vec![ Card{suit:Suit::Joker, rank:*initial } ],
            HandRank::Flush { suit } => vec![ Card{ suit:*suit, rank:Rank::X } ],
            HandRank::Fullhouse { set, pair } => vec![
                Card { suit: Suit::Joker, rank: *set },
                Card { suit: Suit::Joker, rank: *pair },
            ],
            HandRank::Quads { set } => vec![ Card{ suit:Suit::Joker, rank:*set } ],
            HandRank::StFlush { initial, suit } => vec![ Card{ suit:*suit, rank:*initial } ],
            HandRank::Royal { suit } => vec![ Card{ suit:*suit, rank:Rank::Ace} ],
        };
        item
    }
}
