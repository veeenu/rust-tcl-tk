use tcl_tk::*;

fn main() {
    let interp = TclInterp::new().unwrap();
    println!("Interp created");

    interp.eval("puts \"hello world\"").unwrap();
}
