use std::io::{stdin, stdout, Write, Read, BufRead, BufReader};
use std::fs::File;
use std::env;
mod module;
mod range;
// use module::card::{Card, Suit, Rank};
// use module::convert;
// use crate::module::estimation;
// use crate::module::equity;

#[allow(dead_code)]
fn main() {
    let mut orders: Vec<String> = Vec::new();
    let args: Vec<String> = env::args().collect();
    let path = &args[1];
    println!("{}", path);
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);
    for line in reader.lines(){
        let order: String = line.unwrap();
        orders.push(order);
    }

    println!("{:?}", range::make(&orders));

    // let mut cards: Vec<Card> = Vec::new();

    // println!("*************** CARDS ***************");
    // for label in [
    //     "hero's 1st card: ",
    //     "hero's 2nd card: ",
    //     "villain's 1st card: ",
    //     "villain's 2nd card: ",
    //     "Flop 1st card: ",
    //     "Flop 2nd card: ",
    //     "Flop 3rd card: ",
    // ] {
    //     print!("{}", label);
    //     stdout().flush().unwrap(); // 中間領域にある文字を強制的に標準出力
    //     let mut text: String = String::new();
    //     stdin().read_line(&mut text).expect("Empty");
    //     cards.push(convert::lavel_to_card(text));
    // }
    // let hero_cards: Vec<Card> = (&cards[0..2]).to_vec();
    // let vill_cards: Vec<Card> = (&cards[2..4]).to_vec();
    // let flop_cards: Vec<Card> = (&cards[4..7]).to_vec();
    // if vill_cards[0] == ( Card{suit:Suit::default(), rank: Rank::default()} ){
    //     estimation::calc(&flop_cards, &hero_cards);
    // }else{
    //     equity::calc(&flop_cards, &hero_cards, &vill_cards);   
    // }
}
