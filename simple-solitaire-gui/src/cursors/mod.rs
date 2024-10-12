use image::imageops::FilterType;
use image::DynamicImage;
use winit::window::CustomCursor;

#[derive(Copy, Clone, Eq, PartialEq)]
pub(crate) enum SolitaireCursor {
    Pointer,
    Grab,
    Grabbing,
}

struct CursorInfo {
    image: DynamicImage,
    size: (u16, u16),
    hotspot: (u16, u16),
}

impl CursorInfo {
    fn scale_size(&self, scale_factor: f64) -> (u16, u16) {
        let one = (self.size.0 as f64 * scale_factor).round() as u16;
        let two = (self.size.1 as f64 * scale_factor).round() as u16;

        (one, two)
    }

    fn scale_hotspot(&self, scale_factor: f64) -> (u16, u16) {
        let one = (self.hotspot.0 as f64 * scale_factor).round() as u16;
        let two = (self.hotspot.1 as f64 * scale_factor).round() as u16;

        (one, two)
    }
}

struct ScalableCursor {
    info: CursorInfo,
    cursor: CustomCursor
}
pub struct SolitaireCursors {
    scale_factor: f64,
    pointer_cursor: ScalableCursor,
    grab_cursor: ScalableCursor,
    grabbing_cursor: ScalableCursor,
}

impl ScalableCursor {
    fn create(scale_factor: f64, bytes: &[u8], size: (u16, u16), hotspot: (u16, u16), event_loop: &winit::event_loop::ActiveEventLoop) -> Self {
        // TODO: Maybe scale mouse like this? So mouse size stays in line with card size
        // let scale_factor = scale_factor * ( render_config.scale / 2.);
        let dyn_image = image::load_from_memory(bytes).expect("Bytes should represent valid cursor");
        let info = CursorInfo {
            image: dyn_image,
            size,
            hotspot,
        };

        let (w, h) = info.scale_size(scale_factor);
        let (hx, hy) = info.scale_hotspot(scale_factor);

        let initial_cursor = info.image
            .clone()
            .resize(w as u32, h as u32, FilterType::Nearest)
            .to_rgba8()
            .to_vec();

        let source = CustomCursor::from_rgba(initial_cursor, w, h, hx, hy).expect("Should be able to load cursor from rgba");
        let cursor = event_loop.create_custom_cursor(source);

        Self {
            info,
            cursor,
        }
    }

    fn resize(&mut self, scale_factor: f64, event_loop: &winit::event_loop::ActiveEventLoop) {
        let (w, h) = self.info.scale_size(scale_factor);
        let (hx, hy) = self.info.scale_hotspot(scale_factor);
        let bytes = self.info.image.clone()
            .resize(w as u32, h as u32, FilterType::Nearest)
            .to_rgba8()
            .to_vec();

        let source = CustomCursor::from_rgba(bytes, w, h, hx, hy).expect("Should be able to parse cursor");
        let cursor = event_loop.create_custom_cursor(source);

        self.cursor = cursor;
    }
}

impl SolitaireCursors {
    pub(crate) fn create(scale_factor: f64, event_loop: &winit::event_loop::ActiveEventLoop) -> Self {
        let pointer_bytes = include_bytes!("./point.png");
        let grab_bytes = include_bytes!("./grab.png");
        let grabbing_bytes = include_bytes!("./grabbing.png");

        const SIZE: (u16, u16) = (32, 32);
        const HOTSPOT: (u16, u16) = (11, 5);

        let pointer = ScalableCursor::create(scale_factor, pointer_bytes, SIZE, HOTSPOT, event_loop);
        let grab = ScalableCursor::create(scale_factor, grab_bytes, SIZE, HOTSPOT, event_loop);
        let grabbing = ScalableCursor::create(scale_factor, grabbing_bytes, SIZE, HOTSPOT, event_loop);

        Self {
            scale_factor,
            pointer_cursor: pointer,
            grab_cursor: grab,
            grabbing_cursor: grabbing,
        }
    }

    pub(crate) fn get_cursor(&self, cursor: SolitaireCursor) -> CustomCursor {
        match cursor {
            SolitaireCursor::Pointer => { self.pointer_cursor.cursor.clone() }
            SolitaireCursor::Grab => { self.grab_cursor.cursor.clone() }
            SolitaireCursor::Grabbing => { self.grabbing_cursor.cursor.clone() }
        }
    }

    pub(crate) fn resize_cursors(&mut self, scale_factor: f64, event_loop: &winit::event_loop::ActiveEventLoop) {
        if self.scale_factor == scale_factor { return; }

        self.scale_factor = scale_factor;
        self.pointer_cursor.resize(scale_factor, event_loop);
        self.grab_cursor.resize(scale_factor, event_loop);
        self.grabbing_cursor.resize(scale_factor, event_loop);
    }
}