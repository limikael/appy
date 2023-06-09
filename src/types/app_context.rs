use crate::{components::*, types::*, utils::*};
use std::cell::RefCell;
use std::rc::Rc;

/// Information about the current application window.
///
/// Access the current [`AppContext`] from within a function component
/// with `use_context::<AppContext>()`. See [`use_context`](crate::hooks::use_context).
#[derive(Clone)]
pub struct AppContext {
    pub flow_elements: Rc<RefCell<Vec<Flow>>>,
    pub pixel_ratio: f32,
    pub rect: Rect,
    pub rect_renderer: Rc<RectRenderer>,
    pub text_renderer: Rc<RefCell<TextRenderer>>,
    pub image_renderer: Rc<RefCell<ImageRenderer>>,
    pub default_font: Rc<Font>,
    pub viewport_size: (f32, f32),
    pub alpha: f32,
}

impl AppContext {
    #[doc(hidden)]
    pub fn new(w: f32, h: f32, pixel_ratio: f32, default_font: Font) -> Self {
        Self {
            pixel_ratio: pixel_ratio,
            viewport_size: (w, h),
            rect: Rect { x: 0., y: 0., w, h },
            rect_renderer: Rc::new(RectRenderer::new()),
            text_renderer: Rc::new(RefCell::new(TextRenderer::new(w, h, pixel_ratio))),
            image_renderer: Rc::new(RefCell::new(ImageRenderer::new(w, h))),
            flow_elements: Rc::new(RefCell::new(vec![])),
            default_font: Rc::new(default_font),
            alpha: 1.0
        }
    }

    #[doc(hidden)]
    pub fn resize(&self, w: f32, h: f32, pixel_ratio: f32) -> Self {
        let mut resized = self.clone();
        resized.viewport_size = (w, h);
        resized.rect.w = w;
        resized.rect.h = h;
        resized.pixel_ratio = pixel_ratio;
        resized.text_renderer.borrow_mut().window_width = w;
        resized.text_renderer.borrow_mut().window_height = h;
        resized.image_renderer.borrow_mut().set_size(w, h);
        resized.flow_elements = Rc::new(RefCell::new(vec![]));

        resized
    }

    pub fn abs(&self, x: f32, y: f32, w: f32, h: f32, alpha: f32) -> Self {
        let mut resized = self.clone();
        resized.flow_elements = Rc::new(RefCell::new(vec![]));
        resized.rect = resized.rect.abs(x, y, w, h);
        resized.alpha *= alpha;
        resized
    }

    pub fn begin_frame(&self) {
        self.flow_elements.replace(vec![]);
        self.text_renderer.borrow_mut().begin_frame();
    }

    pub fn end_frame(&self) {
        self.text_renderer.borrow_mut().end_frame();
    }
}
