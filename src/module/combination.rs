use super::card::Card;

pub fn run(in_cards: &[Card]) -> Vec<Vec<Card>> {
    let mut combomap: Vec<Vec<Card>> = Vec::new(); // 組合せを記録するマップ
    let mut cards = in_cards.to_vec(); // 入力されたカード
                                       // 手札をソートする
    cards.sort_by(|a, b| b.rank.value().cmp(&a.rank.value()));
    //println!("total cards: {:?}", cards);

    // 手札が5枚以上あるときのみ処理する
    if cards.len() >= 5 {
        // 手札が5枚のとき
        if cards.len() == 5 {
            // 5枚をマップに追加
            combomap.push(cards.to_vec());
        } else
        // 手札が6枚のとき
        if cards.len() == 6 {
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
        } else
        // 手札が7枚のとき
        if cards.len() == 7 {
            //let mut bp = 6;      // 除外するカード b をずらすスタート位置
            for a in (1..=6).rev() {
                //bp -= 1;     // b のスタート位置 p をずらす
                //for b in (0..=bp).rev(){
                for b in (0..=(a - 1)).rev() {
                    let mut row: Vec<Card> = Vec::new();
                    for i in 0..7 {
                        if (a != i) && (b != i) {
                            row.push(cards[i]);
                        };
                    }
                    combomap.push(row);
                }
            }
        };
    } else {
        //
    };
    combomap
}
