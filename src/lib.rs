pub mod piece{
    #[derive(Debug, Clone, Copy)]    
    pub enum SUIT{
        CLUB, DIAMOND, HEART, SPADE,
    }
    pub struct Card{
        pub number: u16,
        pub suit: SUIT,
    }

    pub const ALL_CARDS: [SUIT; 4] = [SUIT::CLUB, SUIT::DIAMOND, SUIT::HEART, SUIT::SPADE];
}

pub mod member{
    use crate::piece::{Card, self};

    pub struct Dealer{
        cards: Vec<Card>,
    }
    
    impl Dealer{
        pub fn draw_next(&mut self) -> &Card{
            self.cards.get(0).unwrap()
        }
    
        pub fn new() -> Self{
            let mut arr: Vec<Card> = Vec::new();
    
            for s in piece::ALL_CARDS{
                for x in 2..15{
                    arr.push(Card{number: x, suit: s});
                }
            }
    
            Self { cards: arr }
        }
    }
    
    pub struct Player<'a>{
        pub cards: Vec<&'a Card>,
    }

    impl Player<'_>{
        pub fn new() -> Self{
            Self{cards: Vec::new()}
        }

        pub fn create(number: i32) -> Vec<Self>{
            let mut arr: Vec<Self> = Vec::new();
            
            for _ in 0..number{
                arr.push(Self::new());
            }

            arr
        }
    }
}