use appy::{*, components::*, types::*, hooks::*};

#[main_window]
pub fn app()->Elements {
	let font_face=use_font_face(||include_bytes!("./Roboto-Regular.ttf"));
	let font=use_font(font_face,100.0);

	apx!{
		<Bg col=0x102030/>
		<Blk top=Pc(40.0) width=Pc(100.0) height=Dp(100.0)>
			<Bg col=0x008080/>
			<Text text="Hello World".to_string() align=Align::Center font=font col=0xffffff/>
		</Blk>
	}
}