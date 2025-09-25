package dev.diplomattest.somelib

import com.sun.jna.Callback
import com.sun.jna.Library
import com.sun.jna.Native
import com.sun.jna.Pointer
import com.sun.jna.Structure

interface TesterTrait {
    fun testTraitFn(x: UInt): UInt;
    fun testVoidTraitFn(): Unit;
    fun testStructTraitFn(s: TraitTestingStruct): Int;
}


internal interface Runner_DiplomatTraitMethod_TesterTrait_testTraitFn: Callback {
    fun invoke(ignored: Pointer?, x: UInt ): FFIUint32
}
internal interface Runner_DiplomatTraitMethod_TesterTrait_testVoidTraitFn: Callback {
    fun invoke(ignored: Pointer?): Unit
}
internal interface Runner_DiplomatTraitMethod_TesterTrait_testStructTraitFn: Callback {
    fun invoke(ignored: Pointer?, s: TraitTestingStructNative ): Int
}

internal object TesterTrait_VTable_destructor: Callback {
    fun invoke(obj_pointer: Pointer) {
        DiplomatJVMRuntime.dropRustCookie(obj_pointer);
    }
};

internal class DiplomatTrait_TesterTrait_VTable_Native: Structure(), Structure.ByValue {
    @JvmField
    internal var destructor: Callback = TesterTrait_VTable_destructor;
    @JvmField
    internal var size: Pointer = Pointer(0L);
    @JvmField
    internal var alignment: Pointer = Pointer(0L);
    
    @JvmField
    internal var run_testTraitFn_callback: Runner_DiplomatTraitMethod_TesterTrait_testTraitFn
        = object :  Runner_DiplomatTraitMethod_TesterTrait_testTraitFn {
                override fun invoke(ignored: Pointer?, x: UInt ): FFIUint32 {
                    throw Exception("ERROR NOT IMPLEMENTED")
                }
            }
    @JvmField
    internal var run_testVoidTraitFn_callback: Runner_DiplomatTraitMethod_TesterTrait_testVoidTraitFn
        = object :  Runner_DiplomatTraitMethod_TesterTrait_testVoidTraitFn {
                override fun invoke(ignored: Pointer?): Unit {
                    throw Exception("ERROR NOT IMPLEMENTED")
                }
            }
    @JvmField
    internal var run_testStructTraitFn_callback: Runner_DiplomatTraitMethod_TesterTrait_testStructTraitFn
        = object :  Runner_DiplomatTraitMethod_TesterTrait_testStructTraitFn {
                override fun invoke(ignored: Pointer?, s: TraitTestingStructNative ): Int {
                    throw Exception("ERROR NOT IMPLEMENTED")
                }
            }
    // Define the fields of the struct
    override fun getFieldOrder(): List<String> {
        return listOf("destructor", "size", "alignment", "run_testTraitFn_callback", "run_testVoidTraitFn_callback", "run_testStructTraitFn_callback")
    }
}

internal class DiplomatTrait_TesterTrait_Wrapper_Native: Structure(), Structure.ByValue {
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

internal class DiplomatTrait_TesterTrait_Wrapper internal constructor (
    internal val nativeStruct: DiplomatTrait_TesterTrait_Wrapper_Native) {
    val data_: Pointer = nativeStruct.data_
    val vtable: DiplomatTrait_TesterTrait_VTable_Native = nativeStruct.vtable

    companion object {
        val NATIVESIZE: Long = Native.getNativeSize(DiplomatTrait_TesterTrait_Wrapper_Native::class.java).toLong()

        fun fromTraitObj(trt_obj: TesterTrait): DiplomatTrait_TesterTrait_Wrapper {
            val vtable = DiplomatTrait_TesterTrait_VTable_Native()

            
            val testTraitFn: Runner_DiplomatTraitMethod_TesterTrait_testTraitFn = object :  Runner_DiplomatTraitMethod_TesterTrait_testTraitFn {
                override fun invoke(ignored: Pointer?, x: UInt ): FFIUint32 {
                    return FFIUint32(trt_obj.testTraitFn(x));
                }
            }
            vtable.run_testTraitFn_callback = testTraitFn;
            val testVoidTraitFn: Runner_DiplomatTraitMethod_TesterTrait_testVoidTraitFn = object :  Runner_DiplomatTraitMethod_TesterTrait_testVoidTraitFn {
                override fun invoke(ignored: Pointer?): Unit {
                    return (trt_obj.testVoidTraitFn());
                }
            }
            vtable.run_testVoidTraitFn_callback = testVoidTraitFn;
            val testStructTraitFn: Runner_DiplomatTraitMethod_TesterTrait_testStructTraitFn = object :  Runner_DiplomatTraitMethod_TesterTrait_testStructTraitFn {
                override fun invoke(ignored: Pointer?, s: TraitTestingStructNative ): Int {
                    return (trt_obj.testStructTraitFn(TraitTestingStruct(s)));
                }
            }
            vtable.run_testStructTraitFn_callback = testStructTraitFn;
            val native_wrapper = DiplomatTrait_TesterTrait_Wrapper_Native();
            native_wrapper.vtable = vtable;
            val ret_val = DiplomatTrait_TesterTrait_Wrapper(native_wrapper);
            ret_val.nativeStruct.data_ = DiplomatJVMRuntime.buildRustCookie(ret_val as Object);
            return ret_val;
        }
    }
}
