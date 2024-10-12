use glam::Vec2;
use rand::Rng;
use crate::{GameEvent, SolitaireCursor};
use crate::graphics::config::CardSizes;
use crate::graphics::context::DrawContext;

use simple_solitaire_lib::prelude::*;

pub(crate) struct SolitaireLogic {
    mouse_down: bool,
    mouse_pos: Vec2,
    mouse_just_pressed: bool,
    mouse_just_released: bool,
    init: bool,
    game: games::Game,
    board_offset: Vec2,
}

impl SolitaireLogic {
    pub(crate) fn new() -> Self {
        let entry = games::get_game_entries().first().expect("There should be at least one entry");
        let mut game = (entry.creator)();
        game.setup();

        Self {
            mouse_down: false,
            mouse_pos: Vec2::ZERO,
            mouse_just_pressed: false,
            mouse_just_released: false,
            init: false,
            game,
            board_offset: Vec2::ZERO,
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

    pub(crate) fn update(&mut self, event: GameEvent, card_info: &CardSizes, draw_context: &DrawContext) -> SolitaireCursor {
        if !self.init {
            let max_game_pos = self.game.board_ref().max_board_pos();
            let total_width = card_info.calc_piles_width(max_game_pos.x as u32 + 1);

            let (_, text_height) = draw_context.get_text_size("Free Cell");

            self.board_offset = Vec2::new(-(total_width / 2.), text_height + 16.);
            self.init = true
        }

        self.process_event(event);

        SolitaireCursor::Pointer
    }

    pub(crate) fn render(&self, draw: &mut DrawContext, card_info: &CardSizes) {
        let text = "Free Cell";
        let (text_width, _) = draw.get_text_size(text);
        draw.text("Free Cell", 0. - text_width / 2., 20.);

        for pile in self.game.board_ref().pile_iter() {
            draw.draw_pile(pile, card_info, &self.board_offset);
            for (card, loc) in pile.card_iter_ex() {
                draw.draw_card(card, pile, loc, card_info, &self.board_offset);
            }
        }
    }
}

impl DrawContext<'_> {
    fn draw_pile(&mut self, pile: &cards::Pile, card_info: &CardSizes, board_offset: &Vec2) {
        let pile_x = (pile.loc.x as f32 * card_info.card_width()) + (pile.loc.x as f32 * card_info.pile_padding_x()) + board_offset.x;
        let pile_y = (pile.loc.y as f32 * card_info.card_height()) + (pile.loc.y as f32 * card_info.pile_padding_y()) + board_offset.y;

        self.board_item(pile_x, pile_y, pile.empty_style)
    }

    fn draw_card(&mut self, card: &cards::Card, pile: &cards::Pile, card_loc: cards::CardLocation, card_info: &CardSizes, board_offset: &Vec2) {
        let pile_x = (pile.loc.x as f32 * card_info.card_width()) + (pile.loc.x as f32 * card_info.pile_padding_x()) + board_offset.x;
        let pile_y = (pile.loc.y as f32 * card_info.card_height()) + (pile.loc.y as f32 * card_info.pile_padding_y()) + board_offset.y;

        let card_x = pile_x + (card_loc.card_idx as f32 * match pile.flow {
            cards::PileFlow::Stack => 0.,
            cards::PileFlow::Down => 0.,
            cards::PileFlow::Right => card_info.card_offset_x(),
        });

        let card_y = pile_y + (card_loc.card_idx as f32 * match pile.flow {
            cards::PileFlow::Stack => 0.,
            cards::PileFlow::Down => card_info.card_offset_y(),
            cards::PileFlow::Right => 0.,
        });

        self.card(card_x, card_y, card);
    }
}