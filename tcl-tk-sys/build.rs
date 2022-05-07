use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use anyhow::*;
use bindgen::{Builder, CargoCallbacks};

fn main() {
    let build_dir = project_root().join("target").join("tcl-tk");

    if cfg!(target_os = "macos") {
        if !build_dir.join("lib").join("libtcl8.6.a").exists() {
            compile_tcl_tk_macos().unwrap();
        }
    }
    if cfg!(target_os = "windows") {
        println!("cargo:rustc-link-lib=user32");
        println!("cargo:rustc-link-lib=kernel32");
        println!("cargo:rustc-link-lib=advapi32");
        println!("cargo:rustc-link-lib=netapi32");
        println!("cargo:rustc-link-lib=gdi32");
        println!("cargo:rustc-link-lib=winmm");
        println!("cargo:rustc-link-lib=spoolss");
        println!("cargo:rustc-link-lib=static=tcl86sx");
        println!("cargo:rustc-link-lib=static=tk86sx");
    }

    println!(
        "cargo:rustc-link-search={}",
        build_dir.join("lib").to_string_lossy()
    );
    println!("cargo:rustc-link-lib=tcl8.6");
    println!("cargo:rustc-link-lib=tk8.6");

    let bindings = Builder::default()
        .header(build_dir.join("include").join("tcl.h").to_string_lossy())
        .header(build_dir.join("include").join("tk.h").to_string_lossy())
        .parse_callbacks(Box::new(CargoCallbacks))
        .clang_arg(env!("CFLAGS"))
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}

fn git_clone_tcl_tk() -> Result<()> {
    let build_dir = project_root().join("target").join("tcl-tk");

    fs::create_dir_all(&build_dir)?;

    Command::new("git")
        .arg("clone")
        .arg("--branch")
        .arg("release")
        .arg("https://github.com/tcltk/tcl")
        .current_dir(&build_dir)
        .status()?;

    Command::new("git")
        .arg("clone")
        .arg("--branch")
        .arg("release")
        .arg("https://github.com/tcltk/tk")
        .current_dir(&build_dir)
        .status()?;

    Ok(())
}

fn compile_tcl_tk_macos() -> Result<()> {
    let build_dir = project_root().join("target").join("tcl-tk");
    let tcl_dir = build_dir.join("tcl");
    let tk_dir = build_dir.join("tk");

    git_clone_tcl_tk()?;

    Command::new("bash")
        .arg("./configure")
        .arg("--disable-shared")
        .arg("--disable-load")
        .arg("--enable-corefoundation")
        .arg(&format!("--prefix={}", build_dir.to_string_lossy()))
        .env("CC", "clang")
        .current_dir(tcl_dir.join("unix"))
        .status()?;

    Command::new("bash")
        .arg("./configure")
        .arg("--disable-shared")
        .arg("--disable-load")
        .arg("--enable-corefoundation")
        .arg("--enable-aqua")
        .arg(&format!(
            "--with-tcl={}",
            tcl_dir.join("unix").to_string_lossy()
        ))
        .arg(&format!("--prefix={}", build_dir.to_string_lossy()))
        .env("CC", "clang")
        .current_dir(tk_dir.join("unix"))
        .status()?;

    Command::new("make")
        .env("CC", "clang")
        .current_dir(tcl_dir.join("unix"))
        .status()?;

    Command::new("make")
        .env("CC", "clang")
        .current_dir(tk_dir.join("unix"))
        .status()?;

    Command::new("make")
        .arg("install")
        .env("CC", "clang")
        .current_dir(tcl_dir.join("unix"))
        .status()?;

    Command::new("make")
        .arg("install")
        .env("CC", "clang")
        .current_dir(tk_dir.join("unix"))
        .status()?;

    Ok(())
}

fn project_root() -> PathBuf {
    Path::new(&env!("CARGO_MANIFEST_DIR"))
        .ancestors()
        .nth(1)
        .unwrap()
        .to_path_buf()
}
