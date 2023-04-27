use appy::{*, components::*, types::*};

#[main_window]
pub fn app()->Elements {
	apx!{
		<bg col=0x102030/>
		<text text="Testing SDL" align=Align::Center size=Pc(10.0) col=0xffffff/>
		<blk top=Pc(0.0) left=Pc(0.0) width=Pc(25.0) height=Pc(25.0)>
			<bg col=0x00ffff/>
		</blk>
	}
}