// minimal WASM logger based on https://github.com/DeMille/wasm-glue
extern "C" {
    fn trace_js(ptr: *const u8, len: usize);
    fn warn_js(ptr: *const u8, len: usize);
    fn log_js(ptr: *const u8, len: usize);
}

/// Throw an exception.
pub fn console_trace(msg: &str) {
    trace_js(str.as_ptr(), str.len());
}

/// Write a message to `console.warn`.
pub fn console_warn(msg: &str) {
    warn_js(str.as_ptr(), str.len())
}

/// Write a message to `console.log`.
pub fn console_log(msg: &str) {
    log_js(str.as_ptr(), str.len())
}

#[no_mangle]
pub unsafe extern "C" fn diplomat_init() {
    #[cfg(debug_assertions)]
    /// Sets a custom panic hook using `trace_js`, which by default crates a JS error
    std::panic::set_hook(Box::new(|info| {
        let file = info.location().unwrap().file();
        let line = info.location().unwrap().line();
        let col = info.location().unwrap().column();

        let msg = match info.payload().downcast_ref::<&'static str>() {
            Some(&s) => s,
            None => match info.payload().downcast_ref::<String>() {
                Some(s) => s.as_str(),
                None => "Box<Any>",
            },
        };
        console_trace(&format!("Panicked at '{msg}', {file}:{line}:{col}"));
    }));
}
