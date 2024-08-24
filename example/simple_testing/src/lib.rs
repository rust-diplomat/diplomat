use diplomat_runtime::DiplomatCallback;
use core::ffi::c_void;

// #[diplomat::bridge]
// mod ffi {

pub trait TesterTrait {
    fn test_trait_fn(&self, x: i32) -> i32;
    fn test_void_trait_fn(&self);
}

pub struct Wrapper {
    cant_be_empty: bool,
}


impl Wrapper {
    pub fn test_with_trait(t: impl TesterTrait, x: i32) -> i32 {
        t.test_void_trait_fn();
        t.test_trait_fn(x)
    }
}  

// }


// ------------------------------------------------------------ generated code below

#[repr(C)]
pub struct TesterTrait_VTable {
    pub run_test_trait_fn_callback: unsafe extern "C" fn(*const c_void, i32) -> i32,
    pub run_test_void_trait_fn_callback: unsafe extern "C" fn(*const c_void),
    pub destructor: Option<unsafe extern "C" fn(*const c_void)>,
}

#[repr(C)]
pub struct DiplomatTraitStruct_TesterTrait {
    pub data: *const c_void,
    pub vtable: TesterTrait_VTable,
}

impl Drop for DiplomatTraitStruct_TesterTrait {
    fn drop(&mut self) {
        if let Some(destructor) = self.vtable.destructor {
            unsafe {
                (destructor)(self.data);
            }
        }
    }
}

impl TesterTrait for DiplomatTraitStruct_TesterTrait {
    fn test_trait_fn(&self, x: i32) -> i32 {
        unsafe {
            ((self.vtable).run_test_trait_fn_callback)(self.data, x)
        }
    }

    fn test_void_trait_fn(&self) {
        unsafe {
            ((self.vtable).run_test_void_trait_fn_callback)(self.data);
        }
    }
}

#[no_mangle]
extern "system" fn Wrapper_test_with_trait(
    t_trait_wrap: DiplomatTraitStruct_TesterTrait,
    x: i32,
) -> i32 {
    Wrapper::test_with_trait(
        t_trait_wrap,
        x,
    )
}