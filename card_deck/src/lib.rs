use rand::Rng;

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Suit {
    Heart,
    Diamond,
    Spade,
    Club,
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Rank{
    Ace,
    King,
    Queen, 
    Jack,
    Number(u8)
}

impl Suit {
    pub fn random() -> Suit {
        let mut rnd = rand::thread_rng();
        match rnd.gen_range(1..4) {
            1 => Suit::Heart,
            2 => Suit::Diamond,
            3 => Suit::Spade,
            _ => Suit::Club,
        }
    }
    pub fn translate(value: u8) -> Suit {
        match value {
            1 => Suit::Heart,
            2 => Suit::Diamond,
            3 => Suit::Spade,
            4 => Suit::Club,
            _ => panic!("Invalid suit value!"),
        }
    }
}

impl Rank {
    pub fn random() -> Rank {
        let mut random = rand::thread_rng();
        match random.gen_range(1..=13) {
            1 => Rank::Ace,
            11 => Rank::Jack,
            12 => Rank::Queen,
            13 => Rank::King,
            value => Rank::Number(value),
        }
    }

    pub fn translate(value: u8) -> Rank {
        match value {
            1 => Rank::Ace,
            11 => Rank::Jack,
            12 => Rank::Queen,
            13 => Rank::King,
            2..=10 => Rank::Number(value),
            _ => panic!("Invalid rank value!"),
        }
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Card {
    pub suit: Suit,
    pub rank: Rank,
}

pub fn winner_card(card: Card) -> bool {
    card == Card {
        suit: Suit::Spade,
        rank: Rank::Ace,
    }
}