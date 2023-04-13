#[macro_export]
macro_rules! with_clone {
	([$ ($var:ident), *],$body:expr) => {
		{
			$(let $var=($var).clone();)*
			$body
		}
	}
}

#[macro_export]
macro_rules! rc_with_clone {
	($args:tt,$body:expr) => {
		{
			Rc::new($crate::with_clone!($args,$body))
		}
	}
}
