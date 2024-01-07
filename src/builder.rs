use rand::Rng;

use crate::{evaluator::Evaluator, card::Card, model::{HandResponse, PlayerHand}};

static CARDS: &'static [&'static str; 52] = &[
    "Ac", "Ad", "Ah", "As", "2c", "2d", "2h", "2s", "3c", "3d", "3h", "3s", "4c", "4d", "4h", "4s",
    "5c", "5d", "5h", "5s", "6c", "6d", "6h", "6s", "7c", "7d", "7h", "7s", "8c", "8d", "8h", "8s",
    "9c", "9d", "9h", "9s", "Tc", "Td", "Th", "Ts", "Jc", "Jd", "Jh", "Js", "Qc", "Qd", "Qh", "Qs",
    "Kc", "Kd", "Kh", "Ks",
];

fn shuffle() -> Vec<&'static str> {
    let mut rng = rand::thread_rng();
    let mut sample = CARDS.to_vec();
    for i in 0..CARDS.len() {
        let rand: usize = rng.gen_range(0..=i);
        let temp = sample[i];
        sample[i] = sample[rand];
        sample[rand] = temp;
    }
    sample
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

fn build_deal_payload(player_count: usize) {
    let evaluator = Evaluator::new();
    let mut cards = shuffle();
    let mut hands: Vec<PlayerHand> = Vec::new();
    let mut nextn = IndexGenerator::new();
    let mut i = 0;
    let players: Vec<_> = (0..player_count).collect();
    let pl = players.len();
    for p in players {
        i = nextn.next().unwrap();
        let player_hand = vec![cards[i], cards[i + pl]];
        let scores: Vec<_> = player_hand.iter().map(|&card| Card::new(card)).collect();
    }

    let flop = vec![
        cards[nextn.next().unwrap() + pl],
        cards[nextn.next().unwrap() + pl],
        cards[nextn.next().unwrap() + pl],
    ];

    let flop_score: Vec<_> = flop.iter().map(|&card| Card::new(card)).collect();
    let turn = cards[nextn.next().unwrap() + pl];
    let turn_score = Card::new(turn);
    let river = cards[nextn.next().unwrap() + pl];
    let river_score = Card::new(river);
    let mut player_hands: Vec<HandResponse> = Vec::new();
    for hand in &mut hands {
        let mut combined_score = flop_score.clone();
        combined_score.push(turn_score);
        combined_score.push(river_score);
        let hand_score = evaluator.evaluate(combined_score, hand.score);
        let score = evaluator.get_rank_class(hand_score);
        let percentage = 1.0 - evaluator.get_five_card_rank_percentage(hand_score);
        let description = evaluator.class_to_string(score);
        player_hands.push(("score", percentage), ("description", description));
    }
}
