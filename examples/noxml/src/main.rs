use appy::{*, components::*, types::*};

#[main_window]
pub fn app()->Elements {
	vec![
		blk().left(0).width(pct(50)).children(vec![
			bg().color(0xffff00)
		]),
		blk().right(0).width(pct(50)).children(vec![
			bg().color(0xff0000)
		])
	]

/*	apx! {
		<Blk left=0 width=pct(50)>
			<Bg col=0xffff00/>
		</Blk>
		{
			apx!{
				<Blk right=0 width=pct(50)>
					<Bg col=0xff0000/>
				</Blk>
			}
		}
	}*/
}
