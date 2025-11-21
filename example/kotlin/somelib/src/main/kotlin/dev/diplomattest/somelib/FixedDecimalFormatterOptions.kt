package dev.diplomattest.somelib

import com.sun.jna.Callback
import com.sun.jna.Library
import com.sun.jna.Native
import com.sun.jna.Pointer
import com.sun.jna.Structure

internal interface FixedDecimalFormatterOptionsLib: Library {
    fun icu4x_FixedDecimalFormatterOptions_default_mv1(): FixedDecimalFormatterOptionsNative
}

internal class FixedDecimalFormatterOptionsNative: Structure(), Structure.ByValue {
    @JvmField
    internal var groupingStrategy: Int = FixedDecimalGroupingStrategy.default().toNative();
    @JvmField
    internal var someOtherConfig: Byte = 0;

    // Define the fields of the struct
    override fun getFieldOrder(): List<String> {
        return listOf("groupingStrategy", "someOtherConfig")
    }
}




internal class OptionFixedDecimalFormatterOptionsNative constructor(): Structure(), Structure.ByValue {
    @JvmField
    internal var value: FixedDecimalFormatterOptionsNative = FixedDecimalFormatterOptionsNative()

    @JvmField
    internal var isOk: Byte = 0

    // Define the fields of the struct
    override fun getFieldOrder(): List<String> {
        return listOf("value", "isOk")
    }

    internal fun option(): FixedDecimalFormatterOptionsNative? {
        if (isOk == 1.toByte()) {
            return value
        } else {
            return null
        }
    }


    constructor(value: FixedDecimalFormatterOptionsNative, isOk: Byte): this() {
        this.value = value
        this.isOk = isOk
    }

    companion object {
        internal fun some(value: FixedDecimalFormatterOptionsNative): OptionFixedDecimalFormatterOptionsNative {
            return OptionFixedDecimalFormatterOptionsNative(value, 1)
        }

        internal fun none(): OptionFixedDecimalFormatterOptionsNative {
            return OptionFixedDecimalFormatterOptionsNative(FixedDecimalFormatterOptionsNative(), 0)
        }
    }

}

class FixedDecimalFormatterOptions internal constructor (
    internal val nativeStruct: FixedDecimalFormatterOptionsNative) {
    val groupingStrategy: FixedDecimalGroupingStrategy = FixedDecimalGroupingStrategy.fromNative(nativeStruct.groupingStrategy)
    val someOtherConfig: Boolean = nativeStruct.someOtherConfig > 0

    companion object {
        internal val libClass: Class<FixedDecimalFormatterOptionsLib> = FixedDecimalFormatterOptionsLib::class.java
        internal val lib: FixedDecimalFormatterOptionsLib = Native.load("diplomat_example", libClass)
        val NATIVESIZE: Long = Native.getNativeSize(FixedDecimalFormatterOptionsNative::class.java).toLong()
        @JvmStatic
        
        fun default_(): FixedDecimalFormatterOptions {
            
            val returnVal = lib.icu4x_FixedDecimalFormatterOptions_default_mv1();
            
            val returnStruct = FixedDecimalFormatterOptions(returnVal)
            return returnStruct
        }
    }

}
