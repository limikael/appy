/// Clone variables for closure.
///
/// Convenience macro for cloning variables, especially intended to be used
/// in closures. For example, the following:
/// ```rust
/// use appy::with_clone;
/// let a=1;
/// let b=2;
/// with_clone!([a,b],||{
///   // ..
/// });
/// ```
/// Will expand to:
/// ```rust
/// use appy::with_clone;
/// let a=1;
/// let b=2;
/// {
///   let a=a.clone();
///   let b=b.clone();
///   ||{
///     // ..
///   }
/// };
/// ```
#[macro_export]
macro_rules! with_clone {
	([$ ($var:ident), *],$body:expr) => {
		{
			$(let $var=($var).clone();)*
			$body
		}
	}
}

/// Clone variables for closure, and create Rc.
///
/// Does the same as [with_clone], and then also applies `Rc::new` 
/// on the result.
#[macro_export]
macro_rules! rc_with_clone {
	($args:tt,$body:expr) => {
		{
			std::rc::Rc::new($crate::with_clone!($args,$body))
		}
	}
}
