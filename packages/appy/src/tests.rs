use crate::{*};
use std::rc::Rc;

// dyn Fn()

#[derive(Clone)]
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
}

#[derive(Clone)]
struct CbP<P> {
    f: Rc<dyn Fn(P)>,
}

impl<P, T: Fn(P) + 'static> From <T> for CbP<P> {
	fn from(f: T)->CbP<P> {
	    Self {
	        f: Rc::new(f) //||println!("hello")),
	    }
	}
}

impl<P> std::ops::Deref for CbP<P> {
    type Target = Rc<dyn Fn(P)>;

    fn deref(&self) -> &Self::Target {
        &self.f
    }
}

#[test]
fn test() {
	let cb=Cb::from(||println!("hello..."));
	cb();

	let cb=CbP::from(|i:(i32)|println!("hello: {}",i));
	cb((123));
}
