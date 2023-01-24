use super::card::{Rank, Suit};

#[allow(dead_code)]
#[derive(PartialEq, Default, Debug)]
pub enum HandRank{
    #[default]
    High,
    OnePair{pair: Rank},
    TwoPair{pair_high: Rank, pair_low: Rank},
    Trips{initial: Rank},
    Straight{initial: Rank},
    Flush{suit: Suit},
    Fullhouse{initial: Rank, pair: Rank},
    Quads{initial:Rank},
    StFlush{initial: Rank, suit: Suit},
    Royal{suit: Suit},
}

impl HandRank {
    pub fn get_value(&self) -> u8{
        let value: u8 = match self {
            HandRank::High => 0_u8,
            HandRank::OnePair { pair:_ } => 1_u8,
            HandRank::TwoPair { pair_high:_, pair_low:_ } => 2_u8,
            HandRank::Trips { initial:_ } => 3_u8,
            HandRank::Straight { initial:_ } => 4_u8,
            HandRank::Flush { suit:_ } => 5_u8,
            HandRank::Fullhouse { initial:_, pair:_ } => 6_u8,
            HandRank::Quads { initial:_ } => 7_u8,
            HandRank::StFlush { initial:_, suit:_ } => 8_u8,
            HandRank::Royal { suit:_ } => 9_u8,
        };
        value
    }
}
