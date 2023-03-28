use appy::{*};

#[appy_main]
//#[allow(unused_braces)]
pub fn app()->Elements {
	apx!{
		<window>
			<bg col=0x800000/>
			<text text="Hello".to_string() align=Align::Center/>
		</window>
	}
}