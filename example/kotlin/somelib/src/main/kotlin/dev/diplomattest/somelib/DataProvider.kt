package dev.diplomattest.somelib;
import com.sun.jna.Callback
import com.sun.jna.Library
import com.sun.jna.Native
import com.sun.jna.Pointer
import com.sun.jna.Structure

internal interface DataProviderLib: Library {
    fun icu4x_DataProvider_destroy_mv1(handle: Pointer)
    fun icu4x_DataProvider_new_static_mv1(): Pointer
    fun icu4x_DataProvider_returns_result_mv1(): ResultUnitUnit
}
/** An  data provider, capable of loading  data keys from some source.
*
*See the [Rust documentation for `icu_provider`](https://docs.rs/icu_provider/latest/icu_provider/index.html) for more information.
*/
class DataProvider internal constructor (
    internal val handle: Pointer,
    // These ensure that anything that is borrowed is kept alive and not cleaned
    // up by the garbage collector.
    internal val selfEdges: List<Any>,
    internal var owned: Boolean,
)  {

    init {
        if (this.owned) {
            this.registerCleaner()
        }
    }

    private class DataProviderCleaner(val handle: Pointer, val lib: DataProviderLib) : Runnable {
        override fun run() {
            lib.icu4x_DataProvider_destroy_mv1(handle)
        }
    }
    private fun registerCleaner() {
        CLEANER.register(this, DataProvider.DataProviderCleaner(handle, DataProvider.lib));
    }

    companion object {
        internal val libClass: Class<DataProviderLib> = DataProviderLib::class.java
        internal val lib: DataProviderLib = Native.load("diplomat_example", libClass)
        @JvmStatic
        
        /** See the [Rust documentation for `get_static_provider`](https://docs.rs/icu_testdata/latest/icu_testdata/fn.get_static_provider.html) for more information.
        */
        fun newStatic(): DataProvider {
            
            val returnVal = lib.icu4x_DataProvider_new_static_mv1();
            val selfEdges: List<Any> = listOf()
            val handle = returnVal 
            val returnOpaque = DataProvider(handle, selfEdges, true)
            return returnOpaque
        }
        @JvmStatic
        
        /** This exists as a regression test for https://github.com/rust-diplomat/diplomat/issues/155
        */
        fun returnsResult(): Result<Unit> {
            
            val returnVal = lib.icu4x_DataProvider_returns_result_mv1();
            val nativeOkVal = returnVal.getNativeOk();
            if (nativeOkVal != null) {
                return Unit.ok()
            } else {
                return UnitError().err()
            }
        }
    }

}