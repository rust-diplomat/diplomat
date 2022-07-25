use std::ffi::CString;
use std::io;
use std::os::raw::c_char;

#[cfg(debug_assertions)]
use std::panic;

// minimal WASM logger based on https://github.com/DeMille/wasm-glue
extern "C" {
    #[allow(dead_code)] // we want a consistent set of externs
    fn trace_js(ptr: *const c_char);
    #[allow(dead_code)]
    fn warn_js(ptr: *const c_char);
    #[allow(dead_code)]
    fn log_js(ptr: *const c_char);
}

/// Throw an exception.
#[allow(dead_code)]
pub fn console_trace(msg: &str) -> io::Result<()> {
    console(msg, trace_js)
}

/// Write a message to `console.warn`.
#[allow(dead_code)]
pub fn console_warn(msg: &str) -> io::Result<()> {
    console(msg, warn_js)
}

/// Write a message to `console.log`.
#[allow(dead_code)]
pub fn console_log(msg: &str) -> io::Result<()> {
    console(msg, log_js)
}

/// Call a JavaScript function that takes a string.
fn console(msg: &str, f: unsafe extern "C" fn(*const c_char)) -> io::Result<()> {
    let cstring = CString::new(msg)?;
    unsafe {
        f(cstring.as_ptr());
    }
    Ok(())
}

/// Sets a custom panic hook, uses your JavaScript `trace` function
#[cfg(debug_assertions)]
fn set_panic_hook() {
    panic::set_hook(Box::new(|info| {
        let file = info.location().unwrap().file();
        let line = info.location().unwrap().line();
        let col = info.location().unwrap().column();

        let msg = match info.payload().downcast_ref::<&'static str>() {
            Some(s) => *s,
            None => match info.payload().downcast_ref::<String>() {
                Some(s) => &s[..],
                None => "Box<Any>",
            },
        };

        let err_info = format!("Panicked at '{}', {}:{}:{}", msg, file, line, col);
        console_trace(&err_info).unwrap();
    }));
}

#[no_mangle]
pub unsafe extern "C" fn diplomat_init() {
    #[cfg(debug_assertions)]
    set_panic_hook();
}
