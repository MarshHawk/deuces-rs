use rand::Rng;

use crate::{
    card::Card,
    evaluator::Evaluator,
    model::{Board, Deal, Hand, PlayerHand},
};

static CARDS: [&str; 52] = [
    "Ac", "Ad", "Ah", "As", "2c", "2d", "2h", "2s", "3c", "3d", "3h", "3s", "4c", "4d", "4h", "4s",
    "5c", "5d", "5h", "5s", "6c", "6d", "6h", "6s", "7c", "7d", "7h", "7s", "8c", "8d", "8h", "8s",
    "9c", "9d", "9h", "9s", "Tc", "Td", "Th", "Ts", "Jc", "Jd", "Jh", "Js", "Qc", "Qd", "Qh", "Qs",
    "Kc", "Kd", "Kh", "Ks",
];

// Define the interface
pub trait CardShuffler {
    fn shuffle(&self) -> Vec<&'static str>;
}

// Implement the interface for a struct
pub struct RandomCardShuffler;

impl CardShuffler for RandomCardShuffler {
    fn shuffle(&self) -> Vec<&'static str> {
        let mut rng = rand::thread_rng();
        let mut sample = CARDS.clone();
        for i in 0..CARDS.len() {
            let rand: usize = rng.gen_range(0..=i);
            sample.swap(i, rand);
        }
        sample.to_vec()
    }
}

struct IndexGenerator {
    index: usize,
}

impl IndexGenerator {
    fn new() -> IndexGenerator {
        IndexGenerator { index: 0 }
    }
}

impl Iterator for IndexGenerator {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        let current_index = self.index;
        self.index += 1;
        Some(current_index)
    }
}

pub trait Dealer {
    fn deal(&self, player_count: usize) -> Deal;
}

pub struct GameDealer<S: CardShuffler> {
    shuffler: S,
}

impl<S: CardShuffler> GameDealer<S> {
    #[allow(dead_code)]
    pub fn new(shuffler: S) -> Self {
        GameDealer { shuffler }
    }
}

impl<S: CardShuffler> Dealer for GameDealer<S> {
    fn deal(&self, player_count: usize) -> Deal {
        let evaluator = Evaluator::new();
        let cards = self.shuffler.shuffle();
        let mut hands: Vec<PlayerHand> = Vec::new();
        let mut nextn = IndexGenerator::new();
        let players: Vec<_> = (0..player_count).collect();
        let pl = players.len();
        for _ in players {
            let i = nextn.next().unwrap();
            let player_hand: Vec<String> = vec![cards[i].to_string(), cards[i + pl].to_string()];
            let scores: Vec<u32> = player_hand.iter().map(|card| Card::new(card).0).collect();
            hands.push(PlayerHand {
                hand: player_hand,
                score: scores,
            });
        }
        let flop = vec![
            cards[nextn.next().unwrap() + pl].to_string(),
            cards[nextn.next().unwrap() + pl].to_string(),
            cards[nextn.next().unwrap() + pl].to_string(),
        ];
        let flop_score: Vec<u32> = flop.iter().map(|card| Card::new(card.as_str()).0).collect();
        let turn = cards[nextn.next().unwrap() + pl];
        let turn_score = Card::new(turn).0;
        let river = cards[nextn.next().unwrap() + pl];
        let river_score = Card::new(river).0;
        let mut player_hands: Vec<Hand> = Vec::new();
        for hand in hands {
            let mut combined_score = flop_score.clone();
            combined_score.push(turn_score);
            combined_score.push(river_score);
            let hand_score = evaluator.evaluate(hand.score, combined_score);
            let score = evaluator.get_rank_class(hand_score);
            let percentage = 1.0 - evaluator.get_five_card_rank_percentage(hand_score);
            let description = evaluator.class_to_string(score.unwrap());
            player_hands.push(Hand {
                cards: hand.hand.clone(),
                score: percentage,
                description,
            });
        }
        let board = Board {
            flop: flop.clone(),
            turn: turn.to_string(),
            river: river.to_string(),
        };
        Deal {
            board,
            hands: player_hands,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct MockCardShuffler;

    impl CardShuffler for MockCardShuffler {
        fn shuffle(&self) -> Vec<&'static str> {
            CARDS.to_vec()
        }
    }

    #[test]
    fn test_deal() {
        let expected_deal = Deal {
            board: Board {
                flop: vec!["2h".to_string(), "2s".to_string(), "3c".to_string()],
                turn: "3d".to_string(),
                river: "3h".to_string(),
            },
            hands: vec![
                Hand {
                    cards: vec!["Ac".to_string(), "As".to_string()],
                    score: 0.9599303135888502,
                    description: "Full House".to_string(),
                },
                Hand {
                    cards: vec!["Ad".to_string(), "2c".to_string()],
                    score: 0.9584561779683731,
                    description: "Full House".to_string(),
                },
                Hand {
                    cards: vec!["Ah".to_string(), "2d".to_string()],
                    score: 0.9584561779683731,
                    description: "Full House".to_string(),
                },
            ],
        };
        let mock_shuffler = MockCardShuffler;
        let dealer = GameDealer::new(mock_shuffler);
        let deal = dealer.deal(3);
        assert!(deal == expected_deal);
    }
}
