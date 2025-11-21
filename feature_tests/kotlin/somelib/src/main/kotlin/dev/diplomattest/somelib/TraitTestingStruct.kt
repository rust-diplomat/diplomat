package dev.diplomattest.somelib

import com.sun.jna.Callback
import com.sun.jna.Library
import com.sun.jna.Native
import com.sun.jna.Pointer
import com.sun.jna.Structure

internal interface TraitTestingStructLib: Library {
}

internal class TraitTestingStructNative: Structure(), Structure.ByValue {
    @JvmField
    internal var x: Int = 0;
    @JvmField
    internal var y: Int = 0;

    // Define the fields of the struct
    override fun getFieldOrder(): List<String> {
        return listOf("x", "y")
    }
}




internal class OptionTraitTestingStructNative: Structure(), Structure.ByValue  {
    @JvmField
    internal var value: TraitTestingStructNative = TraitTestingStructNative()

    @JvmField
    internal var isOk: Byte = 0

    // Define the fields of the struct
    override fun getFieldOrder(): List<String> {
        return listOf("value", "isOk")
    }

    internal fun option(): TraitTestingStructNative? {
        if (isOk == 1.toByte()) {
            return value
        } else {
            return null
        }
    }
}

class TraitTestingStruct internal constructor (
    internal val nativeStruct: TraitTestingStructNative) {
    val x: Int = nativeStruct.x
    val y: Int = nativeStruct.y

    companion object {
        internal val libClass: Class<TraitTestingStructLib> = TraitTestingStructLib::class.java
        internal val lib: TraitTestingStructLib = Native.load("diplomat_feature_tests", libClass)
        val NATIVESIZE: Long = Native.getNativeSize(TraitTestingStructNative::class.java).toLong()
    }

}
