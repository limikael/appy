use appy::{*, components::*, types::*};

#[main_window]
pub fn app()->Elements {
	vec![
		blk().left(0).width(pct(50)).children(vec![
			bg().color(0xffff00)
		]),
		blk().right(0).width(pct(50)).children(vec![
			bg().color(0xff0000)
		]),
		blk().left(0).right(0).height(32).children(vec![
			bg().color(0x000000),
			text().text("NO XML USED HERE").size(24)
		])
	]
}
