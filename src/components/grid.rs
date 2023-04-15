use appy_macros::{apx, function_component};

use super::blk::{Dim::Pc, *};
use crate::core::element::Elements;

/// Props for the [`grid`](grid()) function component.
#[derive(Clone)]
pub struct Grid {
    pub cols: usize,
    pub rows: usize,
}

impl Default for Grid {
    fn default() -> Self {
        Self { rows: 1, cols: 1 }
    }
}

/// Layout of components in a fixed grid.
///
/// The number of children should be the same as rows * columns.
/// The children is a flat array, they will be layed out two dimensionally
/// column-wise, then row-wise.
#[function_component]
pub fn grid(p: Grid, children: Elements) -> Elements {
    let mut items = vec![];

    for (i, c) in children.into_iter().enumerate() {
        let row = i / p.cols;
        let col = i % p.cols;

        items.append(&mut apx! {
            <blk
                    left=Pc(col as f32*100.0/p.cols as f32)
                    top=Pc(row as f32*100.0/p.rows as f32)
                    width=Pc(100.0/p.cols as f32)
                    height=Pc(100.0/p.rows as f32)>
                {vec![c]}
            </blk>
        });
    }

    items
}
