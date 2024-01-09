use crate::{lookup::LookupTable, card::Card};

pub struct Evaluator {
    table: LookupTable,
    //hand_size_map: HashMap<u8, fn() -> ()>,  // Assuming these functions return nothing
}

impl Evaluator {
    pub fn new() -> Self {
        // let mut hand_size_map = HashMap::new();
        // hand_size_map.insert(5, Self::_five);
        // hand_size_map.insert(6, Self::_six);
        // hand_size_map.insert(7, Self::_seven);

        Self {
            table: LookupTable::new(),
            // hand_size_map,
        }
    }

    pub fn evaluate(&self, cards: Vec<u32>, board: Vec<u32>) -> u32 {
        let mut all_cards = cards;
        all_cards.extend(board);
        self._five(&[all_cards[0], all_cards[1], all_cards[2], all_cards[3], all_cards[4]])
        //match all_cards.len() {
        //    5 => self._five(&[all_cards[0], all_cards[1], all_cards[2], all_cards[3], all_cards[4]]),
        //    6 => self._six(&[all_cards[0], all_cards[1], all_cards[2], all_cards[3], all_cards[4], all_cards[5]]),
        //    7 => self._seven(&[all_cards[0], all_cards[1], all_cards[2], all_cards[3], all_cards[4], all_cards[5], all_cards[6]]),
        //    _ => panic!("Invalid number of cards"),
        //}
    }

    pub fn _five(&self, cards: &[u32; 5]) -> u32 {
        // if flush
        if cards[0] & cards[1] & cards[2] & cards[3] & cards[4] & 0xF000 != 0 {
            let hand_or = (cards[0] | cards[1] | cards[2] | cards[3] | cards[4]) >> 16;
            let prime = Card::prime_product_from_rankbits(hand_or);
            return *self.table.flush_lookup.get(&prime).unwrap();
        }
        // otherwise
        else {
            let prime = Card::prime_product_from_hand(cards);
            return *self.table.unsuited_lookup.get(&prime).unwrap();
        }
    }

/*     pub fn _six(&self, cards: &[u32; 6]) -> u32 {
        let mut minimum = LookupTable::MAX_HIGH_CARD;

        let all5cardcombos: Vec<_> = cards.iter().combinations(5).collect();
        for combo in all5cardcombos {
            let score = self._five(&[combo[0], combo[1], combo[2], combo[3], combo[4]]);
            if score < minimum {
                minimum = score;
            }
        }

        minimum
    }

    pub fn _seven(&self, cards: &[u32; 7]) -> u32 {
        let mut minimum = LookupTable::MAX_HIGH_CARD;

        let all5cardcombos: Vec<_> = cards.iter().combinations(5).collect();
        for combo in all5cardcombos {
            let score = self._five(&[combo[0], combo[1], combo[2], combo[3], combo[4]]);
            if score < minimum {
                minimum = score;
            }
        }

        minimum
    } */

    pub fn get_rank_class(&self, hr: u32) -> Result<u32, &'static str> {
        if hr >= 0 && hr <= LookupTable::MAX_STRAIGHT_FLUSH {
            Ok(self.table.max_to_rank_class_lookup[&LookupTable::MAX_STRAIGHT_FLUSH])
        } else if hr <= LookupTable::MAX_FOUR_OF_A_KIND {
            Ok(self.table.max_to_rank_class_lookup[&LookupTable::MAX_FOUR_OF_A_KIND])
        } else if hr <= LookupTable::MAX_FULL_HOUSE {
            Ok(self.table.max_to_rank_class_lookup[&LookupTable::MAX_FULL_HOUSE])
        } else if hr <= LookupTable::MAX_FLUSH {
            Ok(self.table.max_to_rank_class_lookup[&LookupTable::MAX_FLUSH])
        } else if hr <= LookupTable::MAX_STRAIGHT {
            Ok(self.table.max_to_rank_class_lookup[&LookupTable::MAX_STRAIGHT])
        } else if hr <= LookupTable::MAX_THREE_OF_A_KIND {
            Ok(self.table.max_to_rank_class_lookup[&LookupTable::MAX_THREE_OF_A_KIND])
        } else if hr <= LookupTable::MAX_TWO_PAIR {
            Ok(self.table.max_to_rank_class_lookup[&LookupTable::MAX_TWO_PAIR])
        } else if hr <= LookupTable::MAX_PAIR {
            Ok(self.table.max_to_rank_class_lookup[&LookupTable::MAX_PAIR])
        } else if hr <= LookupTable::MAX_HIGH_CARD {
            Ok(self.table.max_to_rank_class_lookup[&LookupTable::MAX_HIGH_CARD])
        } else {
            Err("Invalid hand rank, cannot return rank class")
        }
    }

    pub fn get_five_card_rank_percentage(&self, hand_rank: u32) -> f64 {
        hand_rank as f64 / LookupTable::MAX_HIGH_CARD as f64
    }

    pub fn class_to_string(&self, class_int: u32) -> String {
        self.table.rank_class_to_string_lookup.get(&class_int).unwrap().to_string()
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_rank_class() {
        let evaluator = Evaluator::new();

        assert_eq!(evaluator.get_rank_class(0).unwrap(), evaluator.table.max_to_rank_class_lookup[&LookupTable::MAX_STRAIGHT_FLUSH]);
        assert_eq!(evaluator.get_rank_class(LookupTable::MAX_STRAIGHT_FLUSH).unwrap(), evaluator.table.max_to_rank_class_lookup[&LookupTable::MAX_STRAIGHT_FLUSH]);
        assert_eq!(evaluator.get_rank_class(LookupTable::MAX_FOUR_OF_A_KIND).unwrap(), evaluator.table.max_to_rank_class_lookup[&LookupTable::MAX_FOUR_OF_A_KIND]);
        assert_eq!(evaluator.get_rank_class(LookupTable::MAX_FULL_HOUSE).unwrap(), evaluator.table.max_to_rank_class_lookup[&LookupTable::MAX_FULL_HOUSE]);
        // Continue for other hand ranks...

        match evaluator.get_rank_class(LookupTable::MAX_HIGH_CARD + 1) {
            Ok(_) => panic!("Should have returned an error for invalid hand rank"),
            Err(e) => assert_eq!(e, "Invalid hand rank, cannot return rank class"),
        }
    }
}