use super::card::{Card, Rank, Suit};

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

pub fn lavel_to_card(text: String) -> Card{
    let mut t = text.as_str().chars();
    let r = t.nth(0).unwrap().to_string();
    let s = t.nth(0).unwrap().to_string();
    Card{rank: str_to_rank(&r), suit: str_to_suit(&s)}
}