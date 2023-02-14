use super::card::{Card, Rank, Suit};
use super::handrank::HandRank;
use super::{combination, judgement};

#[allow(dead_code)]

fn make_boardmap(flop_cards: &[Card], hero_cards: &[Card], vill_cards: &[Card]) -> Vec<Vec<Card>> {
    let mut board_map: Vec<Vec<Card>> = Vec::new();
    // フロップとプレイヤーのカードを除いた残りのスタブ
    let distributed_cards = [flop_cards, hero_cards, vill_cards].concat();

    // フロップとプレイヤーのカードを除いた残りのスタブ
    let mut deck: Vec<Card> = Vec::new();
    for s in [Suit::Spade, Suit::Heart, Suit::Diamond, Suit::Club] {
        for r in [
            Rank::Deuce,
            Rank::Three,
            Rank::Four,
            Rank::Five,
            Rank::Six,
            Rank::Seven,
            Rank::Eight,
            Rank::Nine,
            Rank::Ten,
            Rank::Jack,
            Rank::Queen,
            Rank::King,
            Rank::Ace,
        ] {
            let card = Card { rank: r, suit: s };
            if !(distributed_cards.iter().any(|&c| c == card)) {
                deck.push(card);
            }
        }
    }
    for i in 0..(deck.len() - 1) {
        for j in (i + 1)..deck.len() {
            board_map.push([flop_cards, &[deck[i], deck[j]]].concat());
        }
    }
    board_map
}

