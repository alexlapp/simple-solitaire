use crate::graphics::{texture, Instance};
use crate::graphics::bitmap_font::BitmapFont;

pub(crate) struct CardConfig {
    pub(crate) texture: texture::Texture,
    card_width: f32,
    card_height: f32,
    card_offset_x: f32,
    card_offset_y: f32,
    pile_padding_x: f32,
    pile_padding_y: f32,
}

impl CardConfig {
    pub(crate) fn create_default(device: &wgpu::Device, queue: &wgpu::Queue) -> Self {
        let bytes = include_bytes!("./res/card_spritesheet.png");
        let texture = texture::Texture::from_bytes(device, queue, bytes, "Card Spritesheet").unwrap();

        Self {
            texture,
            card_width: 32.,
            card_height: 48.,
            card_offset_x: 14.,
            card_offset_y: 14.,
            pile_padding_x: 10.,
            pile_padding_y: 10.,
        }
    }

    pub fn instance(&self, x: f32, y: f32, card_x: u32, card_y: u32) -> Instance {
        let upper_left_x = card_x as f32 * self.card_width;
        let upper_left_y = card_y as f32 * self.card_height;

        let sheet_width = self.texture.texture.width() as f32;
        let sheet_height = self.texture.texture.height() as f32;

        Instance {
            src_rect: glam::vec4(upper_left_x / sheet_width, upper_left_y / sheet_height, (upper_left_x + self.card_width) / sheet_width, (upper_left_y + self.card_height) / sheet_height).into(),
            position_mat: glam::Mat4::from_translation(glam::vec3(x, y, 0.0)).into(),
            size_mat: glam::Mat4::from_scale(glam::vec3(self.card_width, self.card_height, 1.)).into(),
        }
    }
}

pub(crate) struct CardInfo<'a> {
    render_scale: f32,
    card_config: &'a CardConfig,
}

impl<'a> CardInfo<'a> {
    pub fn card_width(&self) -> f32 { self.card_config.card_width * self.render_scale }
    pub fn card_height(&self) -> f32 { self.card_config.card_height * self.render_scale }
    pub fn card_offset_x(&self) -> f32 { self.card_config.card_offset_x * self.render_scale }
    pub fn card_offset_y(&self,) -> f32 { self.card_config.card_offset_y * self.render_scale }
    pub fn pile_padding_x(&self) -> f32 { self.card_config.pile_padding_x * self.render_scale }
    pub fn pile_padding_y(&self) -> f32 { self.card_config.pile_padding_y * self.render_scale }

    pub fn get_piles_width(&self, pile_count: u32) -> f32 {
        let pile_count = pile_count as f32;

        ((pile_count - 1.) * self.pile_padding_x()) + (pile_count * self.card_width())
    }
}

pub(crate) struct RenderConfig {
    pub(crate) render_scale: f32,
    pub(crate) bitmap_font: BitmapFont,
    pub(crate) card_config: CardConfig,
}

impl RenderConfig {
    pub(crate) fn get_card_info(&self) -> CardInfo {
        CardInfo {
            render_scale: self.render_scale,
            card_config: &self.card_config,
        }
    }

    pub(crate) fn get_scaling_matrix(&self) -> glam::Mat4 {
        glam::Mat4::from_scale(glam::vec3(self.render_scale, self.render_scale, 1.)).into()
    }
}