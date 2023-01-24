use super::card::{Card, Rank, Suit};
use super::handrank::HandRank;
use super::{combination};

#[allow(dead_code)]
#[derive(Clone, Copy, PartialEq, Debug)]
enum Holder{
    Board,
    Player1,
    Player2,
    Player3,
}

fn make_map(hands: &[(Holder, Vec<Card>)]) {
    let mut all_flg = true;     // ホールカードのみでEQを出す場合
    let mut holders: Vec<(Holder, Vec<Card>)> = Vec::new();
    let mut combomap: Vec<(Holder, Vec<Vec<Card>>)> = Vec::new();

    // ボードの有無とホールカードの確認
    for (holder, hand) in hands.to_vec(){
        all_flg = (holder != Holder::Board) && all_flg;
        holders.push((holder, hand));
    }
    // ボードとプレイヤーのカードを除いた残りのスタブ
    let mut deck: Vec<Card> = Vec::new();
    for s in [Suit::Spade, Suit::Heart, Suit::Diamond, Suit::Club] {
        for r in [Rank::Deuce, Rank::Three, Rank::Four, Rank::Five, Rank::Six,Rank::Seven, 
                        Rank::Eight, Rank::Nine, Rank::Ten, Rank::Jack, Rank::Queen, Rank::King, Rank::Ace]{
            let card = Card{rank: r, suit: s};
            for holder in &holders{
                if holder.1.iter().any(|&c| c == Card{rank:r, suit:s}){
                    deck.push(card);
                }
            }
        }
    }

    if all_flg{
        let mut all_map:Vec<Vec<Card>>= Vec::new();
        for i in 0..deck.len(){
            for j in 1..deck.len(){
                for k in 2..deck.len(){
                    for l in 4..deck.len(){
                        for m in 5..deck.len(){
                            all_map.push(vec![deck[i], deck[j], deck[k], deck[l], deck[m]]);
                        }
                    }
                }
            }
        }
        for (holder , hand) in &holders{
            for row in &all_map{
                let mut cards: Vec<Card> = Vec::new();
                cards.extend(hand);
                cards.extend(row);
                let map = combination::run(&mut cards);
                combomap.push((holder.clone(), map));
            }
        }
    }

}

fn calc(hands: &[(Holder, Vec<Card>)]) -> Vec<f32> {
    let mut eq:Vec<f32> = Vec::new();
    make_map(hands);
    eq
}
