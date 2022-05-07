use std::ffi::CStr;

use tcl_tk_sys::*;

pub struct TclInterp(*mut Tcl_Interp);

impl TclInterp {
    pub fn new() -> Result<Self, String> {
        let interp = unsafe { Tcl_CreateInterp() };

        unsafe {
            if Tcl_Init(interp) == TCL_ERROR as _ {
                return Err(CStr::from_ptr(Tcl_GetStringResult(interp))
                    .to_str()
                    .unwrap()
                    .to_string());
            }
            if Tk_Init(interp) == TCL_ERROR as _ {
                return Err(CStr::from_ptr(Tcl_GetStringResult(interp))
                    .to_str()
                    .unwrap()
                    .to_string());
            }
        }

        Ok(TclInterp(interp))
    }

    pub fn eval(&self, script: &str) -> Result<(), String> {
        unsafe {
            if Tcl_EvalEx(self.0, script.as_ptr() as _, script.len() as _, 0) == TCL_ERROR as _ {
                return Err(CStr::from_ptr(Tcl_GetStringResult(self.0))
                    .to_str()
                    .unwrap()
                    .to_string());
            }
        }

        Ok(())
    }

    pub fn do_one_event(&self) -> i32 {
        unsafe { Tcl_DoOneEvent(0) }
    }
}

impl Drop for TclInterp {
    fn drop(&mut self) {
        unsafe { Tcl_DeleteInterp(self.0) };
    }
}

