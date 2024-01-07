use itertools::Itertools;
use std::collections::HashMap;

use crate::card::Card;

pub struct LookupTable {
    pub flush_lookup: HashMap<u32, u32>,
    pub unsuited_lookup: HashMap<u32, u32>,
    pub max_to_rank_class_lookup: HashMap<u32, u32>,
    pub rank_class_to_string_lookup: HashMap<u32, &'static str>,
}

impl LookupTable {
    pub const MAX_STRAIGHT_FLUSH: u32 = 10;
    pub const MAX_FOUR_OF_A_KIND: u32 = 166;
    pub const MAX_FULL_HOUSE: u32 = 322;
    pub const MAX_FLUSH: u32 = 1599;
    pub const MAX_STRAIGHT: u32 = 1609;
    pub const MAX_THREE_OF_A_KIND: u32 = 2467;
    pub const MAX_TWO_PAIR: u32 = 3325;
    pub const MAX_PAIR: u32 = 6185;
    pub const MAX_HIGH_CARD: u32 = 7462;

    pub fn new() -> Self {
        let mut lookup_table = LookupTable {
            flush_lookup: HashMap::new(),
            unsuited_lookup: HashMap::new(),
            max_to_rank_class_lookup: HashMap::new(),
            rank_class_to_string_lookup: HashMap::new(),
        };

        lookup_table.flushes();
        lookup_table.multiples();
        lookup_table.max_to_rank_class();
        lookup_table.rank_class_to_string();

        lookup_table
    }

    pub fn rank_class_to_string(&mut self) {
        self.rank_class_to_string_lookup.insert(1, "Straight Flush");
        self.rank_class_to_string_lookup.insert(2, "Four of a Kind");
        self.rank_class_to_string_lookup.insert(3, "Full House");
        self.rank_class_to_string_lookup.insert(4, "Flush");
        self.rank_class_to_string_lookup.insert(5, "Straight");
        self.rank_class_to_string_lookup
            .insert(6, "Three of a Kind");
        self.rank_class_to_string_lookup.insert(7, "Two Pair");
        self.rank_class_to_string_lookup.insert(8, "Pair");
        self.rank_class_to_string_lookup.insert(9, "High Card");
    }

    fn get_lexographically_next_bit_sequence(&self, bits: i32) -> impl Iterator<Item = i32> {
        let mut next = (bits | (bits - 1)) + 1;
        next |= (((next as i32) & -(next as i32)) / ((bits as i32) & -(bits as i32)) >> (1 as i32))
            - (1 as i32);

        std::iter::successors(Some(next), |&next| {
            let t = (next | (next - 1)) + 1;
            Some(t | ((t & -t) / (next & -next) >> 1) - 1)
        })
    }

    pub fn flushes(&mut self) {
        let straight_flushes = vec![
            7936, // 0b1111100000000, // royal flush
            3968, // 0b111110000000,
            1984, // 0b11111000000,
            992,  // 0b1111100000,
            496,  // 0b111110000,
            248,  // 0b11111000,
            124,  // 0b1111100,
            62,   // 0b111110,
            31,   // 0b11111,
            4111, // 0b1000000001111, // 5 high
        ];

        let mut flushes = Vec::new();
        let mut gen = self.get_lexographically_next_bit_sequence(0b11111);

        for _ in 0..(1277 + straight_flushes.len() - 1) {
            let f = gen.next().unwrap();

            if !straight_flushes.contains(&f) {
                flushes.push(f);
            }
        }

        // println!("flushes: {:?}", &flushes[0..4]);
        // println!("flushes: {:?}", &flushes.len());

        flushes.reverse();

        let mut rank = 1;
        for sf in &straight_flushes {
            // println!("sf: {}", sf);
            let prime_product = Card::prime_product_from_rankbits(*sf as u32); // Dereference the `sf` variable
                                                                               // println!("prime_product: {}", prime_product);
            self.flush_lookup.insert(prime_product, rank);
            rank += 1;
            // println!("rank: {}", rank);
        }

        // println!("flush_lookup: {:?}", &self.flush_lookup);

        rank = LookupTable::MAX_FULL_HOUSE + 1;
        for f in &flushes {
            let prime_product = Card::prime_product_from_rankbits(*f as u32);
            self.flush_lookup.insert(prime_product, rank);
            rank += 1;
        }

        // println!("flush_lookup: {:?}", &self.flush_lookup[&629629]);

        // for (i, f) in flushes.iter().enumerate() {
        //     self.flush_lookup.insert(*f, (i + 1) as u32);
        // }

        self.straight_and_highcards(straight_flushes, flushes)
    }

    pub fn straight_and_highcards(&mut self, straights: Vec<i32>, highcards: Vec<i32>) {
        let mut rank = LookupTable::MAX_FLUSH + 1;

        for s in straights {
            let prime_product = Card::prime_product_from_rankbits(s as u32);
            self.unsuited_lookup.insert(prime_product, rank);
            rank += 1;
        }

        rank = LookupTable::MAX_PAIR + 1;
        for h in highcards {
            let prime_product = Card::prime_product_from_rankbits(h as u32);
            self.unsuited_lookup.insert(prime_product, rank);
            rank += 1;
        }
    }

