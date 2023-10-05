use std::{env, fs, path::PathBuf};

mod api;
use api::Api;

const RAYLIB_API_PATH: &str = "raylib/parser/output/raylib_api.json";

fn is_web_build() -> bool {
    let target = std::env::var("TARGET").expect("TARGET is always set in the build script");

    target.contains("wasm32")
}

fn build_raylib() {
    let mut builder = cmake::Config::new("raylib");

    builder
        .define("BUILD_EXAMPLES", "OFF")
        .define("CMAKE_BUILD_TYPE", "Release")
        .generator("Ninja")
        .profile(if cfg!(debug_assertions) {
            "Debug"
        } else {
            "Release"
        });

    if is_web_build() {
        builder
            .define("PLATFORM", "Web")
            // NOTE: This doesn't actually do anything, but prevents emcc from breaking somehow?
            .define("CMAKE_C_FLAGS", "");
    }

    let dest = builder.build();

    println!(
        "cargo:rustc-link-search=native={}",
        dest.join("lib").display()
    );
    println!(
        "cargo:rustc-link-search=native={}",
        dest.join("lib64").display()
    );
    println!(
        "cargo:rustc-link-search=native={}",
        dest.join("lib32").display()
    );

    if is_web_build() {
        println!("cargo:rustc-link-lib=dylib=glfw");
        println!("cargo:rustc-link-lib=dylib=raylib");

        // INFO: early return because other platform checks will
        // detect the OS that is being used to build the
        // wasm code, and link the wrong libraries.
        return;
    }

    if cfg!(windows) {
        println!("cargo:rustc-link-lib=dylib=winmm");
        println!("cargo:rustc-link-lib=dylib=gdi32");
        println!("cargo:rustc-link-lib=dylib=user32");
        println!("cargo:rustc-link-lib=dylib=shell32");
    } else if cfg!(target_os = "macos") {
        println!("cargo:rustc-link-search=native=/usr/local/lib");
        println!("cargo:rustc-link-lib=framework=OpenGL");
        println!("cargo:rustc-link-lib=framework=Cocoa");
        println!("cargo:rustc-link-lib=framework=IOKit");
        println!("cargo:rustc-link-lib=framework=CoreFoundation");
        println!("cargo:rustc-link-lib=framework=CoreVideo");
    } else if cfg!(unix) {
        println!("cargo:rustc-link-search=/usr/local/lib");
        println!("cargo:rustc-link-lib=X11");
    }

    println!("cargo:rustc-link-lib=static=raylib");
}

fn main() {
    println!("cargo:rerun-if-changed={}", RAYLIB_API_PATH);
    
    build_raylib();

    let api_text = fs::read_to_string(RAYLIB_API_PATH).expect("Unable to read raylib api file");
    let api: Api = serde_json::from_str(&api_text).unwrap();

    let code = api.generate_code();

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    fs::write(out_path.join("raylib_ffi.rs"), code).expect("Unable to write bindings");
}
