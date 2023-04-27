use appy::{*, components::*, types::*, hooks::*};

#[main_window]
pub fn app()->Elements {
	let lobster=use_font_data(||include_bytes!("./Lobster-Regular.ttf"));
	let s=use_state(||0);

	apx!{
		<Bg col=0x102030/>
		<Blk top=Pc(100.0*1.0/7.0) width=Pc(100.0) height=Pc(100.0*2.0/7.0)>
			<Bg col=0x008080/>
			<Text text="Hello World".to_string() align=Align::Center col=0xffffff size=Pc(100.0)/>
			<interaction on_click=rc_with_clone!([s],move||s.set(*s+1))/>
		</Blk>

		<Blk top=Pc(100.0*4.0/7.0) width=Pc(100.0) height=Pc(100.0*2.0/7.0)>
			<Bg col=0x008080/>
			<Text text="Hello World".to_string() align=Align::Center font=lobster col=0xffffff size=Pc(100.0)/>
		</Blk>
	}
}