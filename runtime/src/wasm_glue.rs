use std::ffi::CString;
use std::os::raw::c_char;

#[cfg(debug_assertions)]
use std::panic;

// minimal WASM logger based on https://github.com/DeMille/wasm-glue
extern "C" {
    #[allow(dead_code)] // we want a consistent set of externs
    fn trace_js(ptr: *const c_char);
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
        let cstring = CString::new(err_info).unwrap();

        unsafe {
            trace_js(cstring.as_ptr());
        }
    }));
}

#[no_mangle]
pub unsafe extern "C" fn diplomat_init() {
    #[cfg(debug_assertions)]
    set_panic_hook();
}
