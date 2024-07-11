package dev.diplomattest.somelib;
import com.sun.jna.Library
import com.sun.jna.Native
import com.sun.jna.Pointer


internal interface ICU4XFixedDecimalLib: Library {
    fun ICU4XFixedDecimal_destroy(handle: Pointer)
    fun icu4x_ICU4XFixedDecimal_new_mv1(v: Int): Pointer
    fun icu4x_ICU4XFixedDecimal_multiply_pow10_mv1(handle: Pointer, power: Short): Unit
    fun icu4x_ICU4XFixedDecimal_to_string_mv1(handle: Pointer, write: Pointer): ResultUnitUnit
}

class ICU4XFixedDecimal internal constructor (
    internal val handle: Pointer,
    // These ensure that anything that is borrowed is kept alive and not cleaned
    // up by the garbage collector.
    internal val selfEdges: List<Any>
)  {

    internal class ICU4XFixedDecimalCleaner(val handle: Pointer, val lib: ICU4XFixedDecimalLib) : Runnable {
        override fun run() {
            lib.ICU4XFixedDecimal_destroy(handle)
        }
    }

    companion object {
        internal val libClass: Class<ICU4XFixedDecimalLib> = ICU4XFixedDecimalLib::class.java
        internal val lib: ICU4XFixedDecimalLib = Native.load("somelib", libClass)
        
        fun new_(v: Int): ICU4XFixedDecimal {
            
            val returnVal = lib.icu4x_ICU4XFixedDecimal_new_mv1(v);
            val selfEdges: List<Any> = listOf()
            val handle = returnVal 
            val returnOpaque = ICU4XFixedDecimal(handle, selfEdges)
            CLEANER.register(returnOpaque, ICU4XFixedDecimal.ICU4XFixedDecimalCleaner(handle, ICU4XFixedDecimal.lib));
            
            return returnOpaque
        }
    }
    
    fun multiplyPow10(power: Short): Unit {
        
        val returnVal = lib.icu4x_ICU4XFixedDecimal_multiply_pow10_mv1(handle, power);
        
    }
    
    fun toString_(): Res<String, Unit> {
        val write = DW.lib.diplomat_buffer_write_create(0)
        val returnVal = lib.icu4x_ICU4XFixedDecimal_to_string_mv1(handle, write);
        if (returnVal.isOk == 1.toByte()) {
            
            val returnString = DW.writeToString(write)
            return returnString.ok()
        } else {
            return Err(Unit)
        }
    }

}
