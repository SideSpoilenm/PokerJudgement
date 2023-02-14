// カード
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Card {
    pub rank: Rank,
    pub suit: Suit,
}

#[allow(unused)]
impl Card {
    // カードを[As]表記にする
    pub fn to_lavel(&self) -> String {
        let r: &str = match self.rank {
            Rank::X => "X",
            Rank::Ace => "A",
            Rank::Deuce => "2",
            Rank::Three => "3",
            Rank::Four => "4",
            Rank::Five => "5",
            Rank::Six => "6",
            Rank::Seven => "7",
            Rank::Eight => "8",
            Rank::Nine => "9",
            Rank::Ten => "T",
            Rank::Jack => "J",
            Rank::Queen => "Q",
            Rank::King => "K",
        };
        let s = match self.suit {
            Suit::Joker => "*",
            Suit::Spade => "s",
            Suit::Heart => "h",
            Suit::Diamond => "d",
            Suit::Club => "c",
        };
        let mut name = r.to_string();
        name += s;
        name
    }
}

#[allow(dead_code)]
#[derive(Clone, Copy, PartialEq, Default, Debug)]
pub enum Rank {
    #[default]
    X,
    Ace,
    Deuce,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
}

impl Rank {
    pub fn value(&self) -> u8 {
        let value: u8 = match self {
            Rank::X => 0_u8,
            Rank::Ace => 14_u8,
            Rank::Deuce => 2_u8,
            Rank::Three => 3_u8,
            Rank::Four => 4_u8,
            Rank::Five => 5_u8,
            Rank::Six => 6_u8,
            Rank::Seven => 7_u8,
            Rank::Eight => 8_u8,
            Rank::Nine => 9_u8,
            Rank::Ten => 10_u8,
            Rank::Jack => 11_u8,
            Rank::Queen => 12_u8,
            Rank::King => 13_u8,
        };
        value
    }
}

#[allow(dead_code)]
#[derive(Clone, Copy, PartialEq, Default, Debug)]
pub enum Suit {
    Spade,
    Heart,
    Diamond,
    Club,
    #[default]
    Joker,
}

#[allow(unused)]
impl Suit {
    pub fn read(&self) -> String {
        let name = match self {
            Suit::Joker => "*",
            Suit::Spade => "Spades",
            Suit::Heart => "Hearts",
            Suit::Diamond => "Diamond",
            Suit::Club => "Clubs",
        };
        name.to_string()
    }
}
