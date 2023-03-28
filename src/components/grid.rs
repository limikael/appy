use crate::{*};

#[derive(Clone)]
pub struct Grid {
	pub cols: usize,
	pub rows: usize
}

impl Default for Grid {
	fn default()->Self {
		Self{
			rows: 1,
			cols: 1
		}
	}
}

#[function_component]
pub fn grid(p: Grid, children: Elements)->Elements {
	let mut items=vec![];

	let mut i=0;
	for c in children {
		let row=i/p.cols;
		let col=i%p.cols;

		items.append(&mut apx!{
			<blk
					left=Pc(col as f32*100.0/p.cols as f32)
					top=Pc(row as f32*100.0/p.rows as f32)
					width=Pc(100.0/p.cols as f32) 
					height=Pc(100.0/p.rows as f32)>
				{vec![c]}
			</blk>
		});

		i+=1;
	}

	items
}