    pub fn multiples(&mut self) {
        let backwards_ranks: Vec<_> = (0..Card::INT_RANKS.len()).rev().collect();

        // println!("{:?}", backwards_ranks);

        // 1) Four of a Kind
        let mut rank = LookupTable::MAX_STRAIGHT_FLUSH + 1;

        // for each choice of a set of four rank
        for i in &backwards_ranks {
            // and for each possible kicker rank
            let mut kickers: Vec<_> = backwards_ranks.clone();
            kickers.retain(|&x| x != *i);
            for k in &kickers {
                let product = Card::PRIMES[*i].pow(4) * Card::PRIMES[*k];
                self.unsuited_lookup.insert(product, rank);
                rank += 1;
            }
        }

        // 2) Full House
        let mut rank = LookupTable::MAX_FOUR_OF_A_KIND + 1;

        // for each three of a kind
        for i in &backwards_ranks {
            // and for each choice of pair rank
            let mut pairranks: Vec<_> = backwards_ranks.clone();
            pairranks.retain(|&x| x != *i);
            for pr in &pairranks {
                let product = Card::PRIMES[*i].pow(3) * Card::PRIMES[*pr].pow(2);
                self.unsuited_lookup.insert(product, rank);
                rank += 1;
            }
        }

        // 3) Three of a Kind
        rank = LookupTable::MAX_STRAIGHT + 1;

        // pick three of one rank
        for r in &backwards_ranks {
            let mut kickers: Vec<_> = backwards_ranks.clone();
            kickers.retain(|&x| x != *r);
            let gen: Vec<_> = kickers.iter().combinations(2).collect();

            for kickers in gen {
                let (c1, c2) = (kickers[0], kickers[1]);
                let product = Card::PRIMES[*r].pow(3) * Card::PRIMES[*c1] * Card::PRIMES[*c2];
                self.unsuited_lookup.insert(product, rank);
                rank += 1;
            }
        }

        // 4) Two Pair
        let mut rank = LookupTable::MAX_THREE_OF_A_KIND + 1;

        let tpgen: Vec<_> = backwards_ranks.iter().combinations(2).collect();
        for tp in tpgen {
            let (pair1, pair2) = (*tp[0], *tp[1]);
            let mut kickers: Vec<_> = backwards_ranks.clone();
            kickers.retain(|&x| x != pair1 && x != pair2);
            for kicker in &kickers {
                let product =
                    Card::PRIMES[pair1].pow(2) * Card::PRIMES[pair2].pow(2) * Card::PRIMES[*kicker];
                self.unsuited_lookup.insert(product, rank);
                rank += 1;
            }
        }

        // 5) Pair
        let mut rank = LookupTable::MAX_TWO_PAIR + 1;

        // choose a pair
        for pairrank in &backwards_ranks {
            let mut kickers: Vec<_> = backwards_ranks.clone();
            kickers.retain(|&x| x != *pairrank);
            let kgen: Vec<_> = kickers.iter().combinations(3).collect();

            for kickers in kgen {
                let (k1, k2, k3) = (*kickers[0], *kickers[1], *kickers[2]);
                let product = Card::PRIMES[*pairrank].pow(2)
                    * Card::PRIMES[k1]
                    * Card::PRIMES[k2]
                    * Card::PRIMES[k3];
                self.unsuited_lookup.insert(product, rank);
                rank += 1;
            }
        }
    }

    pub fn max_to_rank_class(&mut self) {
        //let mut max_to_rank_class = HashMap::new();
        self.max_to_rank_class_lookup
            .insert(Self::MAX_STRAIGHT_FLUSH, 1);
        self.max_to_rank_class_lookup
            .insert(Self::MAX_FOUR_OF_A_KIND, 2);
        self.max_to_rank_class_lookup
            .insert(Self::MAX_FULL_HOUSE, 3);
        self.max_to_rank_class_lookup.insert(Self::MAX_FLUSH, 4);
        self.max_to_rank_class_lookup.insert(Self::MAX_STRAIGHT, 5);
        self.max_to_rank_class_lookup
            .insert(Self::MAX_THREE_OF_A_KIND, 6);
        self.max_to_rank_class_lookup.insert(Self::MAX_TWO_PAIR, 7);
        self.max_to_rank_class_lookup.insert(Self::MAX_PAIR, 8);
        self.max_to_rank_class_lookup.insert(Self::MAX_HIGH_CARD, 9);
    }
}

#[cfg(test)]
mod tests {
    // use crate::Card;
    use super::*;

    #[test]
    fn test_new() {
        let lookup_table = LookupTable::new();
        assert_eq!(lookup_table.flush_lookup.len(), 1287);
        assert_eq!(lookup_table.unsuited_lookup.len(), 6175);
    }
}
