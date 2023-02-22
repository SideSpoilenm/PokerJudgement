use super::card::{Card, Rank, Suit};
use super::dealer::Position;

pub fn str_to_rank(s: &str) -> Rank{
    let rank: Rank = match s {
        "X" => Rank::X,
        "A" => Rank::Ace,
        "2" => Rank::Deuce,
        "3" => Rank::Three,
        "4" => Rank::Four,
        "5" => Rank::Five,
        "6" => Rank::Six,
        "7" => Rank::Seven,
        "8" => Rank::Eight,
        "9" => Rank::Nine,
        "T" => Rank::Ten,
        "J" => Rank::Jack,
        "Q" => Rank::Queen,
        "K" => Rank::King,
         _  => Rank::X,
    };
    return rank
}

#[allow(dead_code)]
pub fn str_to_suit(s: &str) -> Suit{
    let suit: Suit = match s {
        "s" => Suit::Spade,
        "h" => Suit::Heart,
        "d" => Suit::Diamond,
        "c" => Suit::Club, 
         _  => Suit::Joker,
    };
    suit
}

#[allow(dead_code)]
pub fn lavel_to_card(text: String) -> Card{
    let mut t = text.as_str().chars();
    let r = t.nth(0).unwrap().to_string();
    let s = t.nth(0).unwrap().to_string();
    Card{rank: str_to_rank(&r), suit: str_to_suit(&s)}
}

pub fn hand_to_label(hand: &Vec<Card>)->String{
    let mut s = "".to_string();
    for card in hand{
        s = format!("{}{}", s, card.to_lavel());
    }
    s
}

pub fn str_to_position(s: &str) -> Position{
    let position: Position = match s{
        "UTG" => Position::UTG,
        "EP1" => Position::EP1,
        "EP2" => Position::EP2,
        "MP1" => Position::MP1,
        "MP2" => Position::MP2,
        "HJ" => Position::HJ,
        "CO" => Position::CO,
        "BTN" => Position::BTN,
        "SB" => Position::SB,
        "BB" => Position::BB,
        _ => Position::HJ,
    };
    position
}