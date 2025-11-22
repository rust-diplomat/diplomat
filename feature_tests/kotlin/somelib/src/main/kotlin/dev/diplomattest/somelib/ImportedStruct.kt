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




internal class OptionImportedStructNative constructor(): Structure(), Structure.ByValue {
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


    constructor(value: ImportedStructNative, isOk: Byte): this() {
        this.value = value
        this.isOk = isOk
    }

    companion object {
        internal fun some(value: ImportedStructNative): OptionImportedStructNative {
            return OptionImportedStructNative(value, 1)
        }

        internal fun none(): OptionImportedStructNative {
            return OptionImportedStructNative(ImportedStructNative(), 0)
        }
    }

}

class ImportedStruct (var foo: UnimportedEnum, var count: UByte) {
    companion object {

        internal val libClass: Class<ImportedStructLib> = ImportedStructLib::class.java
        internal val lib: ImportedStructLib = Native.load("diplomat_feature_tests", libClass)
        val NATIVESIZE: Long = Native.getNativeSize(ImportedStructNative::class.java).toLong()

        internal fun fromNative(nativeStruct: ImportedStructNative): ImportedStruct {
            val foo: UnimportedEnum = UnimportedEnum.fromNative(nativeStruct.foo)
            val count: UByte = nativeStruct.count.toUByte()

            return ImportedStruct(foo, count)
        }

    }
    internal fun toNative(): ImportedStructNative {
        var native = ImportedStructNative()
        native.foo = this.foo.toNative()
        native.count = FFIUint8(this.count)
        return native
    }

}