// #[diplomat::bridge]
// mod ffi {

fn test_rust_fn(f: impl Fn(i32) -> i32) {
    f(10);
}

// }

// ----------------------------------------------------------------------------------------------------

use jni::objects::{GlobalRef, JObject};
use jni::{JavaVM, JNIEnv};
use jni::sys::jlong;
use core::ffi::c_void;

// enum for the foreign language runtimes
// depending on the language, we'll need this to spawn/attach threads to run the callback in
pub enum ForeignRuntime {
    JVM(JavaVM),
    NoRuntime,
}

// struct representing a callback from Rust into a foreign language
#[repr(C)]
pub struct DiplomatCallback {
    // any data required to run the callback; e.g. a pointer to the callback wrapper object in the foreign runtime
    data: *const c_void,
    // function to actually run the callback
    run_callback: extern "C" fn(*const c_void, *const c_void, *mut c_void), // TODO RETURNS probably make the DiplomatCallback generic over return
    // function to destroy this callback struct
    destructor: extern "C" fn(*const c_void) -> bool,
    foreign_runtime: *mut c_void,
}

#[no_mangle]
pub unsafe extern "C" fn diplomat_callback_destroy(this: *mut DiplomatCallback) {
    let this = Box::from_raw(this);
    // this.call the destructor TODO
    drop(this);
}

// shared functionality amongst the wrappers for callback args
trait FFIArgsWrapper {
    fn num_args(&self) -> usize;
    // do we want "size" or something, to help figure out the layout
}

// --------------------------------------------------------------------- this next part is be generic to the JVM 

// this one is specific to the JVM, so we have some JVM-specific inputs
#[no_mangle]
pub unsafe extern "system" fn diplomat_callback_create_for_jvm<'local>(
    env: JNIEnv<'local>,
    callback_wrapper: JObject,
) -> *mut DiplomatCallback {
    // define the callback runner
    extern "C" fn run_callback(data: *const c_void, args: *const c_void, runtime: *mut c_void) {
        unsafe {
            let runtime = runtime.cast::<ForeignRuntime>();
            let data = data.cast::<GlobalRef>();

            // attach_current_thread_permanently means that it attaches to the
            // current JVM thread, and that if the JVM dies then this call also dies.
            let jvm = match *runtime {
                ForeignRuntime::JVM(ref jvm) => jvm,
                _ => panic!("JVM callbacks need a JVM to run in"),
            };
            let mut env = jvm.attach_current_thread_permanently().unwrap();
            // set the args pointer
            env.call_method((*data).as_obj(), "set_args_pointer", "(J)V", &[jni::objects::JValueGen::Long(args as jlong)])
                .unwrap();
            // note: "run" is a method from Runnable, so we can't give it an arg
            env.call_method((*data).as_obj(), "run", "()V", &[])
                .unwrap();
        }
    }
    // define the destructor for the callback wrapper
    extern "C" fn destructor(this: *const c_void) -> bool {
        unsafe {
            // TODO free the memory
        }
        true
    }

    // initialize the callback wrapper
    let jvm = env.get_java_vm().unwrap();
    let ref_to_cb_wrapper = env.new_global_ref(callback_wrapper).unwrap();
    let ret = DiplomatCallback {
        data: Box::into_raw(Box::new(ref_to_cb_wrapper)) as _,
        run_callback,
        destructor,
        foreign_runtime: Box::into_raw(Box::new(ForeignRuntime::JVM(jvm))) as _,
    };

    Box::into_raw(Box::new(ret))
}

// ------------------------------------------------- this next section is specific to the callback, and goes in the trampoline crate

// wrapper struct for the args of this specific function
// note: this will need to be opaque so it can be passed by ref to the foreign language
pub struct FFIArgsWrapper_f {
    arg0: i32
}

impl FFIArgsWrapper_f {
    pub fn get_arg0(&self) -> i32 {
        self.arg0
    }    
}

impl FFIArgsWrapper for FFIArgsWrapper_f {
    fn num_args(&self) -> usize {
        1
    }
}

// function to get the first arg out of the arg struct for this specific callback
#[no_mangle]
pub unsafe extern "C" fn get_arg0_from_ffiargswrapper_f_pointer(fw: *const FFIArgsWrapper_f) -> i32 {
    unsafe {
        (*fw).get_arg0()
    }
}

// function that actually calls the callback
// it's basically a heavily instrumented version of the original Rust function that takes
// a callback as an argument.
// it wraps the arguments to the callback in the arg wrapper struct, and calls the callback
#[no_mangle]
pub unsafe extern "C" fn GEND_BRIDGE_test_run_fn(cb: &DiplomatCallback) {
    let args = FFIArgsWrapper_f{ arg0: 10 };
    let boxed_args = Box::into_raw(Box::new(args));
    (cb.run_callback)(cb.data, boxed_args as _, cb.foreign_runtime); // the "as _" to convert to void pointer
    (cb.destructor)(cb.data);
}