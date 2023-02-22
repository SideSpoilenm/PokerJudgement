use std::io::{stdin, stdout, Write};

use crate::module::{equity, estimation};
mod module;
mod range;

#[allow(dead_code)]
fn main() {
    println!("*************** MENU ***************");
    println!("[A] EQ Calculation");
    println!("[B] Estimation of villain's hand");
    print!("type your selection[Index]: ");
    stdout().flush().unwrap(); // 中間領域にある文字を強制的に標準出力
    let mut input: String = String::new();
    stdin().read_line(&mut input).expect("Empty");
    match input.trim() {
        "A" | "a" => equity::run(),
        "B" | "b" => estimation::run(),
        _ => println!("{}", input),
    };
}
