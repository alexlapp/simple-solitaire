use std::cmp::{max, PartialEq};
use std::iter::Rev;
use std::slice::Iter;
use crate::board::pile_logic::{PileBuilder, PileLogic};
use crate::cards::{Card, Suit};

#[derive(Debug)]
pub struct BoardLocation {
    pub x: u8,
    pub y: u8,
}

#[derive(Copy, Clone)]
pub enum BoardItemStyle {
    Back,
    Empty,
    Ace(Suit),
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Copy, Clone)]
pub struct PileId(pub usize);

#[derive(Clone, Copy)]
pub enum PileFlow {
    Stack,
    Down,
    Right,
}

pub struct Pile {
    pub(crate) id: PileId,
    pub loc: BoardLocation,
    cards: Vec<Card>,
    logic: PileLogic,
    pub flow: PileFlow,
    pub empty_style: BoardItemStyle,
}
impl Pile {
    pub(crate) fn can_accept_card(&self, incoming: &Card) -> bool {
        self.logic.can_accept(&self.cards, incoming)
    }

    pub(crate) fn can_accept_cards(&self, incoming: &Vec<Card>) -> bool {
        if let Some(first_card) = incoming.first() {
            return self.logic.can_accept(&self.cards, first_card);
        }

        false
    }

    pub(crate) fn is_card_available(&self, target_idx: usize) -> bool {
        self.logic.is_sequence_available(&self.cards[target_idx..])
    }

    pub(crate) fn take_from_card(&mut self, target_idx: usize) -> Vec<Card> {
        self.cards.drain(target_idx..).collect()
    }

    pub(crate) fn add_cards(&mut self, incoming: &mut Vec<Card>) {
        self.cards.append(incoming);
    }

    pub(crate) fn add_card(&mut self, incoming: Card) {
        self.cards.push(incoming);
    }

    pub fn card_iter(&self) -> Iter<Card> {
        self.cards.iter()
    }

    pub fn card_iter_ex(&self) -> impl Iterator<Item = (&Card, CardLocation)> {
        self.cards.iter().enumerate().map(|(i, c)| { return (c, CardLocation { pile_id: self.id, card_idx: i }) })
    }

    pub fn card_iter_rev(&self) -> Rev<Iter<Card>> {
        self.cards.iter().rev()
    }
}

pub struct CardLocation {
    pub pile_id: PileId,
    pub card_idx: usize,
}

pub(crate) struct ActionItemId(pub usize);

pub(crate) struct ActionItem {
    id: ActionItemId,
    loc: BoardLocation,
    style: BoardItemStyle,
}

pub struct Board {
    piles: Vec<Pile>,
    action_items: Vec<ActionItem>,
}

impl Board {
    pub fn default() -> Self {
        Self {
            // selected_pile: None,
            piles: Vec::new(),
            action_items: Vec::new(),
        }
    }

    pub(crate) fn get_next_pile_id(&self) -> PileId {
        self.piles.iter().map(|p| p.id).max().map(|id| PileId(id.0 + 1)).unwrap_or(PileId(0))
    }

    pub(crate) fn add_pile(&mut self, pile: Pile) {
        self.piles.push(pile);
    }

    pub(crate) fn create_pile(&mut self, loc: BoardLocation, builder: impl Fn(&mut PileBuilder) -> &mut PileBuilder) -> PileId {
        let id = self.get_next_pile_id();
        let pile = builder(&mut PileBuilder::new()).build_pile(id, loc);
        self.add_pile(pile);

        id
    }

    fn add_action_item(&mut self, action_item: ActionItem) {
        self.action_items.push(action_item);
    }

    pub(crate) fn get_pile_mut(&mut self, pile_id: PileId) -> &mut Pile {
        self.piles.iter_mut().find(|p| { p.id == pile_id }).expect("Pile ID is invalid")
    }

    pub fn max_board_pos(&self) -> BoardLocation {
        let mut x = 0;
        let mut y = 0;

        for pile in self.piles.iter() {
            x = max(x, pile.loc.x);
            y = max(y, pile.loc.y);
        }

        BoardLocation { x, y }
    }

    pub fn pile_iter(&self) -> Iter<Pile> {
        self.piles.iter()
    }
}

pub mod pile_logic {
    use crate::board::{BoardItemStyle, BoardLocation, Pile, PileFlow, PileId};
    use crate::cards;

    #[derive(Copy, Clone)]
    pub enum SuitOrdering {
        Any,
        Same,
        SameColor,
        AlternatingColor,
    }
    #[derive(Copy, Clone)]
    pub enum RankOrdering {
        Any,
        Descending,
        Decrementing,
        Increasing,
        Incrementing,
    }
    #[derive(Copy, Clone)]
    pub enum Availability {
        All,
        Ordered,
        Top,
    }
    #[derive(Copy, Clone)]
    pub enum AcceptLogic {
        Any,
        None,
        Ordered,
        Count(usize),
    }
    #[derive(Copy, Clone)]
    pub enum EmptyAcceptLogic {
        Any,
        None,
        Only(cards::Rank),
    }
    #[derive(Copy, Clone)]
    pub struct PileLogic {
        suit: SuitOrdering,
        rank: RankOrdering,
        availability: Availability,
        accept: AcceptLogic,
        empty_accept: EmptyAcceptLogic,
    }

