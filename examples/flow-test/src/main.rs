use appy::{*, types::*, components::*};

#[main_window]
pub fn app()->Elements {
	apx!{
		<grid cols=2>
			<blk>
				<bg col=0x800000/>
				<flow height=Dp(50.0)>
					<text text="Hello".to_string() size=Pc(100.0)/>
				</flow>
				<flow height=Dp(50.0)>
					<text text="World".to_string() size=Pc(100.0)/>
				</flow>
			</blk>
			<grid rows=2>
				<blk>
					<bg col=0x000080/>
					{
						let mut x:Elements=vec![];
						for i in 0..25 {
							x.append(&mut apx!{
								<flow width=Pc(25.0) height=Dp(20.0+i as f32*2.0)>
									<blk left=Dp(5.0) top=Dp(5.0) right=Dp(5.0) bottom=Dp(5.0)>
										<bg col=i*10/>
									</blk>
								</flow>
							});
						};

						x
					}
				</blk>
				<blk>
					<bg col=0x008000/>
					{
						let mut x:Elements=vec![];
						for _i in 0..25 {
							x.append(&mut apx!{
								<flow width=Dp(40.0) height=Dp(40.0)>
									<blk left=Dp(5.0) top=Dp(5.0) right=Dp(5.0) bottom=Dp(5.0)>
										<bg col=0xffffff/>
									</blk>
								</flow>
							});
						};

						x
					}
				</blk>
			</grid>
		</grid>
	}
}