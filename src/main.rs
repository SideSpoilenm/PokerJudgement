mod module;
use module::card::Card;
use module::convert;
use std::io::{stdin, stdout, Write};

use crate::module::equity;

#[allow(dead_code)]
fn main() {
    let mut cards: Vec<Card> = Vec::new();

    println!("*************** CARDS ***************");
    for label in [
        "hero's 1st card: ",
        "hero's 2nd card: ",
        "villain's 1st card: ",
        "villain's 2nd card: ",
        "Flop 1st card: ",
        "Flop 2nd card: ",
        "Flop 3rd card: ",
    ] {
        print!("{}", label);
        stdout().flush().unwrap(); // 中間領域にある文字を強制的に標準出力
        let mut text: String = String::new();
        stdin().read_line(&mut text).expect("Empty");
        cards.push(convert::lavel_to_card(text));
    }
    let hero_cards: Vec<Card> = (&cards[0..2]).to_vec();
    let vill_cards: Vec<Card> = (&cards[2..4]).to_vec();
    let flop_cards: Vec<Card> = (&cards[4..7]).to_vec();
    let eq = equity::calc(&flop_cards, &hero_cards, &vill_cards);
    println!("*************** EQUITY ***************");
    print!("hero: {}% \t villain: {}%", eq.0, eq.1);
}
