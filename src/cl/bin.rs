/*

todo:
- specify main function
- shoud be: (can use APPY_PLATFORM)
    appy build --platform android
    appy run --platfom android

override cargo file?

https://users.rust-lang.org/t/rustc-vs-cargo-which-is-better-to-build/35419/3
https://doc.rust-lang.org/stable/cargo/commands/cargo-rustc.html
rustflags

*/

use std::fs::{create_dir_all, read_to_string, write, copy};
use std::env;
use std::process::Command;
use std::path::{Path, PathBuf};
use fs_extra::{copy_items, dir::CopyOptions, remove_items};
use toml::Table;
use toml::value::Value;
use symlink::symlink_dir;

fn get_env_var(key:&str)->String {
	for (k,v) in env::vars() {
		if k==key {
			return v;
		}
	}

	panic!("Need env var: {}",key);
}

fn path_concat(a:Vec<&str>)->String {
	let mut buf=PathBuf::new();
	for s in a {
//		buf.push(s);
		buf.push(s.to_string());
	}
	buf.into_os_string().into_string().unwrap()
}

fn toml_insert(mut config: Table, mut path: Vec<&str>, v:Value)->Table {
	if path.len()==1 {
		config.insert(path[0].to_string(),v);
		config
	}

	else {
		let component=path.remove(0);
		let mut child=Table::new();

		if config.contains_key(component) { 
			if let Value::Table(c)=config[component].clone() {
				child=c;
			}
		}

		let t=toml_insert(child,path,v);
		config.insert(component.to_string(),Value::Table(t));
		config
	}
}

/*fn download_sdl() {
	create_dir_all("target/appy").unwrap();
	if !Path::new("target/appy/SDL").is_dir() {
		assert!(Command::new("git")
			.args(["clone","-b","release-2.26.x","--single-branch","https://github.com/libsdl-org/SDL.git","target/appy/SDL"])
			.status()
			.unwrap()
			.success());
	}
	}*/

fn build_sdl_for_android() {
	let p=Path::new(&*get_env_var("ANDROID_NDK_HOME")).join("ndk-build");

	assert!(Command::new(p)
		.args(["NDK_PROJECT_PATH=.","APP_BUILD_SCRIPT=./Android.mk","APP_PLATFORM=android-18"])
		.current_dir(&*get_env_var("SDL"))
		.status()
		.unwrap()
		.success());
}

fn create_android_cargo_config() {
	let mut config={
		let f=read_to_string(".cargo/config.toml");
		if f.is_ok() {
			f.unwrap().parse::<Table>().unwrap()
		}

		else {
			Table::new()
		}
	};

	let tool_configs=vec![
		(vec!["target","aarch64-linux-android","ar"],
			"toolchains/llvm/prebuilt/linux-x86_64/bin/aarch64-linux-android-ar"),

		(vec!["target","aarch64-linux-android","linker"],
			"toolchains/llvm/prebuilt/linux-x86_64/bin/aarch64-linux-android26-clang"),

		(vec!["target","armv7-linux-androideabi","ar"],
			"toolchains/llvm/prebuilt/linux-x86_64/bin/arm-linux-androideabi-ar"),

		(vec!["target","armv7-linux-androideabi","linker"],
			"toolchains/llvm/prebuilt/linux-x86_64/bin/armv7a-linux-androideabi26-clang"),

		(vec!["target","i686-linux-android","ar"],
			"toolchains/llvm/prebuilt/linux-x86_64/bin/i686-linux-android-ar"),

		(vec!["target","i686-linux-android","linker"],
			"toolchains/llvm/prebuilt/linux-x86_64/bin/i686-linux-android26-clang"),
	];

	for (path,value) in tool_configs {
		config=toml_insert(config,path,Value::String(path_concat(vec![
			&*get_env_var("ANDROID_NDK_HOME"),
			value
		])));
	}

	create_dir_all(".cargo").unwrap();
	write(".cargo/config.toml",config.to_string()).expect("Unable to write file");
}

fn get_android_targets()->Vec<(&'static str,&'static str)> {
	vec![
		("arm64-v8a","aarch64-linux-android"),
		("armeabi-v7a","armv7-linux-androideabi"),
		("x86","i686-linux-android"),
	]
}

fn copy_android_sdl_deps() {
	for (android_name,rust_name) in get_android_targets() {
		let rust_dir=path_concat(vec![
			"target",rust_name,"debug","deps"
		]);

		create_dir_all(rust_dir).unwrap();
		copy(
			Path::new(&*path_concat(vec![
				&*get_env_var("SDL"),"libs",android_name,"libSDL2.so"
			])),
			Path::new(&*path_concat(vec![
				"target",rust_name,"debug","deps","libSDL2.so"
			]))
		).unwrap();
	}
}

fn build_android_targets() {
	for (_android_name,rust_name) in get_android_targets() {
		assert!(Command::new("cargo")
			.args(["build","--lib","--target",rust_name])
			.status()
			.unwrap()
			.success());
	}
}

