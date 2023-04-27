use appy::{*, types::*, components::*};

#[main_window]
pub fn app()->Elements {
	apx!{
		<grid cols=2>
			<blk>
				<bg color=0x800000/>
				<flow height=50>
					<text text="Hello" size=pct(100)/>
				</flow>
				<flow height=50>
					<text text="World" size=pct(100)/>
				</flow>
			</blk>
			<grid rows=2>
				<blk>
					<bg color=0x000080/>
					{
						let mut x:Elements=vec![];
						for i in 0..25 {
							x.append(&mut apx!{
								<flow width=pct(25) height=20+i*2>
									<blk margin=5>
										<bg color=i*10/>
									</blk>
								</flow>
							});
						};

						x
					}
				</blk>
				<blk>
					<bg color=0x008000/>
					{
						let mut x:Elements=vec![];
						for _i in 0..25 {
							x.append(&mut apx!{
								<flow width=40 height=40>
									<blk margin=5>
										<bg color=0xffffff/>
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