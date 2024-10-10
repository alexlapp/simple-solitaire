use glam::Vec2;
use rand::Rng;
use crate::{GameEvent, SolitaireCursor};
use crate::graphics::config::CardInfo;
use crate::graphics::context::DrawContext;

use simple_solitaire_lib::prelude::*;
use simple_solitaire_lib::prelude::cards::PileFlow;

pub(crate) struct SolitaireLogic {
    cards: Vec<(f32, f32, u32, u32)>,
    mouse_down: bool,
    mouse_pos: Vec2,
    mouse_just_pressed: bool,
    mouse_just_released: bool,
    game: games::Game,
}

impl SolitaireLogic {
    pub(crate) fn new() -> Self {
        let entry = games::get_game_entries().first().expect("There should be at least one entry");
        let mut game = (entry.creator)();
        game.setup();

        Self {
            cards: Vec::new(),
            mouse_down: false,
            mouse_pos: Vec2::ZERO,
            mouse_just_pressed: false,
            mouse_just_released: false,
            game,
        }
    }

    fn process_event(&mut self, event: GameEvent) {
        self.mouse_just_released = false;
        self.mouse_just_pressed = false;

        match event {
            GameEvent::MouseMoved(pos) => {
                self.mouse_pos = pos;
            }
            GameEvent::MousePressed(pos) => {
                self.mouse_pos = pos;
                self.mouse_just_pressed = !self.mouse_down;
                self.mouse_down = true;
            }
            GameEvent::MouseReleased(pos) => {
                self.mouse_pos = pos;
                self.mouse_just_released = self.mouse_down;
                self.mouse_down = false;
            }
        }
    }

    pub(crate) fn update(&mut self, event: GameEvent, card_info: &CardInfo) -> SolitaireCursor {
        self.process_event(event);

        let mut is_over_card = false;
        for card in self.cards.iter() {
            if self.mouse_pos.x > card.0 && self.mouse_pos.y > card.1 {
                if self.mouse_pos.x < card.0 + card_info.card_width() && self.mouse_pos.y < card.1 + card_info.card_height() {
                    is_over_card = true;
                }
            }
        }
       if is_over_card { return if self.mouse_down { SolitaireCursor::Grabbing } else { SolitaireCursor::Grab } }

        if self.mouse_just_pressed {
            let s = rand::thread_rng().gen_range(0..4);
            let r = rand::thread_rng().gen_range(0..13);
            self.cards.push((self.mouse_pos.x - (card_info.card_width() / 2.), self.mouse_pos.y - (card_info.card_height() / 2. ), s, r));
        }

        SolitaireCursor::Pointer
    }

    pub(crate) fn render(&self, draw: &mut DrawContext, card_info: &CardInfo) {
        let text = "Hello World!";
        let (text_width, _) = draw.get_text_size(text);
        draw.text("Hello World!", 0. - text_width / 2., 20.);

        for pile in self.game.board_ref().pile_iter() {
            self.draw_pile(pile);
            for (card, loc) in pile.card_iter_ex() {
                self.draw_card(card, pile, loc, draw, card_info);
            }
        }

        for card in self.cards.iter() {
            draw.card(card.0, card.1, card.2, card.3);
        }
    }
}

impl SolitaireLogic {
    fn draw_pile(&self, pile: &cards::Pile) {

    }

    fn draw_card(&self, card: &cards::Card, pile: &cards::Pile, card_loc: cards::CardLocation, draw: &mut DrawContext, card_info: &CardInfo) {
        let pile_x = (pile.loc.x as f32 * card_info.card_width()) + (pile.loc.x as f32 * card_info.card_offset_x());
        let pile_y = (pile.loc.y as f32 * card_info.card_height()) + (pile.loc.y as f32 * card_info.card_offset_y());

        let card_x = pile_x + (card_loc.card_idx as f32 * match pile.flow {
            PileFlow::Stack => 0.,
            PileFlow::Down => 0.,
            PileFlow::Right => card_info.card_offset_x(),
        });

        let card_y = pile_y + (card_loc.card_idx as f32 * match pile.flow {
            PileFlow::Stack => 0.,
            PileFlow::Down => card_info.card_offset_y(),
            PileFlow::Right => 0.,
        });

        draw.card(card_x, card_y, card.suit as u32, card.get_rank_value() as u32);
    }
}