use appy::{*, components::*, types::*};

#[main_window]
pub fn app()->Elements {
	apx!{
		<bg color=0x102030/>
		<text text="Testing SDL" align=Align::Center size=pct(10) color=0xffffff/>
		<blk top=0 left=0 width=pct(25.0) height=pct(25.0)>
			<bg color=0x00ffff/>
		</blk>
	}
}