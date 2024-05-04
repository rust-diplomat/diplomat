package dev.diplomattest.somelib

import com.sun.jna.Library
import com.sun.jna.Native
import com.sun.jna.Pointer
import com.sun.jna.Structure

internal interface ICU4XFixedDecimalFormatterOptionsLib: Library {
    fun ICU4XFixedDecimalFormatterOptions_default(): ICU4XFixedDecimalFormatterOptionsNative
}

class ICU4XFixedDecimalFormatterOptionsNative: Structure(), Structure.ByValue {
    @JvmField
    var groupingStrategy: Int = ICU4XFixedDecimalGroupingStrategy.default().toNative();
    @JvmField
    var someOtherConfig: Byte = 0;
  
    // Define the fields of the struct
    override fun getFieldOrder(): List<String> {
        return listOf("groupingStrategy", "someOtherConfig")
    }
}

class ICU4XFixedDecimalFormatterOptions internal constructor (
    internal val nativeStruct: ICU4XFixedDecimalFormatterOptionsNative) {
    val groupingStrategy: ICU4XFixedDecimalGroupingStrategy = ICU4XFixedDecimalGroupingStrategy.fromNative(nativeStruct.groupingStrategy)
    val someOtherConfig: Boolean = nativeStruct.someOtherConfig > 0

    companion object {
        internal val libClass: Class<ICU4XFixedDecimalFormatterOptionsLib> = ICU4XFixedDecimalFormatterOptionsLib::class.java
        internal val lib: ICU4XFixedDecimalFormatterOptionsLib = Native.load("somelib", libClass)
        val NATIVESIZE: Long = Native.getNativeSize(ICU4XFixedDecimalFormatterOptionsNative::class.java).toLong()
        
        fun default_(): ICU4XFixedDecimalFormatterOptions {
            val returnVal = lib.ICU4XFixedDecimalFormatterOptions_default();
        
            val returnStruct = ICU4XFixedDecimalFormatterOptions(returnVal)
            return returnStruct
        }
    }

}
