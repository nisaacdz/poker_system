pub mod piece {
    #[derive(Debug, Clone, Copy)]
    pub enum SUIT {
        CLUB,
        DIAMOND,
        HEART,
        SPADE,
    }

    impl SUIT {
        pub fn to_number(&self) -> usize {
            match self {
                SUIT::CLUB => 1,
                SUIT::DIAMOND => 2,
                SUIT::HEART => 3,
                SUIT::SPADE => 4,
            }
        }
    }

    #[derive(Debug, Clone, Copy)]
    pub enum RANK {
        TWO,
        THREE,
        FOUR,
        FIVE,
        SIX,
        SEVEN,
        EIGHT,
        NINE,
        TEN,
        JACK,
        QUEEN,
        KING,
        ACE,
    }

    impl RANK {
        pub fn to_number(&self) -> usize {
            match self {
                RANK::TWO => 2,
                RANK::THREE => 3,
                RANK::FOUR => 4,
                RANK::FIVE => 5,
                RANK::SIX => 6,
                RANK::SEVEN => 7,
                RANK::EIGHT => 8,
                RANK::NINE => 9,
                RANK::TEN => 10,
                RANK::JACK => 11,
                RANK::QUEEN => 12,
                RANK::KING => 13,
                RANK::ACE => 14,
            }
        }
    }

    #[derive(Debug)]
    pub struct Card {
        pub rank: RANK,
        pub suit: SUIT,
    }

    pub const ALL_SUITS: [SUIT; 4] = [SUIT::CLUB, SUIT::DIAMOND, SUIT::HEART, SUIT::SPADE];
    pub const ALL_RANKS: [RANK; 13] = [
        RANK::TWO,
        RANK::THREE,
        RANK::FOUR,
        RANK::FIVE,
        RANK::SIX,
        RANK::SEVEN,
        RANK::EIGHT,
        RANK::NINE,
        RANK::TEN,
        RANK::JACK,
        RANK::QUEEN,
        RANK::KING,
        RANK::ACE,
    ];
}

pub mod member {
    use crate::piece::{self, Card};
    use rand::Rng;

    pub struct Counter {
        index: usize,
    }

    impl Counter {
        pub fn next(&mut self) -> usize {
            self.index += 1;
            self.index - 1
        }
    }

    pub static mut DEFAULT_COUNTER: Counter = Counter { index: 0 };

    pub struct Dealer {
        cards: Vec<Card>,
    }

    impl Dealer {
        pub fn shuffle_cards(&mut self) {
            let mut last_index = self.cards.len() - 1;

            while last_index > 0 {
                let index = rand::thread_rng().gen_range(0..last_index);
                self.cards.swap(index, last_index);
                last_index -= 1;
            }

            unsafe {
                DEFAULT_COUNTER = Counter { index: 0 };
            }
        }

        pub fn draw_next(&self) -> &Card {
            unsafe {
                return self.cards.get(DEFAULT_COUNTER.next()).unwrap();
            }
        }

        pub fn new() -> Self {
            let mut arr: Vec<Card> = Vec::new();

            for s in piece::ALL_SUITS {
                for r in piece::ALL_RANKS {
                    arr.push(Card { rank: r, suit: s });
                }
            }

            Self { cards: arr }
        }
    }

    #[derive(Debug)]
    pub struct Player<'a> {
        pub cards: Vec<&'a Card>,
        pub money: i32,
    }

    impl Player<'_> {
        pub fn new(money: i32) -> Self {
            Self {
                cards: Vec::new(),
                money: money,
            }
        }

        pub fn create(number: i32, cash: i32) -> Vec<Self> {
            let mut arr: Vec<Self> = Vec::new();

            for _ in 0..number {
                arr.push(Self::new(cash));
            }

            arr
        }
    }

    pub struct Center<'a> {
        pub cards: Vec<&'a Card>,
        pub money: i32,
    }

    impl Center<'_> {
        pub fn new() -> Self {
            Self {
                cards: Vec::new(),
                money: 0,
            }
        }
    }
}

mod result {
    use crate::piece::Card;
    pub enum Hand<'a> {
        ROYALFLUSH([&'a Card; 5]),
        STRAIGHTFLUSH([&'a Card; 5]),
        FOUROFAKID([&'a Card; 5]),
        FULLHOUSE([&'a Card; 5]),
        FLUSH([&'a Card; 5]),
        STRAIGHT([&'a Card; 5]),
        THREEOFAKIND([&'a Card; 5]),
        TWOPAIR([&'a Card; 5]),
        PAIR([&'a Card; 5]),
        HIGHCARD([&'a Card; 5]),
    }
}
