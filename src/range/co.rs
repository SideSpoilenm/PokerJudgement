use super::super::module::card::{Card, Suit, Rank};

#[allow(unused_imports)]
pub const CO_RANGE: [[Card; 2]; 304] = [
    [Card{suit:Suit::Spade, rank:Rank::Ace}, Card{suit:Suit::Spade, rank:Rank::King}],
    [Card{suit:Suit::Spade, rank:Rank::Ace}, Card{suit:Suit::Spade, rank:Rank::Queen}],
    [Card{suit:Suit::Spade, rank:Rank::Ace}, Card{suit:Suit::Spade, rank:Rank::Jack}],
    [Card{suit:Suit::Spade, rank:Rank::Ace}, Card{suit:Suit::Spade, rank:Rank::Ten}],
    [Card{suit:Suit::Spade, rank:Rank::Ace}, Card{suit:Suit::Spade, rank:Rank::Nine}],
    [Card{suit:Suit::Spade, rank:Rank::Ace}, Card{suit:Suit::Spade, rank:Rank::Eight}],
    [Card{suit:Suit::Spade, rank:Rank::Ace}, Card{suit:Suit::Spade, rank:Rank::Seven}],
    [Card{suit:Suit::Spade, rank:Rank::Ace}, Card{suit:Suit::Spade, rank:Rank::Six}],
    [Card{suit:Suit::Spade, rank:Rank::Ace}, Card{suit:Suit::Spade, rank:Rank::Five}],
    [Card{suit:Suit::Spade, rank:Rank::Ace}, Card{suit:Suit::Spade, rank:Rank::Four}],
    [Card{suit:Suit::Spade, rank:Rank::Ace}, Card{suit:Suit::Spade, rank:Rank::Three}],
    [Card{suit:Suit::Spade, rank:Rank::Ace}, Card{suit:Suit::Spade, rank:Rank::Deuce}],
    [Card{suit:Suit::Spade, rank:Rank::King}, Card{suit:Suit::Spade, rank:Rank::Queen}],
    [Card{suit:Suit::Spade, rank:Rank::King}, Card{suit:Suit::Spade, rank:Rank::Jack}],
    [Card{suit:Suit::Spade, rank:Rank::King}, Card{suit:Suit::Spade, rank:Rank::Ten}],
    [Card{suit:Suit::Spade, rank:Rank::King}, Card{suit:Suit::Spade, rank:Rank::Nine}],
    [Card{suit:Suit::Spade, rank:Rank::King}, Card{suit:Suit::Spade, rank:Rank::Eight}],
    [Card{suit:Suit::Spade, rank:Rank::King}, Card{suit:Suit::Spade, rank:Rank::Seven}],
    [Card{suit:Suit::Spade, rank:Rank::Queen}, Card{suit:Suit::Spade, rank:Rank::Jack}],
    [Card{suit:Suit::Spade, rank:Rank::Queen}, Card{suit:Suit::Spade, rank:Rank::Ten}],
    [Card{suit:Suit::Spade, rank:Rank::Queen}, Card{suit:Suit::Spade, rank:Rank::Nine}],
    [Card{suit:Suit::Spade, rank:Rank::Queen}, Card{suit:Suit::Spade, rank:Rank::Eight}],
    [Card{suit:Suit::Spade, rank:Rank::Jack}, Card{suit:Suit::Spade, rank:Rank::Ten}],
    [Card{suit:Suit::Spade, rank:Rank::Jack}, Card{suit:Suit::Spade, rank:Rank::Nine}],
    [Card{suit:Suit::Spade, rank:Rank::Jack}, Card{suit:Suit::Spade, rank:Rank::Eight}],
    [Card{suit:Suit::Spade, rank:Rank::Ten}, Card{suit:Suit::Spade, rank:Rank::Nine}],
    [Card{suit:Suit::Spade, rank:Rank::Ten}, Card{suit:Suit::Spade, rank:Rank::Eight}],
    [Card{suit:Suit::Spade, rank:Rank::Nine}, Card{suit:Suit::Spade, rank:Rank::Eight}],
    [Card{suit:Suit::Spade, rank:Rank::Eight}, Card{suit:Suit::Spade, rank:Rank::Seven}],
    [Card{suit:Suit::Spade, rank:Rank::Seven}, Card{suit:Suit::Spade, rank:Rank::Six}],
    [Card{suit:Suit::Spade, rank:Rank::Six}, Card{suit:Suit::Spade, rank:Rank::Five}],
    [Card{suit:Suit::Heart, rank:Rank::Ace}, Card{suit:Suit::Heart, rank:Rank::King}],
    [Card{suit:Suit::Heart, rank:Rank::Ace}, Card{suit:Suit::Heart, rank:Rank::Queen}],
    [Card{suit:Suit::Heart, rank:Rank::Ace}, Card{suit:Suit::Heart, rank:Rank::Jack}],
    [Card{suit:Suit::Heart, rank:Rank::Ace}, Card{suit:Suit::Heart, rank:Rank::Ten}],
    [Card{suit:Suit::Heart, rank:Rank::Ace}, Card{suit:Suit::Heart, rank:Rank::Nine}],
    [Card{suit:Suit::Heart, rank:Rank::Ace}, Card{suit:Suit::Heart, rank:Rank::Eight}],
    [Card{suit:Suit::Heart, rank:Rank::Ace}, Card{suit:Suit::Heart, rank:Rank::Seven}],
    [Card{suit:Suit::Heart, rank:Rank::Ace}, Card{suit:Suit::Heart, rank:Rank::Six}],
    [Card{suit:Suit::Heart, rank:Rank::Ace}, Card{suit:Suit::Heart, rank:Rank::Five}],
    [Card{suit:Suit::Heart, rank:Rank::Ace}, Card{suit:Suit::Heart, rank:Rank::Four}],
    [Card{suit:Suit::Heart, rank:Rank::Ace}, Card{suit:Suit::Heart, rank:Rank::Three}],
    [Card{suit:Suit::Heart, rank:Rank::Ace}, Card{suit:Suit::Heart, rank:Rank::Deuce}],
    [Card{suit:Suit::Heart, rank:Rank::King}, Card{suit:Suit::Heart, rank:Rank::Queen}],
    [Card{suit:Suit::Heart, rank:Rank::King}, Card{suit:Suit::Heart, rank:Rank::Jack}],
    [Card{suit:Suit::Heart, rank:Rank::King}, Card{suit:Suit::Heart, rank:Rank::Ten}],
    [Card{suit:Suit::Heart, rank:Rank::King}, Card{suit:Suit::Heart, rank:Rank::Nine}],
    [Card{suit:Suit::Heart, rank:Rank::King}, Card{suit:Suit::Heart, rank:Rank::Eight}],
    [Card{suit:Suit::Heart, rank:Rank::King}, Card{suit:Suit::Heart, rank:Rank::Seven}],
    [Card{suit:Suit::Heart, rank:Rank::Queen}, Card{suit:Suit::Heart, rank:Rank::Jack}],
    [Card{suit:Suit::Heart, rank:Rank::Queen}, Card{suit:Suit::Heart, rank:Rank::Ten}],
    [Card{suit:Suit::Heart, rank:Rank::Queen}, Card{suit:Suit::Heart, rank:Rank::Nine}],
    [Card{suit:Suit::Heart, rank:Rank::Queen}, Card{suit:Suit::Heart, rank:Rank::Eight}],
    [Card{suit:Suit::Heart, rank:Rank::Jack}, Card{suit:Suit::Heart, rank:Rank::Ten}],
    [Card{suit:Suit::Heart, rank:Rank::Jack}, Card{suit:Suit::Heart, rank:Rank::Nine}],
    [Card{suit:Suit::Heart, rank:Rank::Jack}, Card{suit:Suit::Heart, rank:Rank::Eight}],
    [Card{suit:Suit::Heart, rank:Rank::Ten}, Card{suit:Suit::Heart, rank:Rank::Nine}],
    [Card{suit:Suit::Heart, rank:Rank::Ten}, Card{suit:Suit::Heart, rank:Rank::Eight}],
    [Card{suit:Suit::Heart, rank:Rank::Nine}, Card{suit:Suit::Heart, rank:Rank::Eight}],
    [Card{suit:Suit::Heart, rank:Rank::Eight}, Card{suit:Suit::Heart, rank:Rank::Seven}],
    [Card{suit:Suit::Heart, rank:Rank::Seven}, Card{suit:Suit::Heart, rank:Rank::Six}],
    [Card{suit:Suit::Heart, rank:Rank::Six}, Card{suit:Suit::Heart, rank:Rank::Five}],
    [Card{suit:Suit::Diamond, rank:Rank::Ace}, Card{suit:Suit::Diamond, rank:Rank::King}],
    [Card{suit:Suit::Diamond, rank:Rank::Ace}, Card{suit:Suit::Diamond, rank:Rank::Queen}],
    [Card{suit:Suit::Diamond, rank:Rank::Ace}, Card{suit:Suit::Diamond, rank:Rank::Jack}],
    [Card{suit:Suit::Diamond, rank:Rank::Ace}, Card{suit:Suit::Diamond, rank:Rank::Ten}],
    [Card{suit:Suit::Diamond, rank:Rank::Ace}, Card{suit:Suit::Diamond, rank:Rank::Nine}],
    [Card{suit:Suit::Diamond, rank:Rank::Ace}, Card{suit:Suit::Diamond, rank:Rank::Eight}],
    [Card{suit:Suit::Diamond, rank:Rank::Ace}, Card{suit:Suit::Diamond, rank:Rank::Seven}],
    [Card{suit:Suit::Diamond, rank:Rank::Ace}, Card{suit:Suit::Diamond, rank:Rank::Six}],
    [Card{suit:Suit::Diamond, rank:Rank::Ace}, Card{suit:Suit::Diamond, rank:Rank::Five}],
    [Card{suit:Suit::Diamond, rank:Rank::Ace}, Card{suit:Suit::Diamond, rank:Rank::Four}],
    [Card{suit:Suit::Diamond, rank:Rank::Ace}, Card{suit:Suit::Diamond, rank:Rank::Three}],
    [Card{suit:Suit::Diamond, rank:Rank::Ace}, Card{suit:Suit::Diamond, rank:Rank::Deuce}],
    [Card{suit:Suit::Diamond, rank:Rank::King}, Card{suit:Suit::Diamond, rank:Rank::Queen}],
    [Card{suit:Suit::Diamond, rank:Rank::King}, Card{suit:Suit::Diamond, rank:Rank::Jack}],
    [Card{suit:Suit::Diamond, rank:Rank::King}, Card{suit:Suit::Diamond, rank:Rank::Ten}],
    [Card{suit:Suit::Diamond, rank:Rank::King}, Card{suit:Suit::Diamond, rank:Rank::Nine}],
    [Card{suit:Suit::Diamond, rank:Rank::King}, Card{suit:Suit::Diamond, rank:Rank::Eight}],
    [Card{suit:Suit::Diamond, rank:Rank::King}, Card{suit:Suit::Diamond, rank:Rank::Seven}],
    [Card{suit:Suit::Diamond, rank:Rank::Queen}, Card{suit:Suit::Diamond, rank:Rank::Jack}],
    [Card{suit:Suit::Diamond, rank:Rank::Queen}, Card{suit:Suit::Diamond, rank:Rank::Ten}],
    [Card{suit:Suit::Diamond, rank:Rank::Queen}, Card{suit:Suit::Diamond, rank:Rank::Nine}],
    [Card{suit:Suit::Diamond, rank:Rank::Queen}, Card{suit:Suit::Diamond, rank:Rank::Eight}],
    [Card{suit:Suit::Diamond, rank:Rank::Jack}, Card{suit:Suit::Diamond, rank:Rank::Ten}],
    [Card{suit:Suit::Diamond, rank:Rank::Jack}, Card{suit:Suit::Diamond, rank:Rank::Nine}],
    [Card{suit:Suit::Diamond, rank:Rank::Jack}, Card{suit:Suit::Diamond, rank:Rank::Eight}],
    [Card{suit:Suit::Diamond, rank:Rank::Ten}, Card{suit:Suit::Diamond, rank:Rank::Nine}],
    [Card{suit:Suit::Diamond, rank:Rank::Ten}, Card{suit:Suit::Diamond, rank:Rank::Eight}],
    [Card{suit:Suit::Diamond, rank:Rank::Nine}, Card{suit:Suit::Diamond, rank:Rank::Eight}],
    [Card{suit:Suit::Diamond, rank:Rank::Eight}, Card{suit:Suit::Diamond, rank:Rank::Seven}],
    [Card{suit:Suit::Diamond, rank:Rank::Seven}, Card{suit:Suit::Diamond, rank:Rank::Six}],
    [Card{suit:Suit::Diamond, rank:Rank::Six}, Card{suit:Suit::Diamond, rank:Rank::Five}],
    [Card{suit:Suit::Club, rank:Rank::Ace}, Card{suit:Suit::Club, rank:Rank::King}],
    [Card{suit:Suit::Club, rank:Rank::Ace}, Card{suit:Suit::Club, rank:Rank::Queen}],
    [Card{suit:Suit::Club, rank:Rank::Ace}, Card{suit:Suit::Club, rank:Rank::Jack}],
    [Card{suit:Suit::Club, rank:Rank::Ace}, Card{suit:Suit::Club, rank:Rank::Ten}],
    [Card{suit:Suit::Club, rank:Rank::Ace}, Card{suit:Suit::Club, rank:Rank::Nine}],
    [Card{suit:Suit::Club, rank:Rank::Ace}, Card{suit:Suit::Club, rank:Rank::Eight}],
    [Card{suit:Suit::Club, rank:Rank::Ace}, Card{suit:Suit::Club, rank:Rank::Seven}],
    [Card{suit:Suit::Club, rank:Rank::Ace}, Card{suit:Suit::Club, rank:Rank::Six}],
    [Card{suit:Suit::Club, rank:Rank::Ace}, Card{suit:Suit::Club, rank:Rank::Five}],
    [Card{suit:Suit::Club, rank:Rank::Ace}, Card{suit:Suit::Club, rank:Rank::Four}],
    [Card{suit:Suit::Club, rank:Rank::Ace}, Card{suit:Suit::Club, rank:Rank::Three}],
    [Card{suit:Suit::Club, rank:Rank::Ace}, Card{suit:Suit::Club, rank:Rank::Deuce}],
    [Card{suit:Suit::Club, rank:Rank::King}, Card{suit:Suit::Club, rank:Rank::Queen}],
    [Card{suit:Suit::Club, rank:Rank::King}, Card{suit:Suit::Club, rank:Rank::Jack}],
    [Card{suit:Suit::Club, rank:Rank::King}, Card{suit:Suit::Club, rank:Rank::Ten}],
    [Card{suit:Suit::Club, rank:Rank::King}, Card{suit:Suit::Club, rank:Rank::Nine}],
    [Card{suit:Suit::Club, rank:Rank::King}, Card{suit:Suit::Club, rank:Rank::Eight}],
    [Card{suit:Suit::Club, rank:Rank::King}, Card{suit:Suit::Club, rank:Rank::Seven}],
    [Card{suit:Suit::Club, rank:Rank::Queen}, Card{suit:Suit::Club, rank:Rank::Jack}],
    [Card{suit:Suit::Club, rank:Rank::Queen}, Card{suit:Suit::Club, rank:Rank::Ten}],
    [Card{suit:Suit::Club, rank:Rank::Queen}, Card{suit:Suit::Club, rank:Rank::Nine}],
    [Card{suit:Suit::Club, rank:Rank::Queen}, Card{suit:Suit::Club, rank:Rank::Eight}],
    [Card{suit:Suit::Club, rank:Rank::Jack}, Card{suit:Suit::Club, rank:Rank::Ten}],
    [Card{suit:Suit::Club, rank:Rank::Jack}, Card{suit:Suit::Club, rank:Rank::Nine}],
    [Card{suit:Suit::Club, rank:Rank::Jack}, Card{suit:Suit::Club, rank:Rank::Eight}],
    [Card{suit:Suit::Club, rank:Rank::Ten}, Card{suit:Suit::Club, rank:Rank::Nine}],
    [Card{suit:Suit::Club, rank:Rank::Ten}, Card{suit:Suit::Club, rank:Rank::Eight}],
    [Card{suit:Suit::Club, rank:Rank::Nine}, Card{suit:Suit::Club, rank:Rank::Eight}],
    [Card{suit:Suit::Club, rank:Rank::Eight}, Card{suit:Suit::Club, rank:Rank::Seven}],
    [Card{suit:Suit::Club, rank:Rank::Seven}, Card{suit:Suit::Club, rank:Rank::Six}],
    [Card{suit:Suit::Club, rank:Rank::Six}, Card{suit:Suit::Club, rank:Rank::Five}],
    [Card{suit:Suit::Spade, rank:Rank::Ace}, Card{suit:Suit::Heart, rank:Rank::Ace}],
    [Card{suit:Suit::Spade, rank:Rank::Ace}, Card{suit:Suit::Diamond, rank:Rank::Ace}],
    [Card{suit:Suit::Spade, rank:Rank::Ace}, Card{suit:Suit::Club, rank:Rank::Ace}],
    [Card{suit:Suit::Spade, rank:Rank::Ace}, Card{suit:Suit::Heart, rank:Rank::King}],
    [Card{suit:Suit::Spade, rank:Rank::Ace}, Card{suit:Suit::Diamond, rank:Rank::King}],
    [Card{suit:Suit::Spade, rank:Rank::Ace}, Card{suit:Suit::Club, rank:Rank::King}],
    [Card{suit:Suit::Spade, rank:Rank::Ace}, Card{suit:Suit::Heart, rank:Rank::Queen}],
    [Card{suit:Suit::Spade, rank:Rank::Ace}, Card{suit:Suit::Diamond, rank:Rank::Queen}],
    [Card{suit:Suit::Spade, rank:Rank::Ace}, Card{suit:Suit::Club, rank:Rank::Queen}],
    [Card{suit:Suit::Spade, rank:Rank::Ace}, Card{suit:Suit::Heart, rank:Rank::Jack}],
    [Card{suit:Suit::Spade, rank:Rank::Ace}, Card{suit:Suit::Diamond, rank:Rank::Jack}],
    [Card{suit:Suit::Spade, rank:Rank::Ace}, Card{suit:Suit::Club, rank:Rank::Jack}],
    [Card{suit:Suit::Spade, rank:Rank::Ace}, Card{suit:Suit::Heart, rank:Rank::Ten}],
    [Card{suit:Suit::Spade, rank:Rank::Ace}, Card{suit:Suit::Diamond, rank:Rank::Ten}],
    [Card{suit:Suit::Spade, rank:Rank::Ace}, Card{suit:Suit::Club, rank:Rank::Ten}],
    [Card{suit:Suit::Spade, rank:Rank::King}, Card{suit:Suit::Heart, rank:Rank::King}],
    [Card{suit:Suit::Spade, rank:Rank::King}, Card{suit:Suit::Diamond, rank:Rank::King}],
    [Card{suit:Suit::Spade, rank:Rank::King}, Card{suit:Suit::Club, rank:Rank::King}],
    [Card{suit:Suit::Spade, rank:Rank::King}, Card{suit:Suit::Heart, rank:Rank::Queen}],
    [Card{suit:Suit::Spade, rank:Rank::King}, Card{suit:Suit::Diamond, rank:Rank::Queen}],
    [Card{suit:Suit::Spade, rank:Rank::King}, Card{suit:Suit::Club, rank:Rank::Queen}],
    [Card{suit:Suit::Spade, rank:Rank::King}, Card{suit:Suit::Heart, rank:Rank::Jack}],
    [Card{suit:Suit::Spade, rank:Rank::King}, Card{suit:Suit::Diamond, rank:Rank::Jack}],
    [Card{suit:Suit::Spade, rank:Rank::King}, Card{suit:Suit::Club, rank:Rank::Jack}],
    [Card{suit:Suit::Spade, rank:Rank::King}, Card{suit:Suit::Heart, rank:Rank::Ten}],
    [Card{suit:Suit::Spade, rank:Rank::King}, Card{suit:Suit::Diamond, rank:Rank::Ten}],
    [Card{suit:Suit::Spade, rank:Rank::King}, Card{suit:Suit::Club, rank:Rank::Ten}],
    [Card{suit:Suit::Spade, rank:Rank::Queen}, Card{suit:Suit::Heart, rank:Rank::Queen}],
    [Card{suit:Suit::Spade, rank:Rank::Queen}, Card{suit:Suit::Diamond, rank:Rank::Queen}],
    [Card{suit:Suit::Spade, rank:Rank::Queen}, Card{suit:Suit::Club, rank:Rank::Queen}],
    [Card{suit:Suit::Spade, rank:Rank::Queen}, Card{suit:Suit::Heart, rank:Rank::Jack}],
    [Card{suit:Suit::Spade, rank:Rank::Queen}, Card{suit:Suit::Diamond, rank:Rank::Jack}],
    [Card{suit:Suit::Spade, rank:Rank::Queen}, Card{suit:Suit::Club, rank:Rank::Jack}],
    [Card{suit:Suit::Spade, rank:Rank::Queen}, Card{suit:Suit::Heart, rank:Rank::Ten}],
    [Card{suit:Suit::Spade, rank:Rank::Queen}, Card{suit:Suit::Diamond, rank:Rank::Ten}],
    [Card{suit:Suit::Spade, rank:Rank::Queen}, Card{suit:Suit::Club, rank:Rank::Ten}],
    [Card{suit:Suit::Spade, rank:Rank::Jack}, Card{suit:Suit::Heart, rank:Rank::Jack}],
    [Card{suit:Suit::Spade, rank:Rank::Jack}, Card{suit:Suit::Diamond, rank:Rank::Jack}],
    [Card{suit:Suit::Spade, rank:Rank::Jack}, Card{suit:Suit::Club, rank:Rank::Jack}],
    [Card{suit:Suit::Spade, rank:Rank::Jack}, Card{suit:Suit::Heart, rank:Rank::Ten}],
    [Card{suit:Suit::Spade, rank:Rank::Jack}, Card{suit:Suit::Diamond, rank:Rank::Ten}],
    [Card{suit:Suit::Spade, rank:Rank::Jack}, Card{suit:Suit::Club, rank:Rank::Ten}],
    [Card{suit:Suit::Spade, rank:Rank::Ten}, Card{suit:Suit::Heart, rank:Rank::Ten}],
    [Card{suit:Suit::Spade, rank:Rank::Ten}, Card{suit:Suit::Diamond, rank:Rank::Ten}],
    [Card{suit:Suit::Spade, rank:Rank::Ten}, Card{suit:Suit::Club, rank:Rank::Ten}],
    [Card{suit:Suit::Spade, rank:Rank::Nine}, Card{suit:Suit::Heart, rank:Rank::Nine}],
    [Card{suit:Suit::Spade, rank:Rank::Nine}, Card{suit:Suit::Diamond, rank:Rank::Nine}],
    [Card{suit:Suit::Spade, rank:Rank::Nine}, Card{suit:Suit::Club, rank:Rank::Nine}],
    [Card{suit:Suit::Spade, rank:Rank::Eight}, Card{suit:Suit::Heart, rank:Rank::Eight}],
    [Card{suit:Suit::Spade, rank:Rank::Eight}, Card{suit:Suit::Diamond, rank:Rank::Eight}],
    [Card{suit:Suit::Spade, rank:Rank::Eight}, Card{suit:Suit::Club, rank:Rank::Eight}],
    [Card{suit:Suit::Spade, rank:Rank::Seven}, Card{suit:Suit::Heart, rank:Rank::Seven}],
    [Card{suit:Suit::Spade, rank:Rank::Seven}, Card{suit:Suit::Diamond, rank:Rank::Seven}],
    [Card{suit:Suit::Spade, rank:Rank::Seven}, Card{suit:Suit::Club, rank:Rank::Seven}],
    [Card{suit:Suit::Spade, rank:Rank::Six}, Card{suit:Suit::Heart, rank:Rank::Six}],
    [Card{suit:Suit::Spade, rank:Rank::Six}, Card{suit:Suit::Diamond, rank:Rank::Six}],
    [Card{suit:Suit::Spade, rank:Rank::Six}, Card{suit:Suit::Club, rank:Rank::Six}],
    [Card{suit:Suit::Spade, rank:Rank::Five}, Card{suit:Suit::Heart, rank:Rank::Five}],
    [Card{suit:Suit::Spade, rank:Rank::Five}, Card{suit:Suit::Diamond, rank:Rank::Five}],
    [Card{suit:Suit::Spade, rank:Rank::Five}, Card{suit:Suit::Club, rank:Rank::Five}],
    [Card{suit:Suit::Heart, rank:Rank::Ace}, Card{suit:Suit::Diamond, rank:Rank::Ace}],
    [Card{suit:Suit::Heart, rank:Rank::Ace}, Card{suit:Suit::Club, rank:Rank::Ace}],
    [Card{suit:Suit::Heart, rank:Rank::Ace}, Card{suit:Suit::Spade, rank:Rank::King}],
    [Card{suit:Suit::Heart, rank:Rank::Ace}, Card{suit:Suit::Diamond, rank:Rank::King}],
    [Card{suit:Suit::Heart, rank:Rank::Ace}, Card{suit:Suit::Club, rank:Rank::King}],
    [Card{suit:Suit::Heart, rank:Rank::Ace}, Card{suit:Suit::Spade, rank:Rank::Queen}],
    [Card{suit:Suit::Heart, rank:Rank::Ace}, Card{suit:Suit::Diamond, rank:Rank::Queen}],
    [Card{suit:Suit::Heart, rank:Rank::Ace}, Card{suit:Suit::Club, rank:Rank::Queen}],
    [Card{suit:Suit::Heart, rank:Rank::Ace}, Card{suit:Suit::Spade, rank:Rank::Jack}],
    [Card{suit:Suit::Heart, rank:Rank::Ace}, Card{suit:Suit::Diamond, rank:Rank::Jack}],
    [Card{suit:Suit::Heart, rank:Rank::Ace}, Card{suit:Suit::Club, rank:Rank::Jack}],
    [Card{suit:Suit::Heart, rank:Rank::Ace}, Card{suit:Suit::Spade, rank:Rank::Ten}],
    [Card{suit:Suit::Heart, rank:Rank::Ace}, Card{suit:Suit::Diamond, rank:Rank::Ten}],
    [Card{suit:Suit::Heart, rank:Rank::Ace}, Card{suit:Suit::Club, rank:Rank::Ten}],
    [Card{suit:Suit::Heart, rank:Rank::King}, Card{suit:Suit::Diamond, rank:Rank::King}],
    [Card{suit:Suit::Heart, rank:Rank::King}, Card{suit:Suit::Club, rank:Rank::King}],
    [Card{suit:Suit::Heart, rank:Rank::King}, Card{suit:Suit::Spade, rank:Rank::Queen}],
    [Card{suit:Suit::Heart, rank:Rank::King}, Card{suit:Suit::Diamond, rank:Rank::Queen}],
    [Card{suit:Suit::Heart, rank:Rank::King}, Card{suit:Suit::Club, rank:Rank::Queen}],
    [Card{suit:Suit::Heart, rank:Rank::King}, Card{suit:Suit::Spade, rank:Rank::Jack}],
    [Card{suit:Suit::Heart, rank:Rank::King}, Card{suit:Suit::Diamond, rank:Rank::Jack}],
    [Card{suit:Suit::Heart, rank:Rank::King}, Card{suit:Suit::Club, rank:Rank::Jack}],
    [Card{suit:Suit::Heart, rank:Rank::King}, Card{suit:Suit::Spade, rank:Rank::Ten}],
    [Card{suit:Suit::Heart, rank:Rank::King}, Card{suit:Suit::Diamond, rank:Rank::Ten}],
    [Card{suit:Suit::Heart, rank:Rank::King}, Card{suit:Suit::Club, rank:Rank::Ten}],
    [Card{suit:Suit::Heart, rank:Rank::Queen}, Card{suit:Suit::Diamond, rank:Rank::Queen}],
    [Card{suit:Suit::Heart, rank:Rank::Queen}, Card{suit:Suit::Club, rank:Rank::Queen}],
    [Card{suit:Suit::Heart, rank:Rank::Queen}, Card{suit:Suit::Spade, rank:Rank::Jack}],
    [Card{suit:Suit::Heart, rank:Rank::Queen}, Card{suit:Suit::Diamond, rank:Rank::Jack}],
    [Card{suit:Suit::Heart, rank:Rank::Queen}, Card{suit:Suit::Club, rank:Rank::Jack}],
    [Card{suit:Suit::Heart, rank:Rank::Queen}, Card{suit:Suit::Spade, rank:Rank::Ten}],
    [Card{suit:Suit::Heart, rank:Rank::Queen}, Card{suit:Suit::Diamond, rank:Rank::Ten}],
    [Card{suit:Suit::Heart, rank:Rank::Queen}, Card{suit:Suit::Club, rank:Rank::Ten}],
    [Card{suit:Suit::Heart, rank:Rank::Jack}, Card{suit:Suit::Diamond, rank:Rank::Jack}],
    [Card{suit:Suit::Heart, rank:Rank::Jack}, Card{suit:Suit::Club, rank:Rank::Jack}],
    [Card{suit:Suit::Heart, rank:Rank::Jack}, Card{suit:Suit::Spade, rank:Rank::Ten}],
    [Card{suit:Suit::Heart, rank:Rank::Jack}, Card{suit:Suit::Diamond, rank:Rank::Ten}],
    [Card{suit:Suit::Heart, rank:Rank::Jack}, Card{suit:Suit::Club, rank:Rank::Ten}],
    [Card{suit:Suit::Heart, rank:Rank::Ten}, Card{suit:Suit::Diamond, rank:Rank::Ten}],
    [Card{suit:Suit::Heart, rank:Rank::Ten}, Card{suit:Suit::Club, rank:Rank::Ten}],
    [Card{suit:Suit::Heart, rank:Rank::Nine}, Card{suit:Suit::Diamond, rank:Rank::Nine}],
    [Card{suit:Suit::Heart, rank:Rank::Nine}, Card{suit:Suit::Club, rank:Rank::Nine}],
    [Card{suit:Suit::Heart, rank:Rank::Eight}, Card{suit:Suit::Diamond, rank:Rank::Eight}],
    [Card{suit:Suit::Heart, rank:Rank::Eight}, Card{suit:Suit::Club, rank:Rank::Eight}],
    [Card{suit:Suit::Heart, rank:Rank::Seven}, Card{suit:Suit::Diamond, rank:Rank::Seven}],
    [Card{suit:Suit::Heart, rank:Rank::Seven}, Card{suit:Suit::Club, rank:Rank::Seven}],
    [Card{suit:Suit::Heart, rank:Rank::Six}, Card{suit:Suit::Diamond, rank:Rank::Six}],
    [Card{suit:Suit::Heart, rank:Rank::Six}, Card{suit:Suit::Club, rank:Rank::Six}],
    [Card{suit:Suit::Heart, rank:Rank::Five}, Card{suit:Suit::Diamond, rank:Rank::Five}],
    [Card{suit:Suit::Heart, rank:Rank::Five}, Card{suit:Suit::Club, rank:Rank::Five}],
    [Card{suit:Suit::Diamond, rank:Rank::Ace}, Card{suit:Suit::Club, rank:Rank::Ace}],
    [Card{suit:Suit::Diamond, rank:Rank::Ace}, Card{suit:Suit::Spade, rank:Rank::King}],
    [Card{suit:Suit::Diamond, rank:Rank::Ace}, Card{suit:Suit::Heart, rank:Rank::King}],
    [Card{suit:Suit::Diamond, rank:Rank::Ace}, Card{suit:Suit::Club, rank:Rank::King}],
    [Card{suit:Suit::Diamond, rank:Rank::Ace}, Card{suit:Suit::Spade, rank:Rank::Queen}],
    [Card{suit:Suit::Diamond, rank:Rank::Ace}, Card{suit:Suit::Heart, rank:Rank::Queen}],
    [Card{suit:Suit::Diamond, rank:Rank::Ace}, Card{suit:Suit::Club, rank:Rank::Queen}],
    [Card{suit:Suit::Diamond, rank:Rank::Ace}, Card{suit:Suit::Spade, rank:Rank::Jack}],
    [Card{suit:Suit::Diamond, rank:Rank::Ace}, Card{suit:Suit::Heart, rank:Rank::Jack}],
    [Card{suit:Suit::Diamond, rank:Rank::Ace}, Card{suit:Suit::Club, rank:Rank::Jack}],
    [Card{suit:Suit::Diamond, rank:Rank::Ace}, Card{suit:Suit::Spade, rank:Rank::Ten}],
    [Card{suit:Suit::Diamond, rank:Rank::Ace}, Card{suit:Suit::Heart, rank:Rank::Ten}],
    [Card{suit:Suit::Diamond, rank:Rank::Ace}, Card{suit:Suit::Club, rank:Rank::Ten}],
    [Card{suit:Suit::Diamond, rank:Rank::King}, Card{suit:Suit::Club, rank:Rank::King}],
    [Card{suit:Suit::Diamond, rank:Rank::King}, Card{suit:Suit::Spade, rank:Rank::Queen}],
    [Card{suit:Suit::Diamond, rank:Rank::King}, Card{suit:Suit::Heart, rank:Rank::Queen}],
    [Card{suit:Suit::Diamond, rank:Rank::King}, Card{suit:Suit::Club, rank:Rank::Queen}],
    [Card{suit:Suit::Diamond, rank:Rank::King}, Card{suit:Suit::Spade, rank:Rank::Jack}],
    [Card{suit:Suit::Diamond, rank:Rank::King}, Card{suit:Suit::Heart, rank:Rank::Jack}],
    [Card{suit:Suit::Diamond, rank:Rank::King}, Card{suit:Suit::Club, rank:Rank::Jack}],
    [Card{suit:Suit::Diamond, rank:Rank::King}, Card{suit:Suit::Spade, rank:Rank::Ten}],
    [Card{suit:Suit::Diamond, rank:Rank::King}, Card{suit:Suit::Heart, rank:Rank::Ten}],
    [Card{suit:Suit::Diamond, rank:Rank::King}, Card{suit:Suit::Club, rank:Rank::Ten}],
    [Card{suit:Suit::Diamond, rank:Rank::Queen}, Card{suit:Suit::Club, rank:Rank::Queen}],
    [Card{suit:Suit::Diamond, rank:Rank::Queen}, Card{suit:Suit::Spade, rank:Rank::Jack}],
    [Card{suit:Suit::Diamond, rank:Rank::Queen}, Card{suit:Suit::Heart, rank:Rank::Jack}],
    [Card{suit:Suit::Diamond, rank:Rank::Queen}, Card{suit:Suit::Club, rank:Rank::Jack}],
    [Card{suit:Suit::Diamond, rank:Rank::Queen}, Card{suit:Suit::Spade, rank:Rank::Ten}],
    [Card{suit:Suit::Diamond, rank:Rank::Queen}, Card{suit:Suit::Heart, rank:Rank::Ten}],
    [Card{suit:Suit::Diamond, rank:Rank::Queen}, Card{suit:Suit::Club, rank:Rank::Ten}],
    [Card{suit:Suit::Diamond, rank:Rank::Jack}, Card{suit:Suit::Club, rank:Rank::Jack}],
    [Card{suit:Suit::Diamond, rank:Rank::Jack}, Card{suit:Suit::Spade, rank:Rank::Ten}],
    [Card{suit:Suit::Diamond, rank:Rank::Jack}, Card{suit:Suit::Heart, rank:Rank::Ten}],
    [Card{suit:Suit::Diamond, rank:Rank::Jack}, Card{suit:Suit::Club, rank:Rank::Ten}],
    [Card{suit:Suit::Diamond, rank:Rank::Ten}, Card{suit:Suit::Club, rank:Rank::Ten}],
    [Card{suit:Suit::Diamond, rank:Rank::Nine}, Card{suit:Suit::Club, rank:Rank::Nine}],
    [Card{suit:Suit::Diamond, rank:Rank::Eight}, Card{suit:Suit::Club, rank:Rank::Eight}],
    [Card{suit:Suit::Diamond, rank:Rank::Seven}, Card{suit:Suit::Club, rank:Rank::Seven}],
    [Card{suit:Suit::Diamond, rank:Rank::Six}, Card{suit:Suit::Club, rank:Rank::Six}],
    [Card{suit:Suit::Diamond, rank:Rank::Five}, Card{suit:Suit::Club, rank:Rank::Five}],
    [Card{suit:Suit::Club, rank:Rank::Ace}, Card{suit:Suit::Spade, rank:Rank::King}],
    [Card{suit:Suit::Club, rank:Rank::Ace}, Card{suit:Suit::Heart, rank:Rank::King}],
    [Card{suit:Suit::Club, rank:Rank::Ace}, Card{suit:Suit::Diamond, rank:Rank::King}],
    [Card{suit:Suit::Club, rank:Rank::Ace}, Card{suit:Suit::Spade, rank:Rank::Queen}],
    [Card{suit:Suit::Club, rank:Rank::Ace}, Card{suit:Suit::Heart, rank:Rank::Queen}],
    [Card{suit:Suit::Club, rank:Rank::Ace}, Card{suit:Suit::Diamond, rank:Rank::Queen}],
    [Card{suit:Suit::Club, rank:Rank::Ace}, Card{suit:Suit::Spade, rank:Rank::Jack}],
    [Card{suit:Suit::Club, rank:Rank::Ace}, Card{suit:Suit::Heart, rank:Rank::Jack}],
    [Card{suit:Suit::Club, rank:Rank::Ace}, Card{suit:Suit::Diamond, rank:Rank::Jack}],
    [Card{suit:Suit::Club, rank:Rank::Ace}, Card{suit:Suit::Spade, rank:Rank::Ten}],
    [Card{suit:Suit::Club, rank:Rank::Ace}, Card{suit:Suit::Heart, rank:Rank::Ten}],
    [Card{suit:Suit::Club, rank:Rank::Ace}, Card{suit:Suit::Diamond, rank:Rank::Ten}],
    [Card{suit:Suit::Club, rank:Rank::King}, Card{suit:Suit::Spade, rank:Rank::Queen}],
    [Card{suit:Suit::Club, rank:Rank::King}, Card{suit:Suit::Heart, rank:Rank::Queen}],
    [Card{suit:Suit::Club, rank:Rank::King}, Card{suit:Suit::Diamond, rank:Rank::Queen}],
    [Card{suit:Suit::Club, rank:Rank::King}, Card{suit:Suit::Spade, rank:Rank::Jack}],
    [Card{suit:Suit::Club, rank:Rank::King}, Card{suit:Suit::Heart, rank:Rank::Jack}],
    [Card{suit:Suit::Club, rank:Rank::King}, Card{suit:Suit::Diamond, rank:Rank::Jack}],
    [Card{suit:Suit::Club, rank:Rank::King}, Card{suit:Suit::Spade, rank:Rank::Ten}],
    [Card{suit:Suit::Club, rank:Rank::King}, Card{suit:Suit::Heart, rank:Rank::Ten}],
    [Card{suit:Suit::Club, rank:Rank::King}, Card{suit:Suit::Diamond, rank:Rank::Ten}],
    [Card{suit:Suit::Club, rank:Rank::Queen}, Card{suit:Suit::Spade, rank:Rank::Jack}],
    [Card{suit:Suit::Club, rank:Rank::Queen}, Card{suit:Suit::Heart, rank:Rank::Jack}],
    [Card{suit:Suit::Club, rank:Rank::Queen}, Card{suit:Suit::Diamond, rank:Rank::Jack}],
    [Card{suit:Suit::Club, rank:Rank::Queen}, Card{suit:Suit::Spade, rank:Rank::Ten}],
    [Card{suit:Suit::Club, rank:Rank::Queen}, Card{suit:Suit::Heart, rank:Rank::Ten}],
    [Card{suit:Suit::Club, rank:Rank::Queen}, Card{suit:Suit::Diamond, rank:Rank::Ten}],
    [Card{suit:Suit::Club, rank:Rank::Jack}, Card{suit:Suit::Spade, rank:Rank::Ten}],
    [Card{suit:Suit::Club, rank:Rank::Jack}, Card{suit:Suit::Heart, rank:Rank::Ten}],
    [Card{suit:Suit::Club, rank:Rank::Jack}, Card{suit:Suit::Diamond, rank:Rank::Ten}],
];