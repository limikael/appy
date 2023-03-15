use std::any::TypeId;
use std::any::Any;

/*trait Typed {
	fn get_type_id(&self)->TypeId;
}

trait Tr: Typed {
}

struct Sr {}
impl Tr for Sr {}
impl Typed for Sr {
	fn get_type_id(&self)->TypeId {
		TypeId::of::<Self>()
	}
}

struct Qr {}
impl Tr for Qr {}
impl Typed for Qr {
	fn get_type_id(&self)->TypeId {
		TypeId::of::<Self>()
	}
}*/

struct Test {
}

impl<T:Any> Any for Test {

}

fn main() {
/*	let v:Vec<Box<dyn Tr>>=vec![Box::new(Sr{}),Box::new(Qr{}),Box::new(Qr{})];

	println!("{:?}",v[0].type_id());
	println!("{:?}",v[1].type_id());
	println!("{:?}",v[2].type_id());*/

//	let p=hello as usize;

//	println!("{}",p);
}