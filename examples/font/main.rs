use appy::{*, components::*, types::*, hooks::*};

#[main_window]
#[allow(unused_parens)]
pub fn app()->Elements {
	let lobster=use_font_data(||include_bytes!("./Lobster-Regular.ttf"));
	let s=use_state(||0);

	apx!{
		<Bg color=0x102030/>
		<Blk top=8 height=32>
			<Bg color=0x000000
					border_width=1 
					border_color=0xffffff
					borders=[true,false,true,false]/>
			<Text text="I'm 32dp" size=pct(100)/>
		</Blk>

		<Blk top=pct(100.0*1.0/7.0) width=pct(100) height=pct(100.0*2.0/7.0)>
			<Bg color=0x008080/>
			<Text text="Hello World" align=Align::Center color=0xffffff size=pct(100)/>
			<interaction on_click=rc_with_clone!([s],move||s.set(*s+1))/>
		</Blk>

		<Blk top=pct(100.0*4.0/7.0) width=pct(100) height=pct(100.0*2.0/7.0)>
			<Bg color=0x008080/>
			<Text text="Hello Lobster" align=Align::Center font=lobster color=0xffffff size=pct(100)/>
		</Blk>
	}
}