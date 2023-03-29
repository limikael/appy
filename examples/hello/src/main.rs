use appy::{*};

#[main_window]
pub fn app()->Elements {
	apx!{
		<bg col=0x800000/>
		<text text="Hello World".to_string() align=Align::Center size=Pc(10.0)/>
	}
}