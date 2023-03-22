use appy::{*};

fn main() {
	Appy::run(||apx!{
		<window>
			<blk left=Px(0.0) top=Px(0.0) right=Pc(0.0) height=Px(100.0)>
				<bg col=0xff0000/>
				<text size=50.0 text="Hello..." align=Align::Center/>
			</blk>
		</window>
	});
}
