pub struct Card;

impl Card {
    pub const INT_RANKS: [u32; 13] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12];
    pub const PRIMES: [u32; 13] = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41];
    pub const CHAR_RANK_TO_INT_RANK: [(char, u32); 13] = [
        ('2', 0),
        ('3', 1),
        ('4', 2),
        ('5', 3),
        ('6', 4),
        ('7', 5),
        ('8', 6),
        ('9', 7),
        ('T', 8),
        ('J', 9),
        ('Q', 10),
        ('K', 11),
        ('A', 12),
    ];
    pub const CHAR_SUIT_TO_INT_SUIT: [(char, u32); 4] = [
        ('s', 1), // spades
        ('h', 2), // hearts
        ('d', 4), // diamonds
        ('c', 8), // clubs
    ];

    pub fn new(string: &str) -> u32 {
        let rank_char = string.chars().next().unwrap();
        let suit_char = string.chars().nth(1).unwrap();
        let rank_int = Card::CHAR_RANK_TO_INT_RANK.iter().find(|&&(r, _)| r == rank_char).unwrap().1;
        let suit_int = Card::CHAR_SUIT_TO_INT_SUIT.iter().find(|&&(s, _)| s == suit_char).unwrap().1;
        let rank_prime = Card::PRIMES[rank_int as usize];

        let bitrank = 1 << rank_int << 16;
        let suit = suit_int << 12;
        let rank = rank_int << 8;

        bitrank | suit | rank | rank_prime
    }

    pub fn prime_product_from_rankbits(rankbits: u32) -> u32 {
        let mut product = 1;
        for i in Card::INT_RANKS.iter() {
            if rankbits & (1 << i) != 0 {
                product *= Card::PRIMES[*i as usize];
            }
        }
        product
    }

    pub fn prime_product_from_hand(card_ints: &[u32]) -> u32 {
        let mut product = 1;
        for &c in card_ints {
            product *= c & 0xFF;
        }
        product
    }
}

#[cfg(test)]
mod tests {
    use super::*;

     #[test]
    fn test_new() {
        assert_eq!(Card::new("2s"), 69634);
        assert_eq!(Card::new("3h"), 139523);
        assert_eq!(Card::new("4d"), 279045);
        assert_eq!(Card::new("Ac"), 268471337);
    }

    #[test]
    fn test_prime_product_from_rankbits() {
        assert_eq!(Card::prime_product_from_rankbits(0b1000000000001), 82);
        assert_eq!(Card::prime_product_from_rankbits(0b1000000000010), 123);
        assert_eq!(Card::prime_product_from_rankbits(0b1000000000100), 205);
        assert_eq!(Card::prime_product_from_rankbits(0b1000001000000), 697);
        assert_eq!(Card::prime_product_from_rankbits(7936), 31367009);
        assert_eq!(Card::prime_product_from_rankbits(124), 85085);
        assert_eq!(Card::prime_product_from_rankbits(62), 15015);
        assert_eq!(Card::prime_product_from_rankbits(31), 2310);
        assert_eq!(Card::prime_product_from_rankbits(4111), 8610);
    }
}