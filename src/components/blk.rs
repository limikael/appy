use crate::core::element::ElementWrap;
use crate::core::element::Element;
use crate::core::app_context::AppContext;
use crate::core::element::Elements;
use crate::core::hooks::{use_context, use_post_render};
use std::rc::Rc;
use appy::{derive_component,SnakeFactory,ComponentBuilder,with_clone};

/// Specify dimension.
#[derive(Default, Clone)]
pub enum Dim {
    /// No dimension specified.
    #[default]
    None,

    /// Percentual size relative to parent.
    Pc(f32),

    /// Absolute size specified in hardware pixels.
    Px(f32),

    /// Size specified in device independent pixels.
    /// This is the same as hardware pixels, scaled with
    /// a factor defined by the device.
    Dp(f32)
}

impl Dim {
    pub fn is_some(&self) -> bool {
        !matches!(*self, Dim::None)
    }

    pub fn to_px(&self, max: f32, pixel_ratio: f32) -> f32 {
        match *self {
            Dim::Px(v) => v,
            Dim::Pc(v) => max * v / 100.0,
            Dim::Dp(v) => v * pixel_ratio,
            Dim::None => panic!("can't get px from none"),
        }
    }

    pub fn compute_span(max: f32, pr: f32, start: Dim, size: Dim, end: Dim) -> (f32, f32) {
        let c = if start.is_some() { 1 << 2 } else { 0 }
            + if size.is_some() { 1 << 1 } else { 0 }
            + if end.is_some() { 1 << 0 } else { 0 };

        match c {
            0b000 => (0.0, max),
            0b001 => (0.0, max - end.to_px(max,pr)),
            0b010 => ((max - size.to_px(max,pr)) / 2.0, size.to_px(max,pr)),
            0b011 => (max - size.to_px(max,pr) - end.to_px(max,pr), size.to_px(max,pr)),
            0b100 => (start.to_px(max,pr), max - start.to_px(max,pr)),
            0b101 => (start.to_px(max,pr), max - start.to_px(max,pr) - end.to_px(max,pr)),
            0b110 => (start.to_px(max,pr), size.to_px(max,pr)),
            0b111 => panic!("over constrained"),
            _ => panic!("bad constraints"),
        }
    }
}

impl Dim {}

/// Positions a block relative to the parent.
///
/// For each dimension (horizontal vs. vertical) there are three values. E.g.
/// in the vertical direction there is top, height and bottom. In the 
/// horizontal direction there is left, width and right. You need to specify
/// two values in each direction, but not all three.
///
/// For example, if you specify left and right, the block will be fixed relative
/// to the left and right edges, and hence the size will be dynamic and change
/// relative to the parent size.
///
/// If you specify left and width, the block will be fixed relative to the left
/// edge with a fixed size (i.e., the distance to the right edge will be dynamic).
#[derive_component(ComponentBuilder,Default,SnakeFactory)]
pub struct Blk {
    left: Dim,
    top: Dim,
    width: Dim,
    height: Dim,
    bottom: Dim,
    right: Dim,
}

impl Element for Blk {
    fn render(self: ElementWrap<Blk>)->Elements {
        let instance_ref = use_context::<AppContext>();
        let mut instance = instance_ref.borrow_mut();

        let old_rect = instance.rect.clone();
        let h = Dim::compute_span(old_rect.w as f32, instance.pixel_ratio, self.left, self.width, self.right);
        let v = Dim::compute_span(old_rect.h as f32, instance.pixel_ratio, self.top, self.height, self.bottom);

        instance.rect = instance
            .rect
            .abs(h.0 as i32, v.0 as i32, h.1 as i32, v.1 as i32);

        use_post_render(Rc::new(with_clone!([instance_ref], move || {
            let mut instance = instance_ref.borrow_mut();
            instance.rect = old_rect.clone();
        })));

        self.children
    }
}