    impl PileLogic {
        pub fn are_suits_ordered(&self, top: &cards::Card, bottom: &cards::Card) -> bool {
            match self.suit {
                SuitOrdering::Any => true,
                SuitOrdering::Same => top.suit == bottom.suit,
                SuitOrdering::SameColor => top.get_color() == bottom.get_color(),
                SuitOrdering::AlternatingColor => top.get_color() != bottom.get_color(),
            }
        }

        pub fn are_ranks_ordered(&self, top: &cards::Card, bottom: &cards::Card) -> bool {
            match self.rank {
                RankOrdering::Any => true,
                RankOrdering::Descending => top.get_rank_value() > bottom.get_rank_value(),
                RankOrdering::Decrementing => top.get_rank_value().saturating_sub(bottom.get_rank_value()) == 1,
                RankOrdering::Increasing => top.get_rank_value() < bottom.get_rank_value(),
                RankOrdering::Incrementing => bottom.get_rank_value().saturating_sub(top.get_rank_value()) == 1,
            }
        }

        pub fn are_cards_ordered(&self, top: &cards::Card, bottom: &cards::Card) -> bool {
            self.are_suits_ordered(&top, &bottom) && self.are_ranks_ordered(&top, &bottom)
        }

        pub fn is_sequence_ordered(&self, cards: &[cards::Card]) -> bool {
            if cards.len() == 1 { return true }

            cards.windows(2).all(|w| {
                self.are_cards_ordered(&w[0], &w[1])
            })
        }

        pub fn is_sequence_available(&self, cards: &[cards::Card]) -> bool {
            match self.availability {
                Availability::All => true,
                Availability::Top => cards.len() == 1,
                Availability::Ordered => self.is_sequence_ordered(cards),
            }
        }

        pub fn can_accept(&self, cards: &[cards::Card], incoming: &cards::Card) -> bool {
            match cards.last() {
                None => match &self.empty_accept {
                    EmptyAcceptLogic::Any => true,
                    EmptyAcceptLogic::None => false,
                    EmptyAcceptLogic::Only(rank) => incoming.rank == *rank,
                }
                Some(card) => match self.accept {
                    AcceptLogic::Any => true,
                    AcceptLogic::None => false,
                    AcceptLogic::Ordered => self.are_cards_ordered(card, incoming),
                    AcceptLogic::Count(count) => cards.len() < count,
                }
            }
        }
    }

    pub struct PileBuilder {
        suit: Option<SuitOrdering>,
        rank: Option<RankOrdering>,
        availability: Option<Availability>,
        accept: Option<AcceptLogic>,
        empty_accept: Option<EmptyAcceptLogic>,
        pile_flow: Option<PileFlow>,
        pile_style: Option<BoardItemStyle>,
    }
    impl PileBuilder {
        pub fn new() -> Self {
            Self {
                suit: None, rank: None, availability: None, accept: None, empty_accept: None, pile_flow: None, pile_style: None
            }
        }

        pub fn with_suit_ordering(&mut self, suit_ordering: SuitOrdering) -> &mut Self {
            self.suit = Some(suit_ordering);
            self
        }

        pub fn with_rank_ordering(&mut self, rank_ordering: RankOrdering) -> &mut Self {
            self.rank = Some(rank_ordering);
            self
        }

        pub fn with_availability(&mut self, availability: Availability) -> &mut Self {
            self.availability = Some(availability);
            self
        }

        pub fn with_accept(&mut self, accept: AcceptLogic) -> &mut Self {
            self.accept = Some(accept);
            self
        }

        pub fn with_empty_accept(&mut self, empty_accept: EmptyAcceptLogic) -> &mut Self {
            self.empty_accept = Some(empty_accept);
            self
        }

        pub fn with_pile_flow(&mut self, pile_flow: PileFlow) -> &mut Self {
            self.pile_flow = Some(pile_flow);
            self
        }

        pub fn with_pile_style(&mut self, pile_style: BoardItemStyle) -> &mut Self {
            self.pile_style = Some(pile_style);
            self
        }

        pub fn build_logic(&self) -> PileLogic {
            PileLogic {
                suit: self.suit.unwrap_or(SuitOrdering::Any),
                rank: self.rank.unwrap_or(RankOrdering::Any),
                availability: self.availability.unwrap_or(Availability::All),
                accept: self.accept.unwrap_or(AcceptLogic::Any),
                empty_accept: self.empty_accept.unwrap_or(EmptyAcceptLogic::Any),
            }
        }

        pub fn build_pile(&self, id: PileId, loc: BoardLocation) -> Pile {
            let logic = self.build_logic();

            Pile {
                id,
                logic,
                loc,
                cards: Vec::new(),
                flow: self.pile_flow.unwrap_or(PileFlow::Down),
                empty_style: self.pile_style.unwrap_or(BoardItemStyle::Empty),
            }
        }
    }
}