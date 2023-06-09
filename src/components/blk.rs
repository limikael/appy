use crate::hooks::{use_context, use_second_render_pass};
use crate::types::{Align, AppContext, Dim, VAlign};
use crate::types::{ElementWrap, Elements};
use crate::utils::{FlowBucket, FlowConf};
use appy::components::context_provider;
use appy::{derive_component, function_component, with_clone, ComponentBuilder, SnakeFactory};
use std::rc::Rc;

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
#[derive_component(ComponentBuilder, SnakeFactory)]
pub struct Blk {
    left: Dim,
    top: Dim,
    width: Dim,
    height: Dim,
    bottom: Dim,
    right: Dim,
    flow_gap: Dim,
    flow_vgap: Dim,
    flow_align: Align,
    flow_valign: VAlign,
    alpha: f32,
}

impl Blk {
    pub fn margin<T>(self: ElementWrap<Blk>, val: T) -> ElementWrap<Blk>
    where
        Dim: From<T>,
        T: Clone,
    {
        self.left(val.clone())
            .top(val.clone())
            .right(val.clone())
            .bottom(val.clone())
    }
}

impl Default for Blk {
    fn default()->Self{
        Self{
            left: Dim::default(),
            top: Dim::default(),
            width: Dim::default(),
            height: Dim::default(),
            bottom: Dim::default(),
            right: Dim::default(),
            flow_gap: Dim::default(),
            flow_vgap: Dim::default(),
            flow_align: Align::default(),
            flow_valign: VAlign::default(),
            alpha: 1.0,
            key: None,
            children: vec![]
        }
    }
}

#[function_component]
fn _blk(props: Blk) -> Elements {
    let app_context = use_context::<AppContext>();

    let h = Dim::compute_span(app_context.rect.w, props.left, props.width, props.right);
    let v = Dim::compute_span(app_context.rect.h, props.top, props.height, props.bottom);
    let new_context = app_context.abs(h.0, v.0, h.1, v.1, props.alpha);

    use_second_render_pass(Box::new(with_clone!([new_context], move || {
        let elements = new_context.flow_elements.take();
        let conf = FlowConf {
            width: new_context.rect.w,
            height: new_context.rect.h,
            gap: props.flow_gap.get_abs(),
            vgap: props.flow_vgap.get_abs(),
            align: props.flow_align,
            valign: props.flow_valign,
        };

        vec![context_provider()
            .value(Rc::new(new_context))
            .children(FlowBucket::flow(elements, conf))]
    })));

    vec![context_provider()
        .value(Rc::new(new_context))
        .children(props.children)]
}
