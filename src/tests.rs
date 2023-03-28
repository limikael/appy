use crate::{*};
use std::rc::Rc;
use std::any::Any;

// dyn Fn()

/*#[derive(Clone)]
struct Cb {
    f: Rc<dyn Fn()>,
}

impl<T: Fn() + 'static> From <T> for Cb {
	fn from(f: T)->Cb {
	    Self {
	        f: Rc::new(f) //||println!("hello")),
	    }
	}
}

impl std::ops::Deref for Cb {
    type Target = Rc<dyn Fn()>;

    fn deref(&self) -> &Self::Target {
        &self.f
    }
}*/

pub struct NoParam {}
pub const NO_PARAM:NoParam=NoParam{};

#[derive(Clone)]
struct CbX<P> {
    f: Rc<dyn Fn(P)>,
}

impl<P, T: Fn(P) + 'static> From <T> for CbX<P> {
	fn from(f: T)->CbX<P> {
	    Self {
	        f: Rc::new(f) //||println!("hello")),
	    }
	}
}

impl<P> std::ops::Deref for CbX<P>
		where P:Any {
    type Target = Rc<dyn Fn(P)>;

    fn deref(&self) -> &Self::Target {
        &self.f
    }
}

/*impl<P> std::ops::Deref for CbX<P>
		where P:Any {
    type Target = Rc<dyn Fn(P)>;

    fn deref(&self) -> &Self::Target {
        &self.f
    }
}*/

#[test]
fn test() {
	/*let cb=Cb::from(||println!("hello..."));
	cb();*/

	let cb:CbX<NoParam>=CbX::from(|_|println!("hello"));
	cb(NO_PARAM);
}
