use super::card::Card;
use super::handrank::HandRank;
use super::{combination, judgement};

#[allow(dead_code)]
pub fn calc(flop_cards: &[Card], hero_cards: &[Card], vill_cards: &[Card]){
    let mut hero_eq: f64 = 0.0;
    let mut vill_eq: f64 = 0.0;

    let boardmap = combination::boardmap(&flop_cards, &hero_cards, &vill_cards);
    let winscore: f64 = 1.0 / (boardmap.len() as f64);
    let chopscore = winscore / 2.0; // 参加プレイヤーの数

    for board in &boardmap {
        let hero_combomap = combination::combomap(&mut [&board, hero_cards].concat());
        let hero_result = judgement::run(&hero_combomap);
        let vill_combomap = combination::combomap(&mut [&board, vill_cards].concat());
        let vill_result = judgement::run(&vill_combomap);

        if vill_result.0.value() < hero_result.0.value() {
            hero_eq += winscore;
        } else if vill_result.0.value() > hero_result.0.value() {
            vill_eq += winscore;
        } else
        // 役が同じ場合
        // ボードチョップの場合
        if hero_result.1 == vill_result.1 {
            hero_eq += chopscore;
            vill_eq += chopscore;
        } else {
            match hero_result.0 {
                HandRank::High | HandRank::Flush { suit:_ } => {
                    for i in 0..5 {
                        if vill_result.1[i].rank.value() < hero_result.1[i].rank.value() {
                            hero_eq += winscore;
                            break;
                        } else if vill_result.1[i].rank.value() > hero_result.1[i].rank.value() {
                            vill_eq += winscore;
                            break;
                        } else {
                            if i == 4 {
                                hero_eq += chopscore;
                                vill_eq += chopscore;
                            }
                        }
                    }
                }
                HandRank::OnePair { pair: _ }
                | HandRank::TwoPair { pair1: _, pair2: _ }
                | HandRank::Trips { set: _ }
                | HandRank::Straight { initial:_ }
                | HandRank::Fullhouse { set: _, pair: _ }
                | HandRank::Quads { set: _ }
                | HandRank::StFlush { initial:_, suit:_ } => {
                    // 役の情報(役によって異なる)
                    let hero_info = hero_result.0.info();
                    let vill_info = vill_result.0.info();
                    let mut check_kicker: bool = false;

                    // ペアのランクを比較
                    for i in 0..hero_info.len() {
                        if vill_info[i].rank.value() < hero_info[i].rank.value() {
                            hero_eq += winscore;
                            break;
                        } else if vill_info[i].rank.value() > hero_info[i].rank.value() {
                            vill_eq += winscore;
                            break;
                        } else if i == (hero_info.len() - 1) {
                            check_kicker = true
                        }
                    }
                    if check_kicker {
                        // キッカーを比較
                        for i in 0..5 {
                            if vill_result.1[i].rank.value() < hero_result.1[i].rank.value() {
                                hero_eq += winscore;
                                break;
                            } else if vill_result.1[i].rank.value() > hero_result.1[i].rank.value()
                            {
                                vill_eq += winscore;
                                break;
                            } else {
                                if i == 4 {
                                    hero_eq += chopscore;
                                    vill_eq += chopscore;
                                }
                            }
                        }
                    }
                }
                HandRank::Royal { suit: _ } => {
                    hero_eq += chopscore;
                    vill_eq += chopscore;
                }
            }
        }
    }
    hero_eq = hero_eq * 100.0;
    vill_eq = vill_eq * 100.0;
    println!("*************** EQUITY ***************");
    print!("hero: {}% \t villain: {}%", hero_eq, vill_eq); 
}
