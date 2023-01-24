mod module;
use std::io::{stdin, stdout, Write};
use module::card::{Card};
use module::{convert, combination, judgement};

#[allow(dead_code)]
fn main() {
    let mut cards: Vec<Card> = Vec::new();          // 手札

    println!("*************** CARDS ***************");
    for i in 0..7{
        print!("Card{}: ", i+1);
        stdout().flush().unwrap();  // 中間領域にある文字を強制的に標準出力
        let mut text: String = String::new();
        stdin().read_line(&mut text).expect("Empty");
        cards.push(convert::lavel_to_card(text));
    }

    // 手札の組合せを記録するマップ
    let mut combomap = combination::run(&mut cards);

    let result = judgement::run(&mut combomap);

    println!("*************** RESULT ***************");
    println!("HandRank: {:?}", result.0);
    print!("Hand: ");
    for c in result.1{
        print!("{} ", c.to_lavel());
    }
}
