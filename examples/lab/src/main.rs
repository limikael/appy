use appy::{*};

/*pub fn main() {
	let a=[[1,2,3],[4,5,6]];

	a.flatten();
}*/

#[appy_main]
#[allow(unused_braces)]
pub fn app()->Elements {
	let u=apx!{<blk/><blk/>};
	let t=apx!{<blk/><blk/>{u}};

//	t.append(&mut u);

	println!("{:?}",t.len());

	apx!{<window/>}

/*	apx!{
		<window>
			<bg col=0x800000/>
			<text text="Hello".to_string() align=Align::Center/>
		</window>
	}*/
}