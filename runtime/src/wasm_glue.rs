use alloc::format;
use core::panic::PanicInfo;

#[no_mangle]
unsafe extern "C" fn diplomat_init() {
    #[cfg(debug_assertions)]
    std::panic::set_hook(Box::new(panic_handler));
    #[cfg(feature = "log")]
    log::set_logger(&ConsoleLogger)
        .map(|()| log::set_max_level(log::LevelFilter::Debug))
        .is_ok()
}


fn panic_handler(info: &PanicInfo) {
    let msg = match info.payload().downcast_ref::<&'static str>() {
        Some(&s) => s,
        None => match info.payload().downcast_ref::<String>() {
            Some(s) => s.as_str(),
            None => "Box<Any>",
        },
    };

    let msg = match info.location() {
        Some(l) => format!(
            "wasm panicked at {}:{}:{}:\n{msg}",
            l.file(),
            l.line(),
            l.column(),
        ),
        None => format!("wasm panicked at <unknown location>:\n{msg}"),
    };

    extern "C" {
        fn throw_error_js(ptr: *const u8, len: usize);
    }

    unsafe { throw_error_js(msg.as_ptr(), msg.len()) }
}

#[cfg(feature = "log")]
struct ConsoleLogger;

#[cfg(feature = "log")]
impl log::Log for ConsoleLogger {
    #[inline]
    fn enabled(&self, metadata: &log::Metadata) -> bool {
        metadata.level() <= log::max_level()
    }

    fn log(&self, record: &log::Record) {
        if !self.enabled(record.metadata()) {
            return;
        }

        let out = match record.level() {
            log::Level::Error => {
                extern "C" {
                    fn console_error_js(ptr: *const u8, len: usize);
                }
                console_error_js
            }
            log::Level::Warn => {
                extern "C" {
                    fn console_warn_js(ptr: *const u8, len: usize);
                }
                console_warn_js
            }
            log::Level::Info => {
                extern "C" {
                    fn console_info_js(ptr: *const u8, len: usize);
                }
                console_info_js
            }
            log::Level::Debug => {
                extern "C" {
                    fn console_log_js(ptr: *const u8, len: usize);
                }
                console_log_js
            }
            log::Level::Trace => {
                extern "C" {
                    fn console_debug_js(ptr: *const u8, len: usize);
                }
                console_debug_js
            }
        };

        let msg = alloc::format!("{}", record.args());

        unsafe { out(msg.as_ptr(), msg.len()) };
    }

    fn flush(&self) {}
}
