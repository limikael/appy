use std::rc::Rc;
use std::cell::RefCell;
use std::cmp::max;
use crate::types::{Rect, Dim, Font};
use crate::utils::{RectRenderer, TextRenderer, ImageRenderer};

struct FlowAnchor {
    x: i32,
    y: i32,
    line_height: i32
}

impl FlowAnchor {
    pub fn new()->Self {
        Self {
            x: 0,
            y: 0,
            line_height: 0
        }
    }

    pub fn advance(&mut self, w:i32, h:i32, max_w:i32)->(i32,i32) {
        if self.x+w>max_w {
            self.x=0;
            self.y+=self.line_height;
            self.line_height=0;
        }

        let res=(self.x,self.y);

        self.x+=w;
        self.line_height=max(self.line_height,h);

        res
    }
}

/// Information about the current application window.
///
/// Access the current [`AppContext`] from within a function component
/// with `use_context::<AppContext>()`. See [`use_context`](crate::hooks::use_context).
#[derive(Clone)]
pub struct AppContext {
    flow_anchor: Rc<RefCell<FlowAnchor>>,
    pub pixel_ratio: f32,
    pub rect: Rect,
    pub rect_renderer: Rc<RefCell<RectRenderer>>,
    pub text_renderer: Rc<RefCell<TextRenderer>>,
    pub image_renderer: Rc<RefCell<ImageRenderer>>,
    pub default_font: Rc<Font>
}

impl AppContext {
    #[doc(hidden)]
    pub fn new(w: i32, h:i32, pixel_ratio:f32, default_font:Font)->Self {
        Self {
            pixel_ratio: pixel_ratio,
            rect: Rect{x:0,y:0,w,h},
            rect_renderer: Rc::new(RefCell::new(RectRenderer::new(w,h))),
            text_renderer: Rc::new(RefCell::new(TextRenderer::new(w,h))),
            image_renderer: Rc::new(RefCell::new(ImageRenderer::new(w,h))),
            flow_anchor: Rc::new(RefCell::new(FlowAnchor::new())),
            default_font: Rc::new(default_font)
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
        resized.image_renderer.borrow_mut().set_size(w,h);

        resized
    }

    pub fn abs(&self, x:i32, y:i32, w:i32, h:i32)->Self {
        let mut resized=self.clone();
        resized.flow_anchor=Rc::new(RefCell::new(FlowAnchor::new()));
        resized.rect=resized.rect.abs(x,y,w,h);
        resized
    }

    pub fn reset_flow(&self) {
        *self.flow_anchor.borrow_mut()=FlowAnchor::new();
    }

    pub fn advance_flow(&self, w:i32, h:i32)->(i32,i32) {
        self.flow_anchor.borrow_mut().advance(w,h,self.rect.w)
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
