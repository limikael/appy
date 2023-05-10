use proc_macro::{*};
use quote::{quote,format_ident};
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
	if !table.contains_key(id) {
		return None
	}

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
	let mut ast=parse_macro_input!(input as ItemFn);
	ast.sig.ident=format_ident!("_appy_main_{}",ast.sig.ident.clone().to_string());

	let name=ast.sig.ident.clone();
	let appname=get_cargo_toml_string(vec!["package","metadata","appname"])
		.unwrap_or("Untitled".to_string());

	let mut out=quote!{#ast};

	out.extend(quote!{
		#[appy::glapp::glapp_main(appy)]
		fn main(mut app: appy::glapp::App) {
			app.title(#appname);
			//app.size(400.0,600.0);
			app.units(appy::glapp::AppUnits::DeviceIndependent);
			appy::core::Appy::new(#name).run(app);
		}

	});

	TokenStream::from(out)
}
