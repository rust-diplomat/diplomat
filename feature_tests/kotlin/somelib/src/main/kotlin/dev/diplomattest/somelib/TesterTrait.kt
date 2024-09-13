package dev.diplomattest.somelib

import com.sun.jna.Callback
import com.sun.jna.Library
import com.sun.jna.Native
import com.sun.jna.Pointer
import com.sun.jna.Structure

interface TesterTrait {
    fun test_trait_fn(x: Int): Int;
    fun test_void_trait_fn(): Unit;
    fun test_struct_trait_fn(s: TraitTestingStruct): Int;
}


internal interface Runner_DiplomatTraitMethod_TesterTrait_test_trait_fn: Callback {
    fun invoke(ignored: Pointer?, x: Int ): Int
}
internal interface Runner_DiplomatTraitMethod_TesterTrait_test_void_trait_fn: Callback {
    fun invoke(ignored: Pointer?): Unit
}
internal interface Runner_DiplomatTraitMethod_TesterTrait_test_struct_trait_fn: Callback {
    fun invoke(ignored: Pointer?, s: TraitTestingStructNative ): Int
}

object TesterTrait_VTable_destructor: Callback {
    fun invoke(obj_pointer: Pointer) {
        DiplomatJVMRuntime.dropRustCookie(obj_pointer);
    }
};

class DiplomatTrait_TesterTrait_VTable_Native: Structure(), Structure.ByValue {
    @JvmField
    internal var destructor: Callback = TesterTrait_VTable_destructor;
    @JvmField
    internal var size: Pointer = Pointer(0L);
    @JvmField
    internal var alignment: Pointer = Pointer(0L);
    
    @JvmField
    internal var run_test_trait_fn_callback: Runner_DiplomatTraitMethod_TesterTrait_test_trait_fn
        = object :  Runner_DiplomatTraitMethod_TesterTrait_test_trait_fn {
                override fun invoke(ignored: Pointer?, x: Int ): Int {
                    throw Exception("ERROR NOT IMPLEMENTED")
                }
            }
    @JvmField
    internal var run_test_void_trait_fn_callback: Runner_DiplomatTraitMethod_TesterTrait_test_void_trait_fn
        = object :  Runner_DiplomatTraitMethod_TesterTrait_test_void_trait_fn {
                override fun invoke(ignored: Pointer?): Unit {
                    throw Exception("ERROR NOT IMPLEMENTED")
                }
            }
    @JvmField
    internal var run_test_struct_trait_fn_callback: Runner_DiplomatTraitMethod_TesterTrait_test_struct_trait_fn
        = object :  Runner_DiplomatTraitMethod_TesterTrait_test_struct_trait_fn {
                override fun invoke(ignored: Pointer?, s: TraitTestingStructNative ): Int {
                    throw Exception("ERROR NOT IMPLEMENTED")
                }
            }
    // Define the fields of the struct
    override fun getFieldOrder(): List<String> {
        return listOf("destructor", "size", "alignment", "run_test_trait_fn_callback", "run_test_void_trait_fn_callback", "run_test_struct_trait_fn_callback")
    }
}

class DiplomatTrait_TesterTrait_Wrapper_Native: Structure(), Structure.ByValue {
    @JvmField
    internal var data_: Pointer = Pointer(0L);
    @JvmField
    internal var vtable: DiplomatTrait_TesterTrait_VTable_Native
        = DiplomatTrait_TesterTrait_VTable_Native();

    // Define the fields of the struct
    override fun getFieldOrder(): List<String> {
        return listOf("data_", "vtable")
    }
}

class DiplomatTrait_TesterTrait_Wrapper internal constructor (
    internal val nativeStruct: DiplomatTrait_TesterTrait_Wrapper_Native) {
    val data_: Pointer = nativeStruct.data_
    val vtable: DiplomatTrait_TesterTrait_VTable_Native = nativeStruct.vtable

    companion object {
        val NATIVESIZE: Long = Native.getNativeSize(DiplomatTrait_TesterTrait_Wrapper_Native::class.java).toLong()

        fun fromTraitObj(trt_obj: TesterTrait): DiplomatTrait_TesterTrait_Wrapper {
            val vtable = DiplomatTrait_TesterTrait_VTable_Native()
            
            
            val test_trait_fn: Runner_DiplomatTraitMethod_TesterTrait_test_trait_fn = object :  Runner_DiplomatTraitMethod_TesterTrait_test_trait_fn {
                override fun invoke(ignored: Pointer?, x: Int ): Int {
                    return trt_obj.test_trait_fn(x);
                }
            }
            vtable.run_test_trait_fn_callback = test_trait_fn;
            val test_void_trait_fn: Runner_DiplomatTraitMethod_TesterTrait_test_void_trait_fn = object :  Runner_DiplomatTraitMethod_TesterTrait_test_void_trait_fn {
                override fun invoke(ignored: Pointer?): Unit {
                    return trt_obj.test_void_trait_fn();
                }
            }
            vtable.run_test_void_trait_fn_callback = test_void_trait_fn;
            val test_struct_trait_fn: Runner_DiplomatTraitMethod_TesterTrait_test_struct_trait_fn = object :  Runner_DiplomatTraitMethod_TesterTrait_test_struct_trait_fn {
                override fun invoke(ignored: Pointer?, s: TraitTestingStructNative ): Int {
                    return trt_obj.test_struct_trait_fn(TraitTestingStruct(s));
                }
            }
            vtable.run_test_struct_trait_fn_callback = test_struct_trait_fn;
            val native_wrapper = DiplomatTrait_TesterTrait_Wrapper_Native();
            native_wrapper.vtable = vtable;
            native_wrapper.data_ = DiplomatJVMRuntime.buildRustCookie(vtable as Object);
            return DiplomatTrait_TesterTrait_Wrapper(native_wrapper);
        }
    }
}
