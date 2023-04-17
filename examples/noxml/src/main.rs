use appy::{main_window, apx};
use appy::core::element::Elements;
use appy::components::bg::*;
use appy::components::blk::*;
use crate::Dim::Pc;

#[main_window]
pub fn app()->Elements {
/*	vec![
		blk().left(Pc(0.0)).width(Pc(50.0)).child(
			bg().col(0xffff00)
		),
		blk().right(Pc(0.0)).width(Pc(50.0)).children(vec![
			bg().col(0xff0000)
		])
	]*/

	apx! {
		<Blk left=Pc(0.0) width=Pc(50.0)>
			<Bg col=0xffff00/>
		</Blk>
		{
			apx!{
				<Blk right=Pc(0.0) width=Pc(50.0)>
					<Bg col=0xff0000/>
				</Blk>
			}
		}
	}
}
