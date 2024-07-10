package dev.diplomattest.somelib;
import com.sun.jna.Library
import com.sun.jna.Native
import com.sun.jna.Pointer


internal interface ICU4XDataProviderLib: Library {
    fun ICU4XDataProvider_destroy(handle: Pointer)
    fun icu4x_ICU4XDataProvider_new_static_mv1(): Pointer
    fun icu4x_ICU4XDataProvider_returns_result_mv1(): ResultUnitUnit
}

class ICU4XDataProvider internal constructor (
    internal val handle: Pointer,
    // These ensure that anything that is borrowed is kept alive and not cleaned
    // up by the garbage collector.
    internal val selfEdges: List<Any>
)  {

    internal class ICU4XDataProviderCleaner(val handle: Pointer, val lib: ICU4XDataProviderLib) : Runnable {
        override fun run() {
            lib.ICU4XDataProvider_destroy(handle)
        }
    }

    companion object {
        internal val libClass: Class<ICU4XDataProviderLib> = ICU4XDataProviderLib::class.java
        internal val lib: ICU4XDataProviderLib = Native.load("somelib", libClass)
        
        fun newStatic(): ICU4XDataProvider {
            
            val returnVal = lib.icu4x_ICU4XDataProvider_new_static_mv1();
            val selfEdges: List<Any> = listOf()
            val handle = returnVal 
            val returnOpaque = ICU4XDataProvider(handle, selfEdges)
            CLEANER.register(returnOpaque, ICU4XDataProvider.ICU4XDataProviderCleaner(handle, ICU4XDataProvider.lib));
            
            return returnOpaque
        }
        
        fun returnsResult(): Res<Unit, Unit> {
            
            val returnVal = lib.icu4x_ICU4XDataProvider_returns_result_mv1();
            if (returnVal.isOk == 1.toByte()) {
                Unit.ok()
            } else {
                return Err(Unit)
            }
        }
    }

}
