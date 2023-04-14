use gl::types::{*};
use std::ffi::CStr;

extern "system" fn error_callback(
		_source: GLenum, _gltype: GLenum, _id: GLuint, severity: GLenum, 
		_length: GLsizei, message: *const GLchar, _user_param: *mut GLvoid) {
	let c_str:&CStr=unsafe {CStr::from_ptr(message)};
    let str_slice:&str=c_str.to_str().unwrap();
    let s:String=str_slice.to_owned();

	println!("** GL({}): {}",severity,s);
}

pub fn install_debug_output() {
	unsafe {
		gl::Enable(gl::DEBUG_OUTPUT);
		gl::DebugMessageCallback(Some(error_callback), std::ptr::null());
	}
}