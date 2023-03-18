use crate::{*};

trait Props {
}

#[derive(Props)]
struct MyProps {
	x: i32
}

#[function_component]
fn hell(p:MyProps) {}

#[test]
fn test_function_component() {
	hell(Props_hell{x:5});
}
