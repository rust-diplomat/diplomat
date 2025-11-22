package dev.diplomattest.somelib

import com.sun.jna.Callback
import com.sun.jna.Library
import com.sun.jna.Native
import com.sun.jna.Pointer
import com.sun.jna.Structure

internal interface ErrorStructLib: Library {
}

internal class ErrorStructNative: Structure(), Structure.ByValue {
    @JvmField
    internal var i: Int = 0;
    @JvmField
    internal var j: Int = 0;

    // Define the fields of the struct
    override fun getFieldOrder(): List<String> {
        return listOf("i", "j")
    }
}




internal class OptionErrorStructNative constructor(): Structure(), Structure.ByValue {
    @JvmField
    internal var value: ErrorStructNative = ErrorStructNative()

    @JvmField
    internal var isOk: Byte = 0

    // Define the fields of the struct
    override fun getFieldOrder(): List<String> {
        return listOf("value", "isOk")
    }

    internal fun option(): ErrorStructNative? {
        if (isOk == 1.toByte()) {
            return value
        } else {
            return null
        }
    }


    constructor(value: ErrorStructNative, isOk: Byte): this() {
        this.value = value
        this.isOk = isOk
    }

    companion object {
        internal fun some(value: ErrorStructNative): OptionErrorStructNative {
            return OptionErrorStructNative(value, 1)
        }

        internal fun none(): OptionErrorStructNative {
            return OptionErrorStructNative(ErrorStructNative(), 0)
        }
    }

}

class ErrorStruct (var i: Int, var j: Int): Exception("Rust error result for ErrorStruct") {
    companion object {

        internal val libClass: Class<ErrorStructLib> = ErrorStructLib::class.java
        internal val lib: ErrorStructLib = Native.load("diplomat_feature_tests", libClass)
        val NATIVESIZE: Long = Native.getNativeSize(ErrorStructNative::class.java).toLong()

        internal fun fromNative(nativeStruct: ErrorStructNative): ErrorStruct {
            val i: Int = nativeStruct.i
            val j: Int = nativeStruct.j

            return ErrorStruct(i, j)
        }

    }
    internal fun toNative(): ErrorStructNative {
        var native = ErrorStructNative()
        native.i = this.i
        native.j = this.j
        return native
    }

}