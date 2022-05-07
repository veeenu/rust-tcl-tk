use std::path::{Path, PathBuf};
use std::process::Command;

use anyhow::*;

fn main() {
    compile_tcl_tk().unwrap();
}

fn compile_tcl_tk() -> Result<()> {
    let build_dir = project_root().join("tcl-tk").join("build");

    Command::new("bash")
        .arg("./configure")
        .arg("--disable-shared")
        .arg("--disable-load")
        .arg("--enable-corefoundation")
        .arg(&format!("--prefix={}", build_dir.to_string_lossy()))
        .env("CC", "clang")
        .current_dir(project_root().join("tcl-tk").join("tcl").join("unix"))
        .status()?;

    Command::new("make")
        .arg("-C")
        .arg("tcl/macosx")
        .env("CC", "clang")
        .current_dir(project_root().join("tcl-tk"))
        .status()?;

    Command::new("bash")
        .arg("./configure")
        .arg("--disable-shared")
        .arg("--disable-load")
        .arg("--enable-corefoundation")
        .arg("--enable-aqua")
        .arg(&format!("--with-tcl={}", build_dir.to_string_lossy()))
        .arg(&format!("--prefix={}", build_dir.to_string_lossy()))
        .env("CC", "clang")
        .current_dir(project_root().join("tcl-tk").join("tk").join("unix"))
        .status()?;

    Command::new("make")
        .arg("-C")
        .arg("tk/macosx")
        .env("CC", "clang")
        .current_dir(project_root().join("tcl-tk"))
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
