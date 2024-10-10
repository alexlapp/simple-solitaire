use std::collections::HashMap;
use crate::graphics::{texture, Instance};

struct BitmapInfoSymbol {
    height: f32,
    width: f32,
    x: f32,
    y: f32,
    x_advance: f32,
    x_offset: f32,
    y_offset: f32,
}

pub struct BitmapFont {
    pub texture: texture::Texture,
    symbol_info_map: HashMap<u32, BitmapInfoSymbol>,
}

impl BitmapFont {
    pub fn load_font(device: &wgpu::Device, queue: &wgpu::Queue) -> Self {
        let png_bytes = include_bytes!("./res/arcade_pix.png");
        let texture = texture::Texture::from_bytes(device, queue, png_bytes, "Font Spritesheet").expect("Unable to load Font Spritesheet");

        let fnt_info = include_str!("./res/arcade_pix.fnt");

        let mut lookup = HashMap::new();

        for line in fnt_info.lines() {
            let parts: HashMap<_, _> = line
                .split_ascii_whitespace()
                .filter_map(|part| {
                    if !part.contains('=')  { return None; }
                    Some(part.split_once('=').expect("Something has gone horribly wrong"))
                })
                .collect();

            lookup.insert(
                parts["id"].parse().expect("Unable to parse id"),
                BitmapInfoSymbol {
                    x: parts["x"].parse().expect("Unable to parse x"),
                    y: parts["y"].parse().expect("Unable to parse y"),
                    width: parts["width"].parse().expect("Unable to parse width"),
                    height: parts["height"].parse().expect("Unable to parse height"),
                    x_advance: parts["xadvance"].parse().expect("Unable to parse xadvance"),
                    x_offset: parts["xoffset"].parse().expect("Unable to parse xoffset"),
                    y_offset: parts["yoffset"].parse().expect("Unable to parse yoffset"),
                }
            );
        }

        Self {
            texture,
            symbol_info_map: lookup,
        }
    }

    fn get_char_source_rect(&self, sym_info: &BitmapInfoSymbol) -> glam::Vec4 {
        let upper_left_x = sym_info.x;
        let upper_left_y = sym_info.y;
        let lower_right_x = sym_info.x + sym_info.width;
        let lower_right_y = sym_info.y + sym_info.height;

        glam::Vec4::new(
            upper_left_x / self.texture.texture.width() as f32,
            upper_left_y / self.texture.texture.height() as f32,
            lower_right_x / self.texture.texture.width() as f32,
            lower_right_y / self.texture.texture.height() as f32,
        )
    }

    fn instance(&self, sym_info: &BitmapInfoSymbol, x: f32, y: f32, scale: f32) -> Instance {
        Instance {
            src_rect: self.get_char_source_rect(sym_info).into(),
            position_mat: glam::Mat4::from_translation(glam::vec3(x + (sym_info.x_offset * scale), y + (sym_info.y_offset * scale), 0.).into()).into(),
            size_mat: glam::Mat4::from_scale(glam::vec3(sym_info.width, sym_info.height, 1.)).into(),
        }
    }

    pub fn instance_string(&self, text: &str, x: f32, y: f32, scale: f32) -> Vec<Instance> {
        let mut result = Vec::new();
        let mut x = x;

        for c in text.chars() {
            let info = self.symbol_info_map.get(&(c as u32)).expect(&format!("Invalid Character: {}", c));

            result.push(self.instance(info, x, y, scale));
            x += info.x_advance * scale;
        }

        result
    }

    pub fn get_text_size(&self, text: &str, scale: f32) -> glam::Vec2 {
        let mut width = 0.;
        let mut lo_y = 0.;
        let mut hi_y = 0.;

        for c in text.chars() {
            let sym = self.symbol_info_map.get(&(c as u32)).expect(&format!("Unexpected Character: {}", c));
            width += sym.x_advance;
            lo_y = f32::min(lo_y, sym.y_offset);
            hi_y = f32::max(hi_y, sym.y_offset);
        }

        if let Some(last_sym) = text
            .chars()
            .last()
            .and_then(|c| self.symbol_info_map.get(&(c as u32))) {
            width -= last_sym.x_advance;
            width += last_sym.width;
        }

        glam::vec2(width, hi_y - lo_y) * scale
    }
}