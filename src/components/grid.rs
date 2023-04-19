use crate::{function_component,apx,derive_component,SnakeFactory,ComponentBuilder};
use crate::types::{Elements, Dim::*};
use crate::components::blk;

/// Layout of components in a fixed grid.
///
/// The number of children should be the same as rows * columns.
/// The children is a flat array, they will be layed out two dimensionally
/// column-wise, then row-wise.
#[derive_component(ComponentBuilder,Default,SnakeFactory)]
pub struct Grid {
    cols: usize,
    rows: usize,
}

#[function_component]
fn _grid(props:Grid)->Elements {
    let rows=if props.rows>=1 {props.rows} else {1};
    let cols=if props.cols>=1 {props.cols} else {1};

    let mut items = vec![];

    for (i, c) in props.children.into_iter().enumerate() {
        let row = i / cols;
        let col = i % cols;

        items.append(&mut apx! {
            <blk
                    left=Pc(col as f32*100.0/cols as f32)
                    top=Pc(row as f32*100.0/rows as f32)
                    width=Pc(100.0/cols as f32)
                    height=Pc(100.0/rows as f32)>
                {vec![c]}
            </blk>
        });
    }

    items
}
