use super::card::{Card, Rank, Suit, self};

pub fn boardmap(flop_cards: &[Card], hero_cards: &[Card], vill_cards: &[Card]) -> Vec<Vec<Card>> {
    let mut board_map: Vec<Vec<Card>> = Vec::new();
    // フロップとプレイヤーのカードを除いた残りのスタブ
    let distributed_cards = [flop_cards, hero_cards, vill_cards].concat();

    // フロップとプレイヤーのカードを除いた残りのスタブ
    let mut deck: Vec<Card> = Vec::new();
    for s in card::SUITS {
        for r in card::RANKS {
            let card = Card { rank: r, suit: s };
            if !(distributed_cards.iter().any(|&c| c == card)) {
                deck.push(card);
            }
        }
    }
    if flop_cards[0]
        == (Card {
            suit: Suit::Joker,
            rank: Rank::X,
        })
    {
        // プリフロップの場合
        for i in 0..(deck.len() - 4) {
            for j in (i + 1)..(deck.len() - 3) {
                for k in (j + 1)..(deck.len() - 2) {
                    for l in (k + 1)..(deck.len() - 1) {
                        for m in (l + 1)..(deck.len()) {
                            board_map.push(vec![deck[i], deck[j], deck[k], deck[l], deck[m]]);
                        }
                    }
                }
            }
        }
    } else {
        // フロップが出ている場合
        for i in 0..(deck.len() - 1) {
            for j in (i + 1)..deck.len() {
                board_map.push([flop_cards, &[deck[i], deck[j]]].concat());
            }
        }
    }
    board_map
}

pub fn combomap(in_cards: &[Card]) -> Vec<Vec<Card>> {
    let mut combomap: Vec<Vec<Card>> = Vec::new(); // 組合せを記録するマップ
    let mut cards = in_cards.to_vec(); // 入力されたカード
                                       // 手札をソートする
    cards.sort_by(|a, b| b.rank.value().cmp(&a.rank.value()));

    // 手札が7枚のとき
    match cards.len() {
        7 => {
            for a in (1..7).rev() {
                for b in (0..a).rev() {
                    let mut row: Vec<Card> = Vec::new();
                    for i in 0..7 {
                        if (a != i) && (b != i) {
                            row.push(cards[i]);
                        };
                    }
                    combomap.push(row);
                }
            }
        }
        // 手札が6枚のとき
        6 => {
            // 除外するカード c を昇順にずらす
            for c in (0..6).rev() {
                let mut row: Vec<Card> = Vec::new();
                for i in 0..6 {
                    if c != i {
                        row.push(cards[i]);
                    };
                }
                combomap.push(row);
            }
        }
        // 手札が5枚のとき
        5 => {
            // 5枚をマップに追加
            combomap.push(cards.to_vec());
        }
        _ => {
            // Err
        }
    }
    combomap
}
