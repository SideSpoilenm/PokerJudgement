use super::card::{Card, Rank};
use super::handrank::HandRank;

#[allow(unused)]
pub fn run(combomap: &[Vec<Card>]) -> (HandRank, Vec<Card>){
    let mut handrank: HandRank =  HandRank::default();      // 判定中の役
    let mut hand = Vec::new();                   // 判定中の手札
    let mut handrankmap: Vec<(HandRank, &Vec<Card>)> = Vec::new();  // 判定中の役と手札を格納するマップ

    // 組合せマップから組合せを取得する
    for combo in combomap{
        let mut flush_flg:bool = true;      // フラッシュのフラグ
        let mut straight_flg: bool = true;  // ストレートのフラグ
        // ヒット系の判定
        if combo[0].rank == combo[1].rank{
            if combo[1].rank == combo[2].rank{
                if combo[2].rank == combo[3].rank{
                    handrankmap.push((HandRank::Quads { set: combo[0].rank } , &combo));
                }else{
                    if combo[3].rank == combo[4].rank{
                        handrankmap.push((HandRank::Fullhouse { set: combo[0].rank, pair: combo[3].rank } , &combo));
                    }else{
                        handrankmap.push((HandRank::Trips { set: combo[0].rank }, &combo));
                    }
                }
            }else{
                if combo[2].rank == combo[3].rank{
                    if combo[3].rank == combo[4].rank{
                        handrankmap.push((HandRank::Fullhouse { set: combo[2].rank, pair: combo[0].rank } , &combo));
                    }else{
                        handrankmap.push((HandRank::TwoPair { pair1: combo[0].rank, pair2: combo[2].rank } , &combo));
                    }
                }else{
                    if combo[3].rank == combo[4].rank{
                        handrankmap.push((HandRank::TwoPair { pair1: combo[0].rank, pair2: combo[3].rank } , &combo));
                    }else{
                        handrankmap.push((HandRank::OnePair { pair: combo[0].rank } , &combo));
                    }
                }
            }
        }else{
            if combo[1].rank == combo[2].rank{
                if combo[2].rank == combo[3].rank{
                    if combo[3].rank == combo[4].rank{
                        handrankmap.push((HandRank::Quads { set: combo[1].rank } , &combo));
                    }else{
                        handrankmap.push((HandRank::Trips { set: combo[1].rank } , &combo));
                    }
                }else{
                    if combo[3].rank == combo[4].rank{
                        handrankmap.push((HandRank::TwoPair { pair1: combo[1].rank, pair2: combo[3].rank } , &combo));
                    }else{
                        handrankmap.push((HandRank::OnePair { pair: combo[1].rank } , &combo));
                    }
                }
            }else{
                if combo[2].rank == combo[3].rank{
                    if combo[3].rank == combo[4].rank{
                        handrankmap.push((HandRank::Trips { set: combo[2].rank } , &combo));
                    }else{
                            handrankmap.push((HandRank::OnePair { pair: combo[2].rank } , &combo));
                    }
                }else{
                    if combo[3].rank == combo[4].rank{
                        handrankmap.push((HandRank::OnePair { pair: combo[3].rank } , &combo));
                    }else{
                        // ストレートの判定
                        if (combo[0].rank == Rank::Ace)&&(combo[1].rank == Rank::Five){
                            for i in 2..5{
                                straight_flg = ((combo[i].rank.value() + 1) == combo[i-1].rank.value()) 
                                                    && straight_flg;
                            }
                        }else{
                            for i in 1..5{
                                straight_flg = ((combo[i].rank.value() + 1) == combo[i-1].rank.value()) 
                                                    && straight_flg;
                            }
                        }

                        // フラッシュの判定
                        for i in 1..5{
                            flush_flg = (combo[i].suit == combo[0].suit) && flush_flg;
                        }
                        // ロイヤルストレートフラッシュの判定
                        if straight_flg && flush_flg{
                            if combo[4].rank == Rank::Ten{
                                handrankmap.push((HandRank::Royal { suit: combo[0].suit }, &combo));
                            }else{
                                // ストレートフラッシュの判定
                                if (combo[0].rank == Rank::Ace)&&(combo[1].rank == Rank::Five){
                                    handrankmap.push((HandRank::StFlush { initial: Rank::Five, suit: combo[0].suit }, &combo));
                                }else{
                                    handrankmap.push((HandRank::StFlush { initial: combo[0].rank, suit: combo[0].suit }, &combo));
                                }
                            }
                        }else if straight_flg {
                            if (combo[0].rank == Rank::Ace)&&(combo[1].rank == Rank::Five){
                                handrankmap.push((HandRank::Straight { initial: Rank::Five }, &combo));
                            }else{
                                handrankmap.push((HandRank::Straight { initial: combo[0].rank }, &combo));
                            }
                        }else if flush_flg {
                            handrankmap.push((HandRank::Flush { suit: combo[0].suit }, &combo));
                        }else {
                            // ハイカード
                            if handrankmap.is_empty(){  // 一番強い最初のみ格納
                                handrankmap.push((HandRank::High, &combo));
                            }
                        }
                    }
                }
            }
        }
    }
    // 役とハンドのマップから最も強い役とハンドを探す
    for (hr, h) in handrankmap{
        if handrank.value() < hr.value(){
            handrank = hr;
            hand = h.to_vec();
        }
    } 
    (handrank, hand)  
}
