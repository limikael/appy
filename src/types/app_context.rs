use std::rc::Rc;
use std::cell::RefCell;
use crate::types::Rect;
use crate::utils::{RectRenderer, TextRenderer};

/// Information about the current application window.
///
/// Access the current [`AppContext`] from within a function component
/// with `use_context::<AppContext>()`. See [`use_context`](crate::hooks::use_context).
#[derive(Clone)]
pub struct AppContext {
    pub rect: Rect,
    pub rect_renderer: Rc<RefCell<RectRenderer>>,
    pub text_renderer: Rc<RefCell<TextRenderer>>,
    pub pixel_ratio: f32
}

impl AppContext {
    #[doc(hidden)]
    pub fn new(w: i32, h:i32, pixel_ratio:f32)->Self {
        Self {
            pixel_ratio: pixel_ratio,
            rect: Rect{x:0,y:0,w,h},
            rect_renderer: Rc::new(RefCell::new(RectRenderer::new(w,h))),
            text_renderer: Rc::new(RefCell::new(TextRenderer::new(w,h)))
        }
    }

    #[doc(hidden)]
    pub fn resize(&self, w:i32, h:i32, pixel_ratio:f32)->Self {
        let mut resized=self.clone();
        resized.rect.w=w;
        resized.rect.h=h;
        resized.pixel_ratio=pixel_ratio;
        resized.rect_renderer.borrow_mut().window_width=w;
        resized.rect_renderer.borrow_mut().window_height=h;
        resized.text_renderer.borrow_mut().window_width=w;
        resized.text_renderer.borrow_mut().window_height=h;

        resized
    }

    pub fn abs(&self, x:i32, y:i32, w:i32, h:i32)->Self {
        let mut resized=self.clone();
        resized.rect=resized.rect.abs(x,y,w,h);
        resized
    }
}
