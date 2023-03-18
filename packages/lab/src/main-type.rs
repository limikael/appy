//#[derive(Props)]
struct MyProps {
	x: i32
}

//#[function_component]
fn my_comp(p: MyProps) {

}

type Props_my_comp=MyProps;

fn main() {
	let props=Props_my_comp{x: 5};
	my_comp(props);
}