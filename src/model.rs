use crate::card::Card;

pub struct PlayerHand {
    pub hand: Vec<String>,
    pub score: Vec<u32>,
}

pub struct Hand {
    pub cards: Vec<String>,
    pub score: f64,
    pub description: String,
}

pub struct Board {
    pub flop: Vec<String>,
    pub turn: String,
    pub river: String,
}

pub struct HandResponse {
    pub board: Board,
    pub hands: Vec<Hand>,
}