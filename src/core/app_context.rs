use crate::utils::rect::Rect;
use crate::utils::rect_renderer::RectRenderer;
use crate::utils::text_renderer::TextRenderer;

pub struct AppContext {
    pub rect: Rect,
    pub rect_renderer: RectRenderer,
    pub text_renderer: TextRenderer,
    pub pixel_ratio: f32
}

impl AppContext {
    pub fn new(w: i32, h:i32, pixel_ratio:f32)->Self {
        Self {
            pixel_ratio: pixel_ratio,
            rect: Rect{x:0,y:0,w,h},
            rect_renderer: RectRenderer::new(w,h),
            text_renderer: TextRenderer::new(w,h)
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
