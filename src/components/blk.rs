use crate::types::{Elements,ElementWrap};
use crate::types::{AppContext, Dim};
use crate::hooks::{use_context,use_second_render_pass};
use std::rc::Rc;
use appy::{function_component,derive_component,SnakeFactory,ComponentBuilder};
use appy::components::context_provider;

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

impl Blk {
    pub fn margin<T>(self:ElementWrap<Blk>, val:T)->ElementWrap<Blk>
            where Dim: From<T>, T: Clone {
        self
            .left(val.clone())
            .top(val.clone())
            .right(val.clone())
            .bottom(val.clone())
    }
}

#[function_component]
fn _blk(props:Blk)->Elements {
    let app_context = use_context::<AppContext>();

    let h=Dim::compute_span(app_context.rect.w,props.left,props.width,props.right);
    let v=Dim::compute_span(app_context.rect.h,props.top,props.height,props.bottom);
    let new_context=app_context.abs(h.0,v.0,h.1,v.1);

    let c=new_context.clone();
    use_second_render_pass(Box::new(move||{
        let mut flow_children:Elements=vec![];

        c.flow_bucket.borrow_mut().with_items(|e,x,y,w,h|{
            flow_children.push(blk()
                .left(x)
                .width(w)
                .top(y)
                .height(h)
                .children(e)
            );
        });

        if flow_children.len()==0 {
            return flow_children;
        }

        vec![
            context_provider()
                .value(Rc::new(c))
                .children(flow_children)
        ]
    }));

    vec![
        context_provider()
            .value(Rc::new(new_context))
            .children(props.children)
    ]
}
