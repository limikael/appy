use appy::{*,types::*,components::*};

#[main_window]
fn main()->Elements {
	apx!{
		<blk left=Dp(100.0) top=Dp(100.0) width=Dp(400.0) height=Dp(200.0)>
/*			<bg col=0x000080 corner_radius/>*/
			<bg color=0x000080
				borders=vec![Dp(2.0),Dp(0.0),Dp(0.0),Dp(0.0)]
				border_color=0x8080ff/>
		</blk>
	}
}
