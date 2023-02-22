use super::card::{Card, Rank, Suit};
use super::dealer::{Play, Street, Action};
use super::{combination, convert, judgement};
use std::io::{stdin, stdout, Write};

const HR: [&str; 10] = [
    "High",
    "OnePair",
    "TwoPair",
    "Trips",
    "Straight",
    "Flush",
    "Fullhouse",
    "Quads",
    "StFlush",
    "Royal",
];

#[allow(dead_code)]
pub fn run() {
    let mut cards: Vec<Card> = Vec::new();
    println!("*************** CARDS ***************");
    for label in [
        "hero's 1st card: ",
        "hero's 2nd card: ",
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
    let flop_cards: Vec<Card> = (&cards[2..5]).to_vec();
    // 相手のポジションの指定
    print!("villain's position: ");
    stdout().flush().unwrap(); // 中間領域にある文字を強制的に標準出力
    let mut text: String = String::new();
    stdin().read_line(&mut text).expect("Empty");
    let vill_posi = convert::str_to_position(&text.trim());

    calc(&flop_cards, &hero_cards, Play{street:Street::Preflop, position:vill_posi, action:Action::Raise});
}

#[allow(dead_code)]
fn calc(flop_cards: &[Card], hero_cards: &[Card], vill_play: Play) {
    let mut abilities: Vec<[f64; HR.len()]> = Vec::new();
    // let mut hands: Vec<Vec<Card>> = CO_RANGE.to_vec().iter().map(|s| s.to_vec()).collect();
    let mut hands = vill_play.range();
    // 相手のレンジから自分のカードを含むものを除外
    hands.retain(|v| !(v.contains(&hero_cards[0]) || v.contains(&hero_cards[1])));
    //CO_RANGE.to_vec();
    for vill_cards in &hands {
        let mut ability = [0.0; HR.len()];
        let boardmap = combination::boardmap(
            &flop_cards,
            &[
                Card {
                    suit: Suit::default(),
                    rank: Rank::default(),
                },
                Card {
                    suit: Suit::default(),
                    rank: Rank::default(),
                },
            ],
            &vill_cards,
        );
        for board in &boardmap {
            let board: &[Card] = &board;
            let vill_cards: &[Card] = &vill_cards;
            let vill_combomap = combination::combomap(&mut [board, vill_cards].concat());
            let vill_result = judgement::run(&vill_combomap);
            ability[vill_result.0.value() as usize] += 1.0 / (boardmap.len() as f64);
        }
        abilities.push(ability);
    }
    println!("*************** ESTIMATION ***************");
    anlysis(&hands, &abilities);
}

fn anlysis(hands: &[Vec<Card>], abilities: &[[f64; HR.len()]]) {
    let mut flush_hands: Vec<Vec<Card>> = Vec::new();
    let mut straight_hands: Vec<Vec<Card>> = Vec::new();
    let mut hit_hands: Vec<Vec<Card>> = Vec::new();
    let mut other_hands: Vec<Vec<Card>> = Vec::new();
    for i in 0..hands.len() {
        // フラッシュ系
        // フラッシュ・ストフラ・ロイヤルの確率を足す
        let flush_ability = abilities[i][5] + abilities[i][8] + abilities[i][9];
        // ストレート系
        let straight_ability = abilities[i][4];
        // セット系
        // トリップス・フルハウス・クワッズの確率を足す
        let hit_ability = abilities[i][3] + abilities[i][6] + abilities[i][7];

        if flush_ability > 0.16 {
            // フラッシュドロー 36%
            flush_hands.push(hands[i].to_vec());
        } else if straight_ability > 0.16 {
            // ガットショットストレートドロー 16%
            straight_hands.push(hands[i].to_vec());
        } else if hit_ability > 0.08 {
            // ワンペアがセットになる   8.4%
            hit_hands.push(hands[i].to_vec());
        } else {
            other_hands.push(hands[i].to_vec());
        }
    }
    let flush_prop = (flush_hands.len() as f32) / (hands.len() as f32) * 100.0;
    let straight_prop = (straight_hands.len() as f32) / (hands.len() as f32) * 100.0;
    let hit_prop = (hit_hands.len() as f32) / (hands.len() as f32) * 100.0;
    let other_prop = (other_hands.len() as f32) / (hands.len() as f32) * 100.0;

    let flushhands_label: Vec<String> = flush_hands
        .iter()
        .map(|hand| convert::hand_to_label(hand))
        .collect();
    let straighthands_label: Vec<String> = straight_hands
        .iter()
        .map(|hand| convert::hand_to_label(hand))
        .collect();
    let hithands_label: Vec<String> = hit_hands
        .iter()
        .map(|hand| convert::hand_to_label(hand))
        .collect();
    let otherhands_label: Vec<String> = other_hands
        .iter()
        .map(|hand| convert::hand_to_label(hand))
        .collect();

    println!("Flush Draw\t{}%", flush_prop);
    println!("Straight Draw\t{}%", straight_prop);
    println!("Hit Hand\t{}%", hit_prop);
    println!("Other Hand\t{}%", other_prop);
    println!("*******************************************");
    println!("Flush Draw\t: {:?}", flushhands_label);
    println!("Straight Draw\t: {:?}", straighthands_label);
    println!("Hit Hand\t: {:?}", hithands_label);
    println!("Other\t:{:?}", otherhands_label);
}
