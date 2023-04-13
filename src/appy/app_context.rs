use crate::*;

/// Information about the current application window.
///
/// Access the current [`AppContext`] from within a function component
/// with `use_context::<AppContext>()`. See [`use_context`].
pub struct AppContext {
    pub rect: Rect,
    pub rect_renderer: RectRenderer,
    pub text_renderer: TextRenderer,
    pub pixel_ratio: f32
}

impl AppContext {
    #[doc(hidden)]
    pub fn new(w: i32, h:i32, pixel_ratio:f32)->Self {
        Self {
            pixel_ratio: pixel_ratio,
            rect: Rect{x:0,y:0,w,h},
            rect_renderer: RectRenderer::new(w,h),
            text_renderer: TextRenderer::new(w,h)
        }
    }

    #[doc(hidden)]
    pub fn set_size(&mut self, w:i32, h:i32) {
        self.rect.w=w;
        self.rect.h=h;
        self.rect_renderer.window_width=w;
        self.rect_renderer.window_height=h;
        self.text_renderer.window_width=w;
        self.text_renderer.window_height=h;
    }
}
