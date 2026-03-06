package dev.diplomattest.somelib;
import com.sun.jna.Callback
import com.sun.jna.Library
import com.sun.jna.Native
import com.sun.jna.Pointer
import com.sun.jna.Structure

internal interface ResultOpaqueLib: Library {
    fun ResultOpaque_destroy(handle: Pointer)
    fun ResultOpaque_new(i: Int): ResultPointerInt
    fun ResultOpaque_new_failing_foo(): ResultPointerInt
    fun ResultOpaque_new_failing_bar(): ResultPointerInt
    fun ResultOpaque_new_failing_unit(): ResultPointerUnit
    fun ResultOpaque_new_failing_struct(i: Int): ResultPointerErrorStructNative
    fun ResultOpaque_new_in_err(i: Int): ResultUnitPointer
    fun ResultOpaque_new_int(i: Int): ResultIntUnit
    fun ResultOpaque_new_failing_int(i: Int): ResultUnitInt
    fun ResultOpaque_new_in_enum_err(i: Int): ResultIntPointer
    fun ResultOpaque_give_self(handle: Pointer): ResultUnitPointer
    fun ResultOpaque_takes_str(handle: Pointer, v: Slice): Pointer
    fun ResultOpaque_assert_integer(handle: Pointer, i: Int): Unit
}

class ResultOpaque internal constructor (
    internal val handle: Pointer,
    // These ensure that anything that is borrowed is kept alive and not cleaned
    // up by the garbage collector.
    internal val selfEdges: List<Any>,
    internal var owned: Boolean,
): Exception("Rust error result for ResultOpaque")  {

    init {
        if (this.owned) {
            this.registerCleaner()
        }
    }

    private class ResultOpaqueCleaner(val handle: Pointer, val lib: ResultOpaqueLib) : Runnable {
        override fun run() {
            lib.ResultOpaque_destroy(handle)
        }
    }
    private fun registerCleaner() {
        CLEANER.register(this, ResultOpaque.ResultOpaqueCleaner(handle, ResultOpaque.lib));
    }

    companion object {
        internal val libClass: Class<ResultOpaqueLib> = ResultOpaqueLib::class.java
        internal val lib: ResultOpaqueLib = Native.load("diplomat_feature_tests", libClass)
        @JvmStatic
        
        fun new_(i: Int): Result<ResultOpaque> {
            
            val returnVal = lib.ResultOpaque_new(i);
            val nativeOkVal = returnVal.getNativeOk();
            if (nativeOkVal != null) {
                val selfEdges: List<Any> = listOf()
                val handle = nativeOkVal 
                val returnOpaque = ResultOpaque(handle, selfEdges, true)
                return returnOpaque.ok()
            } else {
                return ErrorEnumError(ErrorEnum.fromNative(returnVal.getNativeErr()!!)).err()
            }
        }
        @JvmStatic
        
        fun newFailingFoo(): Result<ResultOpaque> {
            
            val returnVal = lib.ResultOpaque_new_failing_foo();
            val nativeOkVal = returnVal.getNativeOk();
            if (nativeOkVal != null) {
                val selfEdges: List<Any> = listOf()
                val handle = nativeOkVal 
                val returnOpaque = ResultOpaque(handle, selfEdges, true)
                return returnOpaque.ok()
            } else {
                return ErrorEnumError(ErrorEnum.fromNative(returnVal.getNativeErr()!!)).err()
            }
        }
        @JvmStatic
        
        fun newFailingBar(): Result<ResultOpaque> {
            
            val returnVal = lib.ResultOpaque_new_failing_bar();
            val nativeOkVal = returnVal.getNativeOk();
            if (nativeOkVal != null) {
                val selfEdges: List<Any> = listOf()
                val handle = nativeOkVal 
                val returnOpaque = ResultOpaque(handle, selfEdges, true)
                return returnOpaque.ok()
            } else {
                return ErrorEnumError(ErrorEnum.fromNative(returnVal.getNativeErr()!!)).err()
            }
        }
        @JvmStatic
        
        fun newFailingUnit(): Result<ResultOpaque> {
            
            val returnVal = lib.ResultOpaque_new_failing_unit();
            val nativeOkVal = returnVal.getNativeOk();
            if (nativeOkVal != null) {
                val selfEdges: List<Any> = listOf()
                val handle = nativeOkVal 
                val returnOpaque = ResultOpaque(handle, selfEdges, true)
                return returnOpaque.ok()
            } else {
                return UnitError().err()
            }
        }
        @JvmStatic
        
        fun newFailingStruct(i: Int): Result<ResultOpaque> {
            
            val returnVal = lib.ResultOpaque_new_failing_struct(i);
            val nativeOkVal = returnVal.getNativeOk();
            if (nativeOkVal != null) {
                val selfEdges: List<Any> = listOf()
                val handle = nativeOkVal 
                val returnOpaque = ResultOpaque(handle, selfEdges, true)
                return returnOpaque.ok()
            } else {
                val returnStruct = ErrorStruct.fromNative(returnVal.getNativeErr()!!)
                return returnStruct.err()
            }
        }
        @JvmStatic
        
        fun newInErr(i: Int): Result<Unit> {
            
            val returnVal = lib.ResultOpaque_new_in_err(i);
            val nativeOkVal = returnVal.getNativeOk();
            if (nativeOkVal != null) {
                return Unit.ok()
            } else {
                val selfEdges: List<Any> = listOf()
                val handle = returnVal.getNativeErr()!! 
                val returnOpaque = ResultOpaque(handle, selfEdges, true)
                return returnOpaque.err()
            }
        }
        @JvmStatic
        
        fun newInt(i: Int): Result<Int> {
            
            val returnVal = lib.ResultOpaque_new_int(i);
            val nativeOkVal = returnVal.getNativeOk();
            if (nativeOkVal != null) {
                return (nativeOkVal).ok()
            } else {
                return UnitError().err()
            }
        }
        @JvmStatic
        
        fun newFailingInt(i: Int): Result<Unit> {
            
            val returnVal = lib.ResultOpaque_new_failing_int(i);
            val nativeOkVal = returnVal.getNativeOk();
            if (nativeOkVal != null) {
                return Unit.ok()
            } else {
                return IntError(returnVal.getNativeErr()!!).err()
            }
        }
        @JvmStatic
        
        fun newInEnumErr(i: Int): Result<ErrorEnum> {
            
            val returnVal = lib.ResultOpaque_new_in_enum_err(i);
            val nativeOkVal = returnVal.getNativeOk();
            if (nativeOkVal != null) {
                return (ErrorEnum.fromNative(nativeOkVal)).ok()
            } else {
                val selfEdges: List<Any> = listOf()
                val handle = returnVal.getNativeErr()!! 
                val returnOpaque = ResultOpaque(handle, selfEdges, true)
                return returnOpaque.err()
            }
        }
    }
    
    fun giveSelf(): Result<Unit> {
        // This lifetime edge depends on lifetimes: 'a
        val aEdges: MutableList<Any> = mutableListOf(this);
        
        val returnVal = lib.ResultOpaque_give_self(handle);
        val nativeOkVal = returnVal.getNativeOk();
        if (nativeOkVal != null) {
            return Unit.ok()
        } else {
            val selfEdges: List<Any> = listOf(this)
            val handle = returnVal.getNativeErr()!! 
            val returnOpaque = ResultOpaque(handle, selfEdges, false)
            return returnOpaque.err()
        }
    }
    
    /** When we take &str, the return type becomes a Result
    *Test that this interacts gracefully with returning a reference type
    */
    fun takesStr(v: String): ResultOpaque {
        // This lifetime edge depends on lifetimes: 'a
        val aEdges: MutableList<Any> = mutableListOf(this);
        val vSliceMemory = PrimitiveArrayTools.borrowUtf8(v)
        
        val returnVal = lib.ResultOpaque_takes_str(handle, vSliceMemory.slice);
        try {
            val selfEdges: List<Any> = listOf(this)
            val handle = returnVal 
            val returnOpaque = ResultOpaque(handle, selfEdges, false)
            return returnOpaque
        } finally {
            vSliceMemory.close()
        }
    }
    
    fun assertInteger(i: Int): Unit {
        
        val returnVal = lib.ResultOpaque_assert_integer(handle, i);
        
    }

}