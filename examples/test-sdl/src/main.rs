#![allow(clippy::needless_update)]

use appy::components::blk::Dim::*;
use appy::components::{bg::*, blk::*, text::*};
use appy::{apx, main_window};

use appy::core::element::Elements;

#[main_window]
pub fn app()->Elements {
	apx!{
		<bg col=0x102030/>
		<text text="Testing SDL".to_string() align=Align::Center size=Pc(10.0) col=0xffffff/>
		<blk top=Pc(0.0) left=Pc(0.0) width=Pc(25.0) height=Pc(25.0)>
			<bg col=0x00ffff/>
		</blk>
	}
}