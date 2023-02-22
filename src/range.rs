pub mod co;
pub mod utg_open;

use super::module::convert;
use super::module::card::{Card, SUITS};

pub fn make(orders: &[String]) -> Vec<Vec<Card>> {
    let mut range: Vec<Vec<Card>> = Vec::new();
    for text in orders {
        // ex. "AKo" -> rank1: "A", rank2: "K", suited: "o"
        let mut t = text.chars();
        let rank1 = convert::str_to_rank(&t.nth(0).unwrap().to_string());
        let rank2 = convert::str_to_rank(&t.nth(0).unwrap().to_string());
        let suited:&str = &t.nth(0).unwrap_or('p').to_string();

        match suited{
            "s" => {  // スーテット
                for suit in SUITS{
                    range.push(vec![Card{suit: suit, rank: rank1}, Card{suit: suit, rank: rank2}]);
                }
            },
            "o" => {  // オフスーテット
                for suit1 in SUITS{
                    for suit2 in SUITS{
                        if suit1 != suit2{
                            range.push(vec![Card{suit: suit1, rank: rank1}, Card{suit: suit2, rank: rank2}])
                        }
                    }
                }
            },
            "p" => {
                if rank1==rank2 {   // ポケット
                    for i in 0..(SUITS.len() - 1){
                        for j in (i + 1)..SUITS.len(){
                            range.push(vec![Card{suit: SUITS[i], rank: rank1}, Card{suit: SUITS[j], rank: rank2}])
                        }
                    }
                }else{
                    //err
                }
            },
            _ =>{
                // err
            }
        }
    }
    range
}
