#[cfg(target_os="android")]
pub use android_log_sys::{__android_log_write,/*LogPriority*/};

#[macro_export]
#[cfg(target_os="android")]
macro_rules! log_debug {
	($($p:expr),+)=>{
		let s=format!($($p),+).to_string();
		println!("{}",s);

		let tag=std::ffi::CString::new("SDL").unwrap();
		let text=std::ffi::CString::new(s).unwrap();
		unsafe {
			__android_log_write(3,tag.as_ptr(),text.as_ptr());
		}
	}
}

#[macro_export]
#[cfg(target_os="linux")]
macro_rules! log_debug {
	($($p:expr),+)=>{
		let s=format!($($p),+).to_string();
		println!("{}",s);
	}
}

#[macro_export]
macro_rules! log_panic {
	($($p:expr),+)=>{{
		let s=format!($($p),+).to_string();
		log_debug!("{}",s);
		panic!("{}",s);
	}}
}