pub fn run(flop_cards: &[Card], hero_cards: &[Card], vill_cards: &[Card]) -> (f32, f32) {
    let mut hero_eq: f32 = 0.0;
    let mut vill_eq: f32 = 0.0;

    let boardmap = make_boardmap(&flop_cards, &hero_cards, &vill_cards);
    let winscore: f32 = 1.0 / (boardmap.len() as f32);
    let chopscore: f32 = (1.0 / 2.0) / (boardmap.len() as f32); // 人数が増えると変わる

    for board in &boardmap {
        let hero_combomap = combination::run(&mut [&board, hero_cards].concat());
        let hero_result = judgement::run(&hero_combomap);
        let vill_combomap = combination::run(&mut [&board, vill_cards].concat());
        let vill_result = judgement::run(&vill_combomap);

        if vill_result.0.value() < hero_result.0.value() {
            hero_eq += winscore;
        } else if hero_result.0.value() < vill_result.0.value() {
            vill_eq += winscore;
        } else {
            // 役の情報(役によって異なる)
            let hero_info = hero_result.0.info();
            let vill_info = vill_result.0.info();
            // キッカーを抽出
            let mut hero_kicker: Vec<Card> = hero_result.1.clone();
            let mut vill_kicker: Vec<Card> = vill_result.1.clone();
            for i in 0..hero_info.len(){
                hero_kicker.retain(|&c| c.rank != hero_info[i].rank );
                vill_kicker.retain(|&c| c.rank != vill_info[i].rank );
            }
            
            match hero_result.0 {
                HandRank::High => {
                    for i in 0..5 {
                        if vill_result.1[i].rank.value() < hero_result.1[i].rank.value() {
                            hero_eq += winscore;
                            break;
                        } else if vill_result.1[i].rank.value() > hero_result.1[i].rank.value() {
                            vill_eq += winscore;
                            break;
                        } else if i == 4 {
                            hero_eq += chopscore;
                            vill_eq += chopscore;
                        }
                    }
                }
                HandRank::OnePair { pair: _ } => {
                    // ペアのランクを比較
                    if vill_info[0].rank.value() < hero_info[0].rank.value() {
                        hero_eq += winscore;
                    } else if vill_info[0].rank.value() > hero_info[0].rank.value() {
                        vill_eq += winscore;
                    } else {
                        // キッカーを比較
                        for i in 0..3 {
                            if vill_kicker[i].rank.value() < hero_kicker[i].rank.value() {
                                hero_eq += winscore;
                                break;
                            } else if vill_kicker[i].rank.value() > hero_kicker[i].rank.value() {
                                vill_eq += winscore;
                                break;
                            } else if i == 2 {
                                hero_eq += chopscore;
                                vill_eq += chopscore;
                            }
                        }
                    }
                }
                HandRank::TwoPair { pair1: _, pair2: _ } => {
                    // ペアのランクを比較
                    if vill_info[0].rank.value() < hero_info[0].rank.value() {
                        hero_eq += winscore;
                    } else if vill_info[0].rank.value() > hero_info[0].rank.value() {
                        vill_eq += winscore;
                    } else {
                        if vill_info[1].rank.value() < hero_info[1].rank.value() {
                            hero_eq += winscore;
                        } else if vill_info[1].rank.value() > hero_info[1].rank.value() {
                            vill_eq += winscore;
                        } else {
                            // キッカーを比較
                            if vill_kicker[0].rank.value() < hero_kicker[0].rank.value() {
                                hero_eq += winscore;
                                break;
                            } else if vill_kicker[0].rank.value() > hero_kicker[0].rank.value() {
                                vill_eq += winscore;
                                break;
                            } else {
                                hero_eq += chopscore;
                                vill_eq += chopscore;
                            }
                        }
                    }
                }
                HandRank::Trips { set: _ } => {
                    // ペアのランクを比較
                    if vill_info[0].rank.value() < hero_info[0].rank.value() {
                        hero_eq += winscore;
                    } else if vill_info[0].rank.value() > hero_info[0].rank.value() {
                        vill_eq += winscore;
                    } else {
                        // キッカーを比較
                        for i in 0..2 {
                            if vill_kicker[i].rank.value() < hero_kicker[i].rank.value() {
                                hero_eq += winscore;
                                break;
                            } else if vill_kicker[i].rank.value() > hero_kicker[i].rank.value() {
                                vill_eq += winscore;
                                break;
                            } else if i == 1 {
                                hero_eq += chopscore;
                                vill_eq += chopscore;
                            }
                        }
                    }
                }
                HandRank::Straight { initial: _ } => {
                    if vill_info[0].rank.value() < hero_info[0].rank.value() {
                        hero_eq += winscore;
                    } else if vill_info[0].rank.value() > hero_info[0].rank.value() {
                        vill_eq += winscore;
                    } else {
                        hero_eq += chopscore;
                        vill_eq += chopscore;
                    }
                }
                HandRank::Flush { suit: _ } => {
                    for i in 0..5 {
                        if vill_result.1[i].rank.value() < hero_result.1[i].rank.value() {
                            hero_eq += winscore;
                            break;
                        } else if vill_result.1[i].rank.value() > hero_result.1[i].rank.value() {
                            vill_eq += winscore;
                            break;
                        } else if i == 4 {
                            hero_eq += chopscore;
                            vill_eq += chopscore;
                        }
                    }
                }
                HandRank::Fullhouse { set: _, pair: _ } => {
                    // セット部分の比較
                    if vill_info[0].rank.value() < hero_info[0].rank.value() {
                        hero_eq += winscore;
                    } else if vill_info[0].rank.value() > hero_info[0].rank.value() {
                        vill_eq += winscore;
                    } else {
                        // ペアの比較
                        if vill_info[1].rank.value() < hero_info[1].rank.value() {
                            hero_eq += winscore;
                        } else if vill_info[1].rank.value() > hero_info[1].rank.value() {
                            vill_eq += winscore;
                        } else {
                            hero_eq += chopscore;
                            vill_eq += chopscore;
                        }
                    }
                }
                HandRank::Quads { set: _ } => {
                    // セットのランクを比較
                    if vill_info[0].rank.value() < hero_info[0].rank.value() {
                        hero_eq += winscore;
                    } else if vill_info[0].rank.value() > hero_info[0].rank.value() {
                        vill_eq += winscore;
                    } else {
                        // キッカーを比較
                        if vill_kicker[0].rank.value() < hero_kicker[0].rank.value() {
                            hero_eq += winscore;
                            break;
                        } else if vill_kicker[0].rank.value() > hero_kicker[0].rank.value() {
                            vill_eq += winscore;
                            break;
                        } else {
                            hero_eq += chopscore;
                            vill_eq += chopscore;
                        }
                    }
                }
                HandRank::StFlush {
                    initial: _,
                    suit: _,
                } => {
                    if vill_info[0].rank.value() < hero_info[0].rank.value() {
                        hero_eq += winscore;
                    } else if vill_info[0].rank.value() > hero_info[0].rank.value() {
                        vill_eq += winscore;
                    } else {
                        hero_eq += chopscore;
                        vill_eq += chopscore;
                    }
                }
                HandRank::Royal { suit: _ } => {
                    hero_eq += chopscore;
                    vill_eq += chopscore;
                } 
            }
        }
    }
    ((hero_eq * 100.0), (vill_eq * 100.0))
}
