use proc_macro::{*};
use quote::quote;
use syn::{parse_macro_input, ItemFn};
use toml::Table;
use toml::value::Value;
use std::fs::read_to_string;

fn get_toml_string(table:&Table, mut path:Vec<&str>)->Option<String> {
	if path.len()==1 {
		if !table.contains_key(path[0]) {
			return None
		}

		return match table[path[0]].clone() {
			Value::String(s)=>Some(s.clone()),
			_=>None
		}
	}

	let id=path.remove(0);
	return match table[id].clone() {
		Value::Table(t)=>get_toml_string(&t,path),
		_=>None
	}
}

fn get_cargo_toml_string(path:Vec<&str>)->Option<String> {
	let config={
		let f=read_to_string("Cargo.toml");
		if f.is_ok() {
			f.unwrap().parse::<Table>().unwrap()
		}

		else {
			panic!("Unable to read Cargo.toml")
		}
	};

	get_toml_string(&config,path)
}

pub fn main_window(_attr: TokenStream, input: TokenStream) -> TokenStream {
	let ast=parse_macro_input!(input as ItemFn);
	let name=ast.sig.ident.clone();
	let appname=get_cargo_toml_string(vec!["package","metadata","appname"])
		.unwrap_or("Untitled".to_string());

	if cfg!(all(not(feature="glutin"),not(feature="sdl"))) {
		panic!("Welcome to Appy! Please enable exactly one of the features \"sdl\" or \"glutin\" to select rendering backend. Enjoy!");
	}

	let mut out=quote!{#ast};

	if cfg!(feature="glutin") {
		out.extend(quote!{
			pub fn main() {
				#[cfg(not(target_os="android"))]
				Appy::new(#name).run(&mut GlutinAppWindowBuilder::new()
					.title(#appname.to_string())
				);
			}

			#[cfg(target_os="android")]
			#[no_mangle]
			pub fn android_main(android_app: appy::AndroidApp) {
				Appy::new(#name).run(&mut GlutinAppWindowBuilder::new()
					.title(#appname.to_string())
					.with_android_app(android_app)
				);
			}
		});
	}

	if cfg!(feature="sdl") {
		out.extend(quote!{
			#[cfg(not(target_os="android"))]
			pub fn main() {
				Appy::new(#name).run(&mut SdlAppWindowBuilder::new()
					.title(#appname.to_string())
				);
			}

			#[cfg(target_os="android")]
			#[no_mangle]
			#[allow(non_snake_case)]
			pub fn SDL_main() {
				Appy::new(#name).run(&mut SdlAppWindowBuilder::new()
					.title(#appname.to_string())
				);
			}
		});
	}

	TokenStream::from(out)
}
