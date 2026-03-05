package dev.diplomattest.somelib

import com.sun.jna.Callback
import com.sun.jna.Library
import com.sun.jna.Native
import com.sun.jna.Pointer
import com.sun.jna.Structure

internal interface NestedBorrowedFieldsLib: Library {
    fun NestedBorrowedFields_from_bar_and_foo_and_strings(bar: Pointer, foo: Pointer, dstr16X: Slice, dstr16Z: Slice, utf8StrY: Slice, utf8StrZ: Slice): NestedBorrowedFieldsNative
}

internal class NestedBorrowedFieldsNative: Structure(), Structure.ByValue {
    @JvmField
    internal var fields: BorrowedFieldsNative = BorrowedFieldsNative();
    @JvmField
    internal var bounds: BorrowedFieldsWithBoundsNative = BorrowedFieldsWithBoundsNative();
    @JvmField
    internal var bounds2: BorrowedFieldsWithBoundsNative = BorrowedFieldsWithBoundsNative();

    // Define the fields of the struct
    override fun getFieldOrder(): List<String> {
        return listOf("fields", "bounds", "bounds2")
    }
}




internal class OptionNestedBorrowedFieldsNative constructor(): Structure(), Structure.ByValue {
    @JvmField
    internal var value: NestedBorrowedFieldsNative = NestedBorrowedFieldsNative()

    @JvmField
    internal var isOk: Byte = 0

    // Define the fields of the struct
    override fun getFieldOrder(): List<String> {
        return listOf("value", "isOk")
    }

    internal fun option(): NestedBorrowedFieldsNative? {
        if (isOk == 1.toByte()) {
            return value
        } else {
            return null
        }
    }


    constructor(value: NestedBorrowedFieldsNative, isOk: Byte): this() {
        this.value = value
        this.isOk = isOk
    }

    companion object {
        internal fun some(value: NestedBorrowedFieldsNative): OptionNestedBorrowedFieldsNative {
            return OptionNestedBorrowedFieldsNative(value, 1)
        }

        internal fun none(): OptionNestedBorrowedFieldsNative {
            return OptionNestedBorrowedFieldsNative(NestedBorrowedFieldsNative(), 0)
        }
    }

}

class NestedBorrowedFields (var fields: BorrowedFields, var bounds: BorrowedFieldsWithBounds, var bounds2: BorrowedFieldsWithBounds) {
    companion object {

        internal val libClass: Class<NestedBorrowedFieldsLib> = NestedBorrowedFieldsLib::class.java
        internal val lib: NestedBorrowedFieldsLib = Native.load("diplomat_feature_tests", libClass)
        val NATIVESIZE: Long = Native.getNativeSize(NestedBorrowedFieldsNative::class.java).toLong()

        internal fun fromNative(nativeStruct: NestedBorrowedFieldsNative, xEdges: List<Any?>, yEdges: List<Any?>, zEdges: List<Any?>): NestedBorrowedFields {
            val fields: BorrowedFields = BorrowedFields.fromNative(nativeStruct.fields, xEdges)
            val bounds: BorrowedFieldsWithBounds = BorrowedFieldsWithBounds.fromNative(nativeStruct.bounds, xEdges, yEdges, yEdges)
            val bounds2: BorrowedFieldsWithBounds = BorrowedFieldsWithBounds.fromNative(nativeStruct.bounds2, zEdges, zEdges, zEdges)

            return NestedBorrowedFields(fields, bounds, bounds2)
        }

        @JvmStatic
        
        fun fromBarAndFooAndStrings(bar: Bar, foo: Foo, dstr16X: String, dstr16Z: String, utf8StrY: String, utf8StrZ: String): NestedBorrowedFields {
            // This lifetime edge depends on lifetimes: 'x, 'y
            val xEdges: MutableList<Any> = mutableListOf(bar);
            // This lifetime edge depends on lifetimes: 'y
            val yEdges: MutableList<Any> = mutableListOf(bar);
            // This lifetime edge depends on lifetimes: 'z
            val zEdges: MutableList<Any> = mutableListOf(foo);
            val dstr16XSliceMemory = PrimitiveArrayTools.borrowUtf16(dstr16X).into(listOf(xEdges))
            val dstr16ZSliceMemory = PrimitiveArrayTools.borrowUtf16(dstr16Z).into(listOf(zEdges))
            val utf8StrYSliceMemory = PrimitiveArrayTools.borrowUtf8(utf8StrY).into(listOf(yEdges))
            val utf8StrZSliceMemory = PrimitiveArrayTools.borrowUtf8(utf8StrZ).into(listOf(zEdges))
            
            val returnVal = lib.NestedBorrowedFields_from_bar_and_foo_and_strings(bar.handle, foo.handle, dstr16XSliceMemory.slice, dstr16ZSliceMemory.slice, utf8StrYSliceMemory.slice, utf8StrZSliceMemory.slice);
            val returnStruct = NestedBorrowedFields.fromNative(returnVal, xEdges, yEdges, zEdges)
            return returnStruct
        }
    }
    internal fun toNative(xAppendArray: Array<MutableList<Any>>, yAppendArray: Array<MutableList<Any>>, zAppendArray: Array<MutableList<Any>>): NestedBorrowedFieldsNative {
        var native = NestedBorrowedFieldsNative()
        native.fields = this.fields.toNative(aAppendArray = xAppendArray)
        native.bounds = this.bounds.toNative(aAppendArray = xAppendArray, bAppendArray = yAppendArray, cAppendArray = yAppendArray)
        native.bounds2 = this.bounds2.toNative(aAppendArray = zAppendArray, bAppendArray = zAppendArray, cAppendArray = zAppendArray)
        return native
    }

    internal fun xEdges(): List<Any?> {
        return TODO("todo")
    }
    internal fun yEdges(): List<Any?> {
        return TODO("todo")
    }
    internal fun zEdges(): List<Any?> {
        return TODO("todo")
    }
}