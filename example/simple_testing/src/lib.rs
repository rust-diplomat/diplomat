// #[diplomat::bridge]
// mod ffi {

fn test_rust_fn(f: impl Fn(i32) -> i32) -> i32 {
    f(10)
}

// }

// ----------------------------------------------------------------------------------------------------

use jni::objects::{GlobalRef, JObject};
use jni::{JavaVM, JNIEnv};
use jni::sys::jlong;
use core::ffi::c_void;

// struct representing a callback from Rust into a foreign language
// TODO restrict the return type?
#[repr(C)]
pub struct DiplomatCallback<ReturnType> {
    // any data required to run the callback; e.g. a pointer to the callback wrapper object in the foreign runtime + the runtime itself
    data: *const c_void,
    // function to actually run the callback
    run_callback: extern "C" fn(*const c_void, ...) -> ReturnType, // TODO RETURNS probably make the DiplomatCallback generic over return
    // function to destroy this callback struct
    destructor: extern "C" fn(*const c_void) -> bool,
}

#[no_mangle]
pub unsafe extern "C" fn diplomat_callback_destroy<ReturnType>(this: *mut DiplomatCallback<ReturnType>) {
    let this = Box::from_raw(this);
    // this.call the destructor TODO
    drop(this);
}

// --------------------------------------------------------------------- this next part is be generic to the JVM 

struct JVMRunInfoWrapper {
    data: GlobalRef,
    runtime: JavaVM,
}

// this one is specific to the JVM, so we have some JVM-specific inputs
unsafe extern "C" fn general_callback_create_for_jvm<'local, ReturnType> (
    env: JNIEnv<'local>,
    callback_wrapper: JObject,
    run_callback: extern "C" fn(*const c_void, ...) -> ReturnType,
 ) -> *mut DiplomatCallback<ReturnType> {
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
    let run_info_wrapper = JVMRunInfoWrapper {
        data: ref_to_cb_wrapper,
        runtime: jvm,
    };
    let ret = DiplomatCallback::<ReturnType> {
        data: Box::into_raw(Box::new(run_info_wrapper)) as _,
        run_callback,
        destructor,
    };

    Box::into_raw(Box::new(ret))
}

// ------------------------------------------------- this next section is specific to the callback

#[no_mangle]
pub unsafe extern "system" fn diplomat_callback_create_for_jvm__callback<'local>(
    env: JNIEnv<'local>,
    callback_wrapper: JObject,
) -> *mut DiplomatCallback<i32> {
    // define the callback runner
    extern "C" fn run_callback(data: *const c_void, arg0: i32) -> i32 {
        unsafe {
            let data = data.cast::<JVMRunInfoWrapper>();

            // attach_current_thread_permanently means that it attaches to the
            // current JVM thread, and that if the JVM dies then this call also dies.
            let mut env = (*data).runtime.attach_current_thread_permanently().unwrap();
            // set the args pointer
            env.call_method((*data).data.as_obj(), "set_arg0", "(I)V", &[jni::objects::JValueGen::Int(arg0)])
                .unwrap();
            // note: "run" is a method from Runnable, so we can't give it an arg
            let res = env.call_method((*data).data.as_obj(), "run_callback", "()I", &[]).unwrap();
            match res {
                jni::objects::JValueGen::Int(int_res) => int_res,
                _ => panic!("callback should have returned an integer"),
            }
        }
    }
    general_callback_create_for_jvm(env, callback_wrapper, 
        std::mem::transmute::<extern "C" fn (*const c_void, i32) -> i32, extern "C" fn(*const c_void, ...) -> i32>(run_callback),
    )
}

// function that actually calls the callback
// it's basically a heavily instrumented version of the original Rust function that takes
// a callback as an argument.
// it wraps the arguments to the callback in the arg wrapper struct, and calls the callback
#[no_mangle]
pub unsafe extern "C" fn GEND_BRIDGE_test_run_fn(cb: &DiplomatCallback<i32>) -> i32 {
    let res = (cb.run_callback)(cb.data, 10);
    (cb.destructor)(cb.data);
    res
}