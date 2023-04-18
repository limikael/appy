use crate::types::{Element, Elements, ElementWrap};
use crate::types::{AppContext, Dim};
use crate::hooks::{use_context, use_post_render};
use std::rc::Rc;
use appy::{derive_component,SnakeFactory,ComponentBuilder,with_clone};

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
