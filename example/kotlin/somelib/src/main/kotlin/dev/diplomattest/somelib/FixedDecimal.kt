package dev.diplomattest.somelib;
import com.sun.jna.Library
import com.sun.jna.Native
import com.sun.jna.Pointer


internal interface FixedDecimalLib: Library {
    fun FixedDecimal_destroy(handle: Pointer)
    fun icu4x_FixedDecimal_new_mv1(v: Int): Pointer
    fun icu4x_FixedDecimal_multiply_pow10_mv1(handle: Pointer, power: Short): Unit
    fun icu4x_FixedDecimal_to_string_mv1(handle: Pointer, write: Pointer): ResultUnitUnit
}

class FixedDecimal internal constructor (
    internal val handle: Pointer,
    // These ensure that anything that is borrowed is kept alive and not cleaned
    // up by the garbage collector.
    internal val selfEdges: List<Any>
)  {

    internal class FixedDecimalCleaner(val handle: Pointer, val lib: FixedDecimalLib) : Runnable {
        override fun run() {
            lib.FixedDecimal_destroy(handle)
        }
    }

    companion object {
        internal val libClass: Class<FixedDecimalLib> = FixedDecimalLib::class.java
        internal val lib: FixedDecimalLib = Native.load("somelib", libClass)
        
        fun new_(v: Int): FixedDecimal {
            
            val returnVal = lib.icu4x_FixedDecimal_new_mv1(v);
            val selfEdges: List<Any> = listOf()
            val handle = returnVal 
            val returnOpaque = FixedDecimal(handle, selfEdges)
            CLEANER.register(returnOpaque, FixedDecimal.FixedDecimalCleaner(handle, FixedDecimal.lib));
            
            return returnOpaque
        }
    }
    
    fun multiplyPow10(power: Short): Unit {
        
        val returnVal = lib.icu4x_FixedDecimal_multiply_pow10_mv1(handle, power);
        
    }
    
    fun toString_(): Res<String, Unit> {
        val write = DW.lib.diplomat_buffer_write_create(0)
        val returnVal = lib.icu4x_FixedDecimal_to_string_mv1(handle, write);
        if (returnVal.isOk == 1.toByte()) {
            
            val returnString = DW.writeToString(write)
            return returnString.ok()
        } else {
            return Err(Unit)
        }
    }

}
