use std::rc::Rc;
use std::cell::RefCell;
use crate::{types::*, utils::*};

/// Information about the current application window.
///
/// Access the current [`AppContext`] from within a function component
/// with `use_context::<AppContext>()`. See [`use_context`](crate::hooks::use_context).
#[derive(Clone)]
pub struct AppContext {
    pub flow_bucket: Rc<RefCell<FlowBucket<Elements>>>,
    pub pixel_ratio: f32,
    pub rect: Rect<i32>,
    pub rect_renderer: Rc<RectRenderer>,
    pub text_renderer: Rc<RefCell<TextRenderer>>,
    pub image_renderer: Rc<RefCell<ImageRenderer>>,
    pub default_font: Rc<Font>,
    pub viewport_size: (u32,u32),
}

impl AppContext {
    #[doc(hidden)]
    pub fn new(w: i32, h:i32, pixel_ratio:f32, default_font:Font)->Self {
        Self {
            pixel_ratio: pixel_ratio,
            viewport_size: (w as u32,h as u32),
            rect: Rect{x:0,y:0,w,h},
            rect_renderer: Rc::new(RectRenderer::new()),
            text_renderer: Rc::new(RefCell::new(TextRenderer::new(w,h))),
            image_renderer: Rc::new(RefCell::new(ImageRenderer::new(w,h))),
            flow_bucket: Rc::new(RefCell::new(FlowBucket::new(w as f32,h as f32))),
            default_font: Rc::new(default_font)
        }
    }

    #[doc(hidden)]
    pub fn resize(&self, w:i32, h:i32, pixel_ratio:f32)->Self {
        let mut resized=self.clone();
        resized.viewport_size=(w as u32,h as u32);
        resized.rect.w=w;
        resized.rect.h=h;
        resized.pixel_ratio=pixel_ratio;
        resized.text_renderer.borrow_mut().window_width=w;
        resized.text_renderer.borrow_mut().window_height=h;
        resized.image_renderer.borrow_mut().set_size(w,h);
        resized.flow_bucket=Rc::new(RefCell::new(FlowBucket::new(w as f32,h as f32)));

        resized
    }

    pub fn abs(&self, x:i32, y:i32, w:i32, h:i32)->Self {
        let mut resized=self.clone();
        resized.flow_bucket=Rc::new(RefCell::new(FlowBucket::new(w as f32,h as f32)));
        resized.rect=resized.rect.abs(x,y,w,h);
        resized
    }

    pub fn reset_flow(&self) {
        *self.flow_bucket.borrow_mut()=FlowBucket::new(self.rect.w as f32,self.rect.h as f32);
    }

    pub fn compute_h_span(&self, start: Dim, size: Dim, end: Dim)->(f32, f32) {
        Dim::compute_span(self.rect.w as f32, self.pixel_ratio, start, size, end)
    }

    pub fn compute_v_span(&self, start: Dim, size: Dim, end: Dim)->(f32, f32) {
        Dim::compute_span(self.rect.h as f32, self.pixel_ratio, start, size, end)
    }

    pub fn compute_h_px(&self, val: Dim)->f32 {
        val.to_px(self.rect.w as f32,self.pixel_ratio)
    }

    pub fn compute_v_px(&self, val: Dim)->f32 {
        val.to_px(self.rect.h as f32,self.pixel_ratio)
    }
}
