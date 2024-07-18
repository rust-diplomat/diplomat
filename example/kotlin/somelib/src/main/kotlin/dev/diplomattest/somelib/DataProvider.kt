package dev.diplomattest.somelib;
import com.sun.jna.Library
import com.sun.jna.Native
import com.sun.jna.Pointer


internal interface DataProviderLib: Library {
    fun DataProvider_destroy(handle: Pointer)
    fun icu4x_DataProvider_new_static_mv1(): Pointer
    fun icu4x_DataProvider_returns_result_mv1(): ResultUnitUnit
}

class DataProvider internal constructor (
    internal val handle: Pointer,
    // These ensure that anything that is borrowed is kept alive and not cleaned
    // up by the garbage collector.
    internal val selfEdges: List<Any>
)  {

    internal class DataProviderCleaner(val handle: Pointer, val lib: DataProviderLib) : Runnable {
        override fun run() {
            lib.DataProvider_destroy(handle)
        }
    }

    companion object {
        internal val libClass: Class<DataProviderLib> = DataProviderLib::class.java
        internal val lib: DataProviderLib = Native.load("somelib", libClass)
        
        fun newStatic(): DataProvider {
            
            val returnVal = lib.icu4x_DataProvider_new_static_mv1();
            val selfEdges: List<Any> = listOf()
            val handle = returnVal 
            val returnOpaque = DataProvider(handle, selfEdges)
            CLEANER.register(returnOpaque, DataProvider.DataProviderCleaner(handle, DataProvider.lib));
            
            return returnOpaque
        }
        
        fun returnsResult(): Res<Unit, Unit> {
            
            val returnVal = lib.icu4x_DataProvider_returns_result_mv1();
            if (returnVal.isOk == 1.toByte()) {
                Unit.ok()
            } else {
                return Err(Unit)
            }
        }
    }

}
