use crate::card::Card;
#[derive(Debug, PartialEq)]
pub struct PlayerHand {
    pub hand: Vec<String>,
    pub score: Vec<u32>,
}
#[derive(Debug, PartialEq)]
pub struct Hand {
    pub cards: Vec<String>,
    pub score: f64,
    pub description: String,
}
#[derive(Debug, PartialEq)]
pub struct Board {
    pub flop: Vec<String>,
    pub turn: String,
    pub river: String,
}

#[derive(Debug, PartialEq)]
pub struct Deal {
    pub board: Board,
    pub hands: Vec<Hand>,
}