use std::rc::Rc;

#[derive(Clone)]
pub struct Cb {
	f: Rc<dyn Fn()>,
}

impl<T: Fn() + 'static> From <T> for Cb {
	fn from(f: T)->Cb {
		Self {
			f: Rc::new(f)
		}
	}
}

impl Default for Cb {
	fn default()->Self {
		Self {
			f: Rc::new(||{})
		}
	}
}

impl std::ops::Deref for Cb {
	type Target = Rc<dyn Fn()>;

	fn deref(&self) -> &Self::Target {
		&self.f
	}
}

#[macro_export]
macro_rules! cb_with_clone {
	($args:tt,$body:expr) => {
		{
			$crate::utils::cb::Cb::from($crate::with_clone!($args,$body))
		}
	}
}

#[derive(Clone)]
pub struct CbP<P> {
	f: Rc<dyn Fn(P)>,
}

impl<P, T: Fn(P) + 'static> From <T> for CbP<P> {
	fn from(f: T)->CbP<P> {
		Self {
			f: Rc::new(f)
		}
	}
}

impl<P> Default for CbP<P> {
	fn default()->Self {
		Self {
			f: Rc::new(|_p|{})
		}
	}
}

impl<P> std::ops::Deref for CbP<P> {
	type Target = Rc<dyn Fn(P)>;

	fn deref(&self) -> &Self::Target {
		&self.f
	}
}

#[macro_export]
macro_rules! cb_p_with_clone {
	($args:tt,$body:expr) => {
		{
			$crate::utils::cb::CbP::from($crate::with_clone!($args,$body))
		}
	}
}