fn get_android_app_id()->String {
	let config={
		let f=read_to_string("Cargo.toml");
		if f.is_ok() {
			f.unwrap().parse::<Table>().unwrap()
		}

		else {
			Table::new()
		}
	};

	if let Value::String(id)=config["package"]["metadata"]["appid"].clone() {
		id
	}

	else {
		panic!("no app id");
	}
}


fn create_android_project() {
	let appid=get_android_app_id();

	copy_items(
		&vec![Path::new(&*path_concat(vec![&*get_env_var("SDL"),"android-project"]))],
		Path::new(&*path_concat(vec!["target"])),
		&CopyOptions::new().skip_exist(true)
	).unwrap();

	let java_main_folder=Path::new("target/android-project/app/src/main/java").join(str::replace(&*appid,".","/"));
	create_dir_all(java_main_folder.clone()).unwrap();
	let main_class="
		package $APP;

		import org.libsdl.app.SDLActivity;

		public class MainActivity extends SDLActivity {
		}
	";
	let main_class=str::replace(main_class,"$APP",&*appid);
	write(java_main_folder.join("MainActivity.java"),main_class.to_string()).expect("Unable to write file");

	let manifest=read_to_string(path_concat(vec![
		&*get_env_var("SDL"),
		"android-project/app/src/main/AndroidManifest.xml"
	])).expect("Unable to read manifest file");

	let manifest=manifest.replace("SDLActivity","MainActivity");
	let manifest=manifest.replace("org.libsdl.app",&*appid);
	write(
		"target/android-project/app/src/main/AndroidManifest.xml",
		manifest.to_string()
	).expect("Unable to write file");

	let build_gradle=read_to_string(path_concat(vec![
		&*get_env_var("SDL"),
		"android-project/app/build.gradle"
	])).expect("Unable to read gradle build file");

	let build_gradle=build_gradle.replace("org.libsdl.app",&*appid);

	write(
		"target/android-project/app/build.gradle",
		build_gradle.to_string()
	).expect("Unable to write file");

	remove_items(&vec![Path::new(&*path_concat(
		vec!["target","android-project","app","jni","src"]
	))]).unwrap();

	if !Path::new("target/android-project/app/jni/SDL").is_dir() {
		symlink_dir(
			Path::new(&*get_env_var("SDL")),
			Path::new("target/android-project/app/jni/SDL"),
		).unwrap();
	}

	for (android_name,rust_name) in get_android_targets() {
		let android_dir=path_concat(vec![
			"target/android-project/app/src/main/jniLibs",
			android_name
		]);
		create_dir_all(android_dir).unwrap();
		copy(
			Path::new(&*path_concat(vec![
				"target",rust_name,"debug/libmain.so"
			])),
			Path::new(&*path_concat(vec![
				"target/android-project/app/src/main/jniLibs",
				android_name,"libmain.so"
			]))
		).unwrap();
	}
}

fn build_android_project() {
	assert!(Command::new("./gradlew")
		.args(["assembleDebug"])
		.current_dir("./target/android-project")
		.status()
		.unwrap()
		.success());
}

fn build_android() {
	for k in vec!["ANDROID_HOME","ANDROID_NDK_HOME","SDL"] {
		let _check_val=get_env_var(k);
	}

	build_sdl_for_android();
	create_android_cargo_config();
	copy_android_sdl_deps();
	build_android_targets();
	create_android_project();
	build_android_project();
}

fn run_android() {
	build_android();

	let appid=get_android_app_id();

	let p=Path::new(&*get_env_var("ANDROID_HOME")).join("platform-tools/adb");
	assert!(Command::new(p.clone())
		.args(["-d","install","target/android-project/app/build/outputs/apk/debug/app-debug.apk"])
		.status()
		.unwrap()
		.success());

	assert!(Command::new(p.clone())
		.args(["shell","am","force-stop",&*appid])
		.status()
		.unwrap()
		.success());

	let mut activity=appid.clone();
	activity.push_str("/.MainActivity");

	assert!(Command::new(p.clone())
		.args(["shell","am","start","-n",&*activity])
		.status()
		.unwrap()
		.success());

	/* wake up:
	adb shell input keyevent KEYCODE_WAKEUP
	[micke@micke-x455ya appaslib]$ adb shell input touchscreen swipe 930 880 930 380
	*/
}

/**
 * cp -r target/appy/SDL/android-project target/appy/android-project
 * rm -r target/appy/android-project/app/jni/src
 * ln -s target/appy/SDL target/appy/android-project/app/jni/SDL
 */

pub fn main() {
	let args:Vec<String>=env::args().collect();

	if args.len()!=2 {
		println!("Usage: appy <cmd>");
		println!("Commands:");
		println!("");
		println!("  build-android");
		println!("  run-android");
		println!("");

		panic!("Bad args...");
	}

	else if args[1]=="build-android" {
		build_android();
	}

	else if args[1]=="run-android" {
		run_android();
	}

	else {
		panic!("Bad args...");
	}
}