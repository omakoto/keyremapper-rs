extern crate bindgen;

use std::path::PathBuf;
use std::{env, process::Command};

use regex::Regex;

fn compile_resources(src: &str) {
    let dest: String = {
        let mut dest = Regex::new(r#"\.gresource$"#).unwrap().replace(src, "").to_string();
        dest.push_str(".bin");
        dest
    };

    let mut arg_dest = "--target=".to_string();
    arg_dest.push_str(&dest);

    let status = Command::new("glib-compile-resources")
        .arg(src)
        .arg(arg_dest)
        .spawn()
        .expect("Failed running glib-compile-resources")
        .wait()
        .unwrap();
    assert!(status.success());
}

fn main() {
    println!("cargo:rerun-if-changed=src/wrapper.h");

    println!("cargo:rustc-link-lib=evdev");
    println!("cargo:rustc-link-lib=udev");

    // Build the native library bindings.
    let bindings = bindgen::Builder::default()
        .header("src/wrapper.h")
        .clang_arg("-I/usr/include/libevdev-1.0")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings.write_to_file(out_path.join("native-bindings.rs")).expect("Couldn't write bindings!");

    // Build the icon binaries.
    println!("cargo:rerun-if-changed=src/res/icons.gresource");
    compile_resources("src/res/icons.gresource");
    compile_resources("examples/keyboard-remapper/icons.gresource");
    compile_resources("examples/shortcut-remote-remapper/icons.gresource");
    compile_resources("examples/trackpoint-speedup/icons.gresource");
    compile_resources("examples/satechi-remapper/icons.gresource");

    // Generate the key codes. Don't run it automatically though; different kernels have different headers.
    // println!("cargo:rerun-if-changed=scripts/gen_binding.py");
    // let status = Command::new("scripts/gen_binding.py")
    //     .spawn()
    //     .expect("Failed running gen_binding.py")
    //     .wait()
    //     .unwrap();
    // assert!(status.success());

}
