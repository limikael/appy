use crate::*;

pub struct AppContext {
    pub rect: Rect,
    pub rect_renderer: RectRenderer,
    pub text_renderer: TextRenderer
}

impl AppContext {
    pub fn new()->Self {
        Self {
            rect: Rect::empty(),
            rect_renderer: RectRenderer::new(),
            text_renderer: TextRenderer::new()
        }
    }

    pub fn set_size(&mut self, w:i32, h:i32) {
        self.rect.w=w;
        self.rect.h=h;
        self.rect_renderer.window_width=w;
        self.rect_renderer.window_height=h;
        self.text_renderer.window_width=w;
        self.text_renderer.window_height=h;
    }
}
