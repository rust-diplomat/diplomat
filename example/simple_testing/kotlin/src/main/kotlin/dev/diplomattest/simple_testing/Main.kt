package dev.diplomattest.simple_testing;

import com.sun.jna.Callback
import com.sun.jna.Library
import com.sun.jna.Native
import com.sun.jna.Pointer
import com.sun.jna.Structure


interface DiplomatTraitInterface_TesterTrait {
    fun test_trait_fn(x: Int): Int;
    fun test_void_trait_fn(): Unit;
}

class DiplomatTrait_TesterTrait_VTable_Native: Structure(), Structure.ByValue {
    @JvmField
    internal var run_test_trait_fn_callback: DiplomatCallback_Lib.Runner_DiplomatTraitMethod_TesterTrait_test_trait_fn
        = object :  DiplomatCallback_Lib.Runner_DiplomatTraitMethod_TesterTrait_test_trait_fn {
                override fun invoke(ignored: Pointer?, input: Int): Int {
                    throw Exception("ERROR NOT IMPLEMENTED")
                }
            }
    @JvmField
    internal var run_test_void_trait_fn_callback: DiplomatCallback_Lib.Runner_DiplomatTraitMethod_TesterTrait_test_void_trait_fn
        = object :  DiplomatCallback_Lib.Runner_DiplomatTraitMethod_TesterTrait_test_void_trait_fn {
                override fun invoke(ignored: Pointer?): Unit {
                    throw Exception("ERROR NOT IMPLEMENTED")
                }
            }
    @JvmField
    internal var destructor: Pointer = Pointer(0L);
  
    // Define the fields of the struct
    override fun getFieldOrder(): List<String> {
        return listOf("run_test_trait_fn_callback", "run_test_void_trait_fn_callback", "destructor")
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

class DiplomatCallback_TesterTrait_Wrapper internal constructor (
    internal val nativeStruct: DiplomatTrait_TesterTrait_Wrapper_Native) {
    val data_: Pointer = nativeStruct.data_
    val vtable: DiplomatTrait_TesterTrait_VTable_Native = nativeStruct.vtable

    companion object {
        val NATIVESIZE: Long = Native.getNativeSize(DiplomatTrait_TesterTrait_Wrapper_Native::class.java).toLong()
        internal val libClass: Class<DiplomatCallback_Lib> = DiplomatCallback_Lib::class.java
        internal val lib: DiplomatCallback_Lib = Native.load("somelib", libClass)

        fun fromTraitObj(trt_obj: DiplomatTraitInterface_TesterTrait): DiplomatCallback_TesterTrait_Wrapper {
            val test_trait_fn: DiplomatCallback_Lib.Runner_DiplomatTraitMethod_TesterTrait_test_trait_fn = object :  DiplomatCallback_Lib.Runner_DiplomatTraitMethod_TesterTrait_test_trait_fn {
                override fun invoke(ignored: Pointer?, input: Int): Int {
                    return trt_obj.test_trait_fn(input);
                }
            }
            val test_void_trait_fn: DiplomatCallback_Lib.Runner_DiplomatTraitMethod_TesterTrait_test_void_trait_fn = object :  DiplomatCallback_Lib.Runner_DiplomatTraitMethod_TesterTrait_test_void_trait_fn {
                override fun invoke(ignored: Pointer?): Unit {
                    trt_obj.test_void_trait_fn();
                }
            }
            val vtable = DiplomatTrait_TesterTrait_VTable_Native()
            vtable.run_test_trait_fn_callback = test_trait_fn;
            vtable.run_test_void_trait_fn_callback = test_void_trait_fn;
            val native_wrapper = DiplomatTrait_TesterTrait_Wrapper_Native();
            native_wrapper.vtable = vtable;
            return DiplomatCallback_TesterTrait_Wrapper(native_wrapper);
        }

        fun test_trait_fn(trait_wrap: DiplomatCallback_TesterTrait_Wrapper, i: Int): Int {
            return lib.Wrapper_test_with_trait(trait_wrap.nativeStruct, i);
        }
    }

}

interface DiplomatCallback_Lib: Library {
    interface Runner_DiplomatTraitMethod_TesterTrait_test_trait_fn: Callback {
        fun invoke(ignored: Pointer?, input: Int): Int
    }

    interface Runner_DiplomatTraitMethod_TesterTrait_test_void_trait_fn: Callback {
        fun invoke(ignored: Pointer?): Unit
    }

    // specific to this callback
    fun Wrapper_test_with_trait(diplomatTraitWrapper: DiplomatTrait_TesterTrait_Wrapper_Native, i: Int): Int
}

class MyTesterTraitImpl: DiplomatTraitInterface_TesterTrait {
    override fun test_trait_fn(x: Int): Int {
        return x*2;
    }

    override fun test_void_trait_fn() {
        println("Calling the void trait function in Kotlin");
    }
} 

object Main {
    
    @JvmStatic
    fun main(args: Array<String>) {
        var trait_obj = MyTesterTraitImpl();
        var trait_obj_wrap = DiplomatCallback_TesterTrait_Wrapper.fromTraitObj(trait_obj);
        var res = DiplomatCallback_TesterTrait_Wrapper.test_trait_fn(trait_obj_wrap, 5); // calls test_rust_fn with the callback
        println("Result: " + res);
    }
}