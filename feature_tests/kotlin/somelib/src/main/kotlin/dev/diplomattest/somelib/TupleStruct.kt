package dev.diplomattest.somelib

import com.sun.jna.Callback
import com.sun.jna.Library
import com.sun.jna.Native
import com.sun.jna.Pointer
import com.sun.jna.Structure

internal interface TupleStructLib: Library {
    fun TupleStruct_takes_st_as_tuple(a: TupleStructNative): Int
    fun TupleStruct_takes_containing(c: ContainingTupleNative): Int
}

internal class TupleStructNative: Structure(), Structure.ByValue {
    @JvmField
    internal var x: Int = 0;
    @JvmField
    internal var y: Int = 0;
    @JvmField
    internal var st: MyStructNative = MyStructNative();
    @JvmField
    internal var op: Pointer = Pointer(0);

    // Define the fields of the struct
    override fun getFieldOrder(): List<String> {
        return listOf("x", "y", "st", "op")
    }
}




internal class OptionTupleStructNative constructor(): Structure(), Structure.ByValue {
    @JvmField
    internal var value: TupleStructNative = TupleStructNative()

    @JvmField
    internal var isOk: Byte = 0

    // Define the fields of the struct
    override fun getFieldOrder(): List<String> {
        return listOf("value", "isOk")
    }

    internal fun option(): TupleStructNative? {
        if (isOk == 1.toByte()) {
            return value
        } else {
            return null
        }
    }


    constructor(value: TupleStructNative, isOk: Byte): this() {
        this.value = value
        this.isOk = isOk
    }

    companion object {
        internal fun some(value: TupleStructNative): OptionTupleStructNative {
            return OptionTupleStructNative(value, 1)
        }

        internal fun none(): OptionTupleStructNative {
            return OptionTupleStructNative(TupleStructNative(), 0)
        }
    }

}

class TupleStruct (var x: Int, var y: Int, var st: MyStruct, var op: Opaque) {
    companion object {

        internal val libClass: Class<TupleStructLib> = TupleStructLib::class.java
        internal val lib: TupleStructLib = Native.load("diplomat_feature_tests", libClass)
        val NATIVESIZE: Long = Native.getNativeSize(TupleStructNative::class.java).toLong()

        internal fun fromNative(nativeStruct: TupleStructNative, aEdges: List<Any?>): TupleStruct {
            val x: Int = nativeStruct.x
            val y: Int = nativeStruct.y
            val st: MyStruct = MyStruct.fromNative(nativeStruct.st)
            val op: Opaque = Opaque(nativeStruct.op, listOf(), false)

            return TupleStruct(x, y, st, op)
        }

        @JvmStatic
        
        fun takesStAsTuple(a: TupleStruct): Int {
            val temporaryEdgeArena: MutableList<Any> = mutableListOf()
            
            val returnVal = lib.TupleStruct_takes_st_as_tuple(a.toNative(aAppendArray = arrayOf(temporaryEdgeArena)));
            return (returnVal)
        }
        @JvmStatic
        
        fun takesContaining(c: ContainingTuple): Int {
            val temporaryEdgeArena: MutableList<Any> = mutableListOf()
            
            val returnVal = lib.TupleStruct_takes_containing(c.toNative(aAppendArray = arrayOf(temporaryEdgeArena)));
            return (returnVal)
        }
    }
    internal fun toNative(aAppendArray: Array<MutableList<Any>>): TupleStructNative {
        var native = TupleStructNative()
        native.x = this.x
        native.y = this.y
        native.st = this.st.toNative()
        native.op = this.op.handle
        return native
    }

    internal fun aEdges(): List<Any?> {
        return TODO("todo")
    }
}