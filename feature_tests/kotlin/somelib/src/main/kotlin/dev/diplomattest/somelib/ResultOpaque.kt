package dev.diplomattest.somelib;
import com.sun.jna.Library
import com.sun.jna.Native
import com.sun.jna.Pointer


internal interface ResultOpaqueLib: Library {
    fun ResultOpaque_destroy(handle: Pointer)
    fun ResultOpaque_new(i: Int): ResultPointerInt
    fun ResultOpaque_new_failing_foo(): ResultPointerInt
    fun ResultOpaque_new_failing_bar(): ResultPointerInt
    fun ResultOpaque_new_failing_unit(): ResultPointerUnit
    fun ResultOpaque_new_failing_struct(i: Int): ResultPointerErrorStructNative
    fun ResultOpaque_new_in_err(i: Int): ResultUnitPointer
    fun ResultOpaque_new_int(i: Int): ResultIntUnit
    fun ResultOpaque_new_in_enum_err(i: Int): ResultIntPointer
    fun ResultOpaque_assert_integer(handle: Pointer, i: Int): Unit
}

class ResultOpaque internal constructor (
    internal val handle: Pointer,

    // These ensure that anything that is borrowed is kept alive and not cleaned
    // up by the garbage collector.
    internal val selfEdges: List<Any>) {

    internal class ResultOpaqueCleaner(val handle: Pointer, val lib: ResultOpaqueLib) : Runnable {
        override fun run() {
            lib.ResultOpaque_destroy(handle)
        }
    }

    companion object {
        internal val libClass: Class<ResultOpaqueLib> = ResultOpaqueLib::class.java
        internal val lib: ResultOpaqueLib = Native.load("somelib", libClass)
        
        fun new_(i: Int): Res<ResultOpaque, ErrorEnum> {
            val returnVal = lib.ResultOpaque_new(i);
        
            if (returnVal.isOk == 1.toByte()) {
                val selfEdges: List<Any> = listOf()
                val handle = returnVal.union.ok 
                val returnOpaque = ResultOpaque(handle, selfEdges)
                CLEANER.register(returnOpaque, ResultOpaque.ResultOpaqueCleaner(handle, ResultOpaque.lib));
                
                return returnOpaque.ok()
            } else {
                return ErrorEnum.fromNative(returnVal.union.err).err()
            }
        }
        
        fun newFailingFoo(): Res<ResultOpaque, ErrorEnum> {
            val returnVal = lib.ResultOpaque_new_failing_foo();
        
            if (returnVal.isOk == 1.toByte()) {
                val selfEdges: List<Any> = listOf()
                val handle = returnVal.union.ok 
                val returnOpaque = ResultOpaque(handle, selfEdges)
                CLEANER.register(returnOpaque, ResultOpaque.ResultOpaqueCleaner(handle, ResultOpaque.lib));
                
                return returnOpaque.ok()
            } else {
                return ErrorEnum.fromNative(returnVal.union.err).err()
            }
        }
        
        fun newFailingBar(): Res<ResultOpaque, ErrorEnum> {
            val returnVal = lib.ResultOpaque_new_failing_bar();
        
            if (returnVal.isOk == 1.toByte()) {
                val selfEdges: List<Any> = listOf()
                val handle = returnVal.union.ok 
                val returnOpaque = ResultOpaque(handle, selfEdges)
                CLEANER.register(returnOpaque, ResultOpaque.ResultOpaqueCleaner(handle, ResultOpaque.lib));
                
                return returnOpaque.ok()
            } else {
                return ErrorEnum.fromNative(returnVal.union.err).err()
            }
        }
        
        fun newFailingUnit(): Res<ResultOpaque, Unit> {
            val returnVal = lib.ResultOpaque_new_failing_unit();
        
            if (returnVal.isOk == 1.toByte()) {
                val selfEdges: List<Any> = listOf()
                val handle = returnVal.union.ok 
                val returnOpaque = ResultOpaque(handle, selfEdges)
                CLEANER.register(returnOpaque, ResultOpaque.ResultOpaqueCleaner(handle, ResultOpaque.lib));
                
                return returnOpaque.ok()
            } else {
                return Err(Unit)
            }
        }
        
        fun newFailingStruct(i: Int): Res<ResultOpaque, ErrorStruct> {
            val returnVal = lib.ResultOpaque_new_failing_struct(i);
        
            if (returnVal.isOk == 1.toByte()) {
                val selfEdges: List<Any> = listOf()
                val handle = returnVal.union.ok 
                val returnOpaque = ResultOpaque(handle, selfEdges)
                CLEANER.register(returnOpaque, ResultOpaque.ResultOpaqueCleaner(handle, ResultOpaque.lib));
                
                return returnOpaque.ok()
            } else {
                
                val returnStruct = ErrorStruct(returnVal.union.err)
                return returnStruct.err()
            }
        }
        
        fun newInErr(i: Int): Res<Unit, ResultOpaque> {
            val returnVal = lib.ResultOpaque_new_in_err(i);
        
            if (returnVal.isOk == 1.toByte()) {
                return Ok(Unit)
            } else {
                val selfEdges: List<Any> = listOf()
                val handle = returnVal.union.err 
                val returnOpaque = ResultOpaque(handle, selfEdges)
                CLEANER.register(returnOpaque, ResultOpaque.ResultOpaqueCleaner(handle, ResultOpaque.lib));
                
                return returnOpaque.err()
            }
        }
        
        fun newInt(i: Int): Res<Int, Unit> {
            val returnVal = lib.ResultOpaque_new_int(i);
        
            if (returnVal.isOk == 1.toByte()) {
                return returnVal.union.ok.ok()
            } else {
                return Err(Unit)
            }
        }
        
        fun newInEnumErr(i: Int): Res<ErrorEnum, ResultOpaque> {
            val returnVal = lib.ResultOpaque_new_in_enum_err(i);
        
            if (returnVal.isOk == 1.toByte()) {
                return ErrorEnum.fromNative(returnVal.union.ok).ok()
            } else {
                val selfEdges: List<Any> = listOf()
                val handle = returnVal.union.err 
                val returnOpaque = ResultOpaque(handle, selfEdges)
                CLEANER.register(returnOpaque, ResultOpaque.ResultOpaqueCleaner(handle, ResultOpaque.lib));
                
                return returnOpaque.err()
            }
        }
    }
    
    fun assertInteger(i: Int): Unit {
        val returnVal = lib.ResultOpaque_assert_integer(handle, i);
    
    }

}
