#![allow(clippy::needless_update)]

use appy::components::{blk::*, text::*, bg::*};
use appy::components::blk::Dim::*;
use appy::{main_window, apx};
use appy::core::element::Elements;

#[main_window]
pub fn app()->Elements {
	apx!{
		<bg col=0x102030/>
		<blk top=Dp(32.0) height=Dp(32.0)>
			<bg col=0x000000/>
			<text text="I am 32 dp".to_string() align=Align::Center size=Pc(100.0)/>
		</blk>
		<blk top=Pc(40.0) width=Pc(100.0) height=Pc(20.0)>
			<bg col=0x008080/>
		</blk>
		<text text="Hello World".to_string() align=Align::Center size=Pc(10.0)/>
	}
}