package dev.diplomattest.somelib

import com.sun.jna.Callback
import com.sun.jna.Library
import com.sun.jna.Native
import com.sun.jna.Pointer
import com.sun.jna.Structure

internal interface ImportedStructLib: Library {
}

internal class ImportedStructNative: Structure(), Structure.ByValue {
    @JvmField
    internal var foo: Int = UnimportedEnum.default().toNative();
    @JvmField
    internal var count: FFIUint8 = FFIUint8();

    // Define the fields of the struct
    override fun getFieldOrder(): List<String> {
        return listOf("foo", "count")
    }
}




internal class OptionImportedStructNative: Structure(), Structure.ByValue  {
    @JvmField
    internal var value: ImportedStructNative = ImportedStructNative()

    @JvmField
    internal var isOk: Byte = 0

    // Define the fields of the struct
    override fun getFieldOrder(): List<String> {
        return listOf("value", "isOk")
    }

    internal fun option(): ImportedStructNative? {
        if (isOk == 1.toByte()) {
            return value
        } else {
            return null
        }
    }
}

class ImportedStruct internal constructor (
    internal val nativeStruct: ImportedStructNative) {
    val foo: UnimportedEnum = UnimportedEnum.fromNative(nativeStruct.foo)
    val count: UByte = nativeStruct.count.toUByte()

    companion object {
        internal val libClass: Class<ImportedStructLib> = ImportedStructLib::class.java
        internal val lib: ImportedStructLib = Native.load("diplomat_feature_tests", libClass)
        val NATIVESIZE: Long = Native.getNativeSize(ImportedStructNative::class.java).toLong()
    }

}
