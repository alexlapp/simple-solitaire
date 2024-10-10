use rand::prelude::{SliceRandom, thread_rng};

#[derive(PartialEq, Eq)]
pub(crate) enum CardColor {
    Red,
    Black,
}

#[derive(PartialEq, Eq, Copy, Clone)]
pub enum Suit {
    Spades,
    Diamonds,
    Clubs,
    Hearts,
}

impl Suit {
    pub fn get_ordered() -> [Suit;4] {
        [Suit::Spades, Suit::Diamonds, Suit::Clubs, Suit::Hearts]
    }
}

#[derive(PartialEq, Eq, Copy, Clone)]
pub enum Rank {
    Ace,
    Value(u8),
    Jack,
    Queen,
    King,
}

pub struct Card {
    pub suit: Suit,
    pub rank: Rank,
}

impl Card {
    pub fn get_color(&self) -> CardColor {
        match self.suit {
            Suit::Hearts |
            Suit::Diamonds => CardColor::Red,
            Suit::Clubs |
            Suit::Spades => CardColor::Black,
        }
    }

    pub fn get_rank_value(&self) -> u8 {
        match self.rank {
            Rank::Ace => 1,
            Rank::Value(v) => v,
            Rank::Jack => 11,
            Rank::Queen => 12,
            Rank::King => 13,
        }
    }
}

pub(crate) struct Deck {
    cards: Vec<Card>
}
impl Deck {
    pub(crate) fn default() -> Self { Self { cards: Vec::new() } }

    pub(crate) fn single_deck() -> Self {
        let mut deck = Self::default();
        deck.add_deck();

        deck
    }

    pub(crate) fn add_deck(&mut self) {
        for suit in [Suit::Spades, Suit::Diamonds, Suit::Clubs, Suit::Hearts] {
            for rank_value in 1..=13 {
                match rank_value {
                    1 => self.cards.push(Card { suit, rank: Rank::Ace }),
                    2..=10 => self.cards.push(Card { suit, rank: Rank::Value(rank_value) }),
                    11 => self.cards.push(Card { suit, rank: Rank::Jack }),
                    12 => self.cards.push(Card { suit, rank: Rank::Queen }),
                    13 => self.cards.push(Card { suit, rank: Rank::King }),
                    _ => panic!("Somehow out of bounds")
                }
            }
        }
    }

    pub(crate) fn deal_card(&mut self) -> Option<Card> {
        self.cards.pop()
    }

    pub(crate) fn shuffle(&mut self) {
        self.cards.shuffle(&mut thread_rng());
    }
}