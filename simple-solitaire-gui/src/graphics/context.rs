use simple_solitaire_lib::prelude::cards;
use simple_solitaire_lib::prelude::cards::BoardItemStyle;
use crate::graphics::bitmap_font::BitmapFont;
use crate::graphics::config::{CardConfig, RenderConfig};
use crate::graphics::{Instance, MAX_CARD_INSTANCES, MAX_TEXT_INSTANCES};

pub(crate) struct DrawContext<'a> {
    pub(crate) card_config: &'a CardConfig,
    pub(crate) font: &'a BitmapFont,
    pub(crate) card_instances: Vec<Instance>,
    pub(crate) char_instances: Vec<Instance>,
    render_scale: f32
}

impl<'a> DrawContext<'a> {
    pub(crate) fn from_render_config(render_config: &'a RenderConfig) -> Self {
        Self::new(&render_config.card_config, &render_config.bitmap_font, render_config.render_scale)
    }

    pub(crate) fn new(card_config: &'a CardConfig, font: &'a BitmapFont, scale: f32) -> Self {
        Self {
            card_instances: Vec::new(),
            char_instances: Vec::new(),
            font,
            card_config,
            render_scale: scale
        }
    }

    pub fn card(&mut self, x: f32, y: f32, card: &cards::Card) {
        self.card_raw(x, y, card.suit as u8, card.get_rank_value() - 1)
    }

    fn card_raw(&mut self, x: f32, y: f32, card_s: u8, card_r: u8) {
        let instance = self.card_config.instance(x, y, card_s, card_r);
        self.card_instances.push(instance);

        assert!(self.card_instances.len() <= MAX_CARD_INSTANCES, "Too many cards instanced");
    }

    pub fn board_item(&mut self, x: f32, y: f32, board_item: BoardItemStyle) {
        let (sheet_x, sheet_y) = match board_item {
            BoardItemStyle::Back => {(2, 13)}
            BoardItemStyle::Empty => {(0, 15)}
            BoardItemStyle::Ace(suit) => {(suit as u8, 14)}
        };

        let instance = self.card_config.instance(x, y, sheet_x, sheet_y);
        self.card_instances.push(instance);

        assert!(self.card_instances.len() <= MAX_CARD_INSTANCES, "Too many cards instanced");
    }

    pub fn text(&mut self, text: &str, x: f32, y: f32) {
        let mut instances = self.font.instance_string(text, x, y, self.render_scale);
        self.char_instances.append(&mut instances);

        assert!(self.char_instances.len() <= MAX_TEXT_INSTANCES, "Too many characters instanced");
    }

    pub fn get_text_size(&self, text: &str) -> (f32, f32) {
        let size = self.font.get_text_size(text, self.render_scale);
        (size.x, size.y)
    }
}