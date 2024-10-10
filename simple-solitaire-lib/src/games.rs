use std::cmp::PartialEq;
use crate::board::{PileId, Board, CardLocation, PileFlow, BoardLocation, BoardItemStyle};
use crate::board::pile_logic::{AcceptLogic, Availability, EmptyAcceptLogic, RankOrdering, SuitOrdering};
use crate::cards::{Card, Deck, Rank, Suit};

#[derive(PartialEq)]
enum GameStatus {
    Ongoing,
    Lost,
    Won,
}
pub(crate) trait GameLogic {
    // fn default() -> Self;
    // fn create_game() -> Game {
    //     Game { logic: Self::default(), board: Board::default(), selection: None }
    // }
    fn setup(&mut self, board: &mut Board);
    fn get_status(&self, board: &Board) -> GameStatus;
}

pub enum GameEvent {
    SelectEvent(CardLocation),
    DropEvent(Option<PileId>),
}

struct SelectedPile {
    cards: Vec<Card>,
    flow: PileFlow,
    source: PileId,
}

pub struct Game {
    logic: Box<dyn GameLogic>,
    board: Board,
    selection: Option<SelectedPile>,
}

impl Game {
    pub(crate) fn create_with_logic(logic: Box<dyn GameLogic>) -> Self {
        Game {
            logic,
            selection: None,
            board: Board::default(),
        }
    }

    fn select_cards(&mut self, source: CardLocation) {
        let pile = self.board.get_pile_mut(source.pile_id);
        let cards = pile.take_from_card(source.card_idx);
        self.selection = Some(SelectedPile {
            cards,
            flow: pile.flow,
            source: pile.id,
        })
    }

    fn add_selection_to_pile(&mut self, target: PileId) {
        if let Some(selection) = &mut self.selection {
            let pile = self.board.get_pile_mut(target);
            pile.add_cards(&mut selection.cards);

            self.selection = None;
        }
    }

    fn return_selection(&mut self) {
        if let Some(id) = self.selection.as_ref().map(|s| s.source) {
            self.add_selection_to_pile(id);
        }
    }

    pub fn setup(&mut self) {
        self.logic.setup(&mut self.board);
    }

    pub fn board_ref(&self) -> &Board {
        &self.board
    }

    pub fn handle_event(&mut self, event: GameEvent) {
        match event {
            GameEvent::SelectEvent(card_location) => {
                let pile = self.board.get_pile_mut(card_location.pile_id);
                if pile.is_card_available(card_location.card_idx) {
                    self.select_cards(card_location);
                }
            }
            GameEvent::DropEvent(target) => {
                match (&mut self.selection, target) {
                    (Some(selection), Some(pile_id)) => {
                        let pile = self.board.get_pile_mut(pile_id);
                        match pile.can_accept_cards(&selection.cards) {
                            true => {
                                pile.add_cards(&mut selection.cards);
                                self.selection = None;
                            },
                            false => self.return_selection(),
                        }
                    },
                    (Some(_), None) => self.return_selection(),
                    (_, _) => {}
                };
            }
        }
    }
}

pub struct FreeCell {
    cell_ids: Vec<PileId>,
    foundation_ids: Vec<PileId>,
    tableau_ids: Vec<PileId>,
}
impl Default for FreeCell {
    fn default() -> Self {
        Self {
            cell_ids: Vec::new(),
            foundation_ids: Vec::new(),
            tableau_ids: Vec::new(),
        }
    }
}
impl GameLogic for FreeCell {
    fn setup(&mut self, board: &mut Board) {
        let mut deck = Deck::default();
        deck.add_deck();

        // Cells
        for i in 0..4 {
            let loc = BoardLocation { x: i, y: 0 };
            let id = board.create_pile(loc, |builder| {
                builder
                    .with_pile_flow(PileFlow::Stack)
                    .with_accept(AcceptLogic::Count(1))
                    .with_pile_style(BoardItemStyle::Empty)
            });

            self.cell_ids.push(id);
        }

        // Foundations
        for i in 0..4 {
            let suit = Suit::get_ordered()[i as usize];
            let loc = BoardLocation { x: i + 4, y: 0 };
            let id = board.create_pile(loc, |builder| {
                builder
                    .with_empty_accept(EmptyAcceptLogic::Only(Rank::Ace))
                    .with_suit_ordering(SuitOrdering::Same)
                    .with_rank_ordering(RankOrdering::Incrementing)
                    .with_accept(AcceptLogic::Ordered)
                    .with_pile_style(BoardItemStyle::Ace(suit))
                    .with_pile_flow(PileFlow::Stack)
                    .with_availability(Availability::Top)
            });

            self.foundation_ids.push(id);
        }

        // Tableau
        for i in 0..8 {
            let loc = BoardLocation { x: i, y: 1 };
            let id = board.create_pile(loc, |builder| {
               builder
                   .with_availability(Availability::Top)
                   .with_suit_ordering(SuitOrdering::AlternatingColor)
                   .with_rank_ordering(RankOrdering::Decrementing)
                   .with_empty_accept(EmptyAcceptLogic::Any)
                   .with_pile_style(BoardItemStyle::Empty)
                   .with_pile_flow(PileFlow::Down)
                   .with_accept(AcceptLogic::Ordered)
            });

            self.tableau_ids.push(id);
        }

        let mut deck = Deck::single_deck();
        deck.shuffle();

        let mut i = 0;
        while let Some(card) = deck.deal_card() {
            let pile_id = self.tableau_ids[i % 8];
            board.get_pile_mut(pile_id).add_card(card);

            i += 1;
        }
    }

    fn get_status(&self, board: &Board) -> GameStatus {
        todo!()
    }
}