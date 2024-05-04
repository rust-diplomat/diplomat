package dev.diplomattest.somelib

import com.sun.jna.Library
import com.sun.jna.Memory
import com.sun.jna.Native
import com.sun.jna.Pointer
import com.sun.jna.Structure
import com.sun.jna.Union

// We spawn a cleaner for the library which is responsible for cleaning opaque types.
val CLEANER = java.lang.ref.Cleaner.create()

interface DiplomatWriteableLib : Library {
    fun diplomat_buffer_writeable_create(size: Long): Pointer
    fun diplomat_buffer_writeable_get_bytes(diplomatWriteable: Pointer): Pointer
    fun diplomat_buffer_writeable_len(diplomatWriteable: Pointer): Long
    fun diplomat_buffer_writeable_destroy(diplomatWriteable: Pointer)
}

object DW {

    val libClass: Class<DiplomatWriteableLib> = DiplomatWriteableLib::class.java
    val lib: DiplomatWriteableLib = Native.load("somelib", libClass)

    fun writeableToString(writeable: Pointer): String {
        val pointer = lib.diplomat_buffer_writeable_get_bytes(writeable)
        val len = lib.diplomat_buffer_writeable_len(writeable)
        val bytes = pointer.getByteArray(0, len.toInt())
        return bytes.decodeToString()
    }
}

internal object PrimitiveArrayTools {

    fun native(boolArray: BooleanArray): Pair<Memory, Slice> {
        val mem = Memory(boolArray.size.toLong())
        val ptr = mem.share(0)
        val byteArray = boolArray.map { if (it) 1.toByte() else 0.toByte() }.toByteArray()
        ptr.write(0, byteArray, 0, byteArray.size)
        val slice = Slice()
        slice.data = ptr
        slice.len = byteArray.size.toLong()
        return Pair(mem, slice)
    }

    fun native(byteArray: ByteArray): Pair<Memory, Slice> {
        val mem = Memory(byteArray.size.toLong())
        val ptr = mem.share(0)
        ptr.write(0, byteArray, 0, byteArray.size)
        val slice = Slice()
        slice.data = ptr
        slice.len = byteArray.size.toLong()
        return Pair(mem, slice)
    }

    @ExperimentalUnsignedTypes
    fun native(uByteArray: UByteArray): Pair<Memory, Slice> {
        val byteArray = uByteArray.asByteArray()
        val mem = Memory(byteArray.size.toLong())
        val ptr = mem.share(0)
        ptr.write(0, byteArray, 0, byteArray.size)
        val slice = Slice()
        slice.data = ptr
        slice.len = uByteArray.size.toLong()
        return Pair(mem, slice)
    }

    fun native(shortArray: ShortArray): Pair<Memory, Slice> {
        val mem = Memory(Short.SIZE_BYTES * shortArray.size.toLong())
        val ptr = mem.share(0)
        ptr.write(0, shortArray, 0, shortArray.size)
        val slice = Slice()
        slice.data = ptr
        slice.len = shortArray.size.toLong()
        return Pair(mem, slice)
    }

    @ExperimentalUnsignedTypes
    fun native(uShortArray: UShortArray): Pair<Memory, Slice> {
        val shortArray = uShortArray.asShortArray()
        val mem = Memory(Short.SIZE_BYTES * shortArray.size.toLong())
        val ptr = mem.share(0)
        ptr.write(0, shortArray, 0, shortArray.size)
        val slice = Slice()
        slice.data = ptr
        slice.len = uShortArray.size.toLong()
        return Pair(mem, slice)
    }

    fun native(intArray: IntArray): Pair<Memory, Slice> {
        val mem = Memory(Int.SIZE_BYTES * intArray.size.toLong())
        val ptr = mem.share(0)
        ptr.write(0, intArray, 0, intArray.size)
        val slice = Slice()
        slice.data = ptr
        slice.len = intArray.size.toLong()
        return Pair(mem, slice)
    }

    @ExperimentalUnsignedTypes
    fun native(uIntArray: UIntArray): Pair<Memory, Slice> {
        val intArray = uIntArray.asIntArray()
        val mem = Memory(Int.SIZE_BYTES * intArray.size.toLong())
        val ptr = mem.share(0)
        ptr.write(0, intArray, 0, intArray.size)
        val slice = Slice()
        slice.data = ptr
        slice.len = uIntArray.size.toLong()
        return Pair(mem, slice)
    }

    fun native(longArray: LongArray): Pair<Memory, Slice> {
        val mem = Memory(Long.SIZE_BYTES * longArray.size.toLong())
        val ptr = mem.share(0)
        ptr.write(0, longArray, 0, longArray.size)
        val slice = Slice()
        slice.data = ptr
        slice.len = longArray.size.toLong()
        return Pair(mem, slice)
    }

    @ExperimentalUnsignedTypes
    fun native(uLongArray: ULongArray): Pair<Memory, Slice> {
        val shortArray = uLongArray.asLongArray()
        val mem = Memory(Short.SIZE_BYTES * shortArray.size.toLong())
        val ptr = mem.share(0)
        ptr.write(0, shortArray, 0, shortArray.size)
        val slice = Slice()
        slice.data = ptr
        slice.len = uLongArray.size.toLong()
        return Pair(mem, slice)
    }

    fun native(floatArray: FloatArray): Pair<Memory, Slice> {
        val mem = Memory(Float.SIZE_BYTES * floatArray.size.toLong())
        val ptr = mem.share(0)
        ptr.write(0, floatArray, 0, floatArray.size)
        val slice = Slice()
        slice.data = ptr
        slice.len = floatArray.size.toLong()
        return Pair(mem, slice)
    }

    fun native(doubleArray: DoubleArray): Pair<Memory, Slice> {
        val mem = Memory(Double.SIZE_BYTES * doubleArray.size.toLong())
        val ptr = mem.share(0)
        ptr.write(0, doubleArray, 0, doubleArray.size)
        val slice = Slice()
        slice.data = ptr
        slice.len = doubleArray.size.toLong()
        return Pair(mem, slice)
    }

    fun getByteArray(slice: Slice): ByteArray {
        return slice.data.getByteArray(0, slice.len.toInt())
    }

    @ExperimentalUnsignedTypes
    fun getUByteArray(slice: Slice): UByteArray {
        return slice.data.getByteArray(0, slice.len.toInt()).asUByteArray()
    }

    fun getIntArray(slice: Slice): IntArray {
        return slice.data.getIntArray(0, slice.len.toInt())
    }

    @ExperimentalUnsignedTypes
    fun getUIntArray(slice: Slice): UIntArray {
        return slice.data.getIntArray(0, slice.len.toInt()).asUIntArray()
    }

    fun getShortArray(slice: Slice): ShortArray {
        return slice.data.getShortArray(0, slice.len.toInt())
    }

    @ExperimentalUnsignedTypes
    fun getUShortArray(slice: Slice): UShortArray {
        return slice.data.getShortArray(0, slice.len.toInt()).asUShortArray()
    }

    fun getLongArray(slice: Slice): LongArray {
        return slice.data.getLongArray(0, slice.len.toInt())
    }

    @ExperimentalUnsignedTypes
    fun getULongArray(slice: Slice): ULongArray {
        return slice.data.getLongArray(0, slice.len.toInt()).asULongArray()
    }

    fun getFloatArray(slice: Slice): FloatArray {
        return slice.data.getFloatArray(0, slice.len.toInt())
    }

    fun getDoubleArray(slice: Slice): DoubleArray {
        return slice.data.getDoubleArray(0, slice.len.toInt())
    }

    fun readUtf8(str: String): Pair<Memory, Slice> {
        return native(str.toByteArray())
    }

    @ExperimentalUnsignedTypes
    fun readUtf16(str: String): Pair<Memory, Slice> {
        return native(str.map { it.code.toUShort() }.toUShortArray())
    }

    fun getUtf8(slice: Slice): String {
        val byteArray = slice.data.getByteArray(0, slice.len.toInt())

        return byteArray.decodeToString()
    }

    fun getUtf16(slice: Slice): String {
        val shortArray = slice.data.getShortArray(0, slice.len.toInt())
        val charArray = shortArray.map { it.toInt().toChar() }.joinToString("")

        return charArray
    }

    fun readUtf8s(array: Array<String>): Pair<List<Memory>, Slice> {
        val sliceSize = Slice.SIZE
        val mem = Memory(sliceSize * array.size.toLong())
        val ptr = mem.share(0)
        val mems: List<Memory> =
                array.zip(0..array.size.toLong()).map { (str, idx) ->
                    val (mem, slice) = readUtf8(str)
                    ptr.setPointer(idx * sliceSize, slice.data)
                    ptr.setLong(idx * sliceSize + Long.SIZE_BYTES, slice.len)
                    mem
                }
        val slice = Slice()
        slice.data = ptr
        slice.len = array.size.toLong()
        return Pair(mems + mem, slice)
    }

    fun readUtf16s(array: Array<String>): Pair<List<Memory>, Slice> {
        val sliceSize = Slice.SIZE
        val mem = Memory(sliceSize * array.size.toLong())
        val ptr = mem.share(0)
        val mems: List<Memory> =
                array.zip(0..array.size.toLong()).map { (str, idx) ->
                    val (mem, slice) = readUtf16(str)
                    ptr.setPointer(idx * sliceSize, slice.data)
                    ptr.setLong(idx * sliceSize + Long.SIZE_BYTES, slice.len)
                    mem
                }
        val slice = Slice()
        slice.data = ptr
        slice.len = array.size.toLong()
        return Pair(mems + mem, slice)
    }

    fun getUtf16s(slice: Slice): List<String> {
        return (0..slice.len).map { idx ->
            val thisSlice = Slice()
            val thisPtr = Pointer(slice.data.getLong(idx * Slice.SIZE))
            val thisLen = slice.data.getLong(idx * Slice.SIZE + Long.SIZE_BYTES)
            thisSlice.data = thisPtr
            thisSlice.len = thisLen
            getUtf16(thisSlice)
        }
    }

    fun getUtf8s(slice: Slice): List<String> {
        return (0..slice.len).map { idx ->
            val thisSlice = Slice()
            val thisPtr = Pointer(slice.data.getLong(idx * Slice.SIZE))
            val thisLen = slice.data.getLong(idx * Slice.SIZE + Long.SIZE_BYTES)
            thisSlice.data = thisPtr
            thisSlice.len = thisLen
            getUtf8(thisSlice)
        }
    }
}

class Slice : Structure(), Structure.ByValue {

    @JvmField var data: Pointer = Pointer(0) // Pointer to const char
    @JvmField var len: Long = 0 // size_t

    // Define the fields of the struct
    override fun getFieldOrder(): List<String> {
        return listOf("data", "len")
    }

    companion object {
        var SIZE: Long = Native.getNativeSize(Slice::class.java).toLong()
    }
}

sealed interface Res<T, E>

class Ok<T, E>(val inner: T) : Res<T, E>

class Err<T, E>(val inner: E) : Res<T, E>

fun <T> Res<T, Throwable>.reThrow(): T {
    return when (this) {
        is Ok -> this.inner
        is Err -> throw this.inner
    }
}

fun <T, E> Res<T, E>.wrapErrAndThrow(): T {
    return when (this) {
        is Ok -> this.inner
        is Err -> throw RuntimeException("Received error ${this.inner}")
    }
}

fun <T, E> T.ok(): Res<T, E> {
    return Ok(this)
}

fun <T, E> E.err(): Res<T, E> {
    return Err(this)
}

class ResultIntUnitUnion : Union() {
    @JvmField var ok: Int = 0
}

class ResultIntUnit : Structure(), Structure.ByValue {
    @JvmField var union: ResultIntUnitUnion = ResultIntUnitUnion()

    @JvmField var isOk: Byte = 0

    // Define the fields of the struct
    override fun getFieldOrder(): List<String> {
        return listOf("union", "isOk")
    }
}

class ResultIntPointerUnion : Union() {
    @JvmField var ok: Int = ErrorEnum.default().toNative()

    @JvmField var err: Pointer = Pointer(0)
}

class ResultIntPointer : Structure(), Structure.ByValue {
    @JvmField var union: ResultIntPointerUnion = ResultIntPointerUnion()

    @JvmField var isOk: Byte = 0

    // Define the fields of the struct
    override fun getFieldOrder(): List<String> {
        return listOf("union", "isOk")
    }
}

class ResultPointerErrorStructNativeUnion : Union() {
    @JvmField var ok: Pointer = Pointer(0)

    @JvmField var err: ErrorStructNative = ErrorStructNative()
}

class ResultPointerErrorStructNative : Structure(), Structure.ByValue {
    @JvmField var union: ResultPointerErrorStructNativeUnion = ResultPointerErrorStructNativeUnion()

    @JvmField var isOk: Byte = 0

    // Define the fields of the struct
    override fun getFieldOrder(): List<String> {
        return listOf("union", "isOk")
    }
}

class ResultPointerIntUnion : Union() {
    @JvmField var ok: Pointer = Pointer(0)

    @JvmField var err: Int = ErrorEnum.default().toNative()
}

class ResultPointerInt : Structure(), Structure.ByValue {
    @JvmField var union: ResultPointerIntUnion = ResultPointerIntUnion()

    @JvmField var isOk: Byte = 0

    // Define the fields of the struct
    override fun getFieldOrder(): List<String> {
        return listOf("union", "isOk")
    }
}

class ResultPointerUnitUnion : Union() {
    @JvmField var ok: Pointer = Pointer(0)
}

class ResultPointerUnit : Structure(), Structure.ByValue {
    @JvmField var union: ResultPointerUnitUnion = ResultPointerUnitUnion()

    @JvmField var isOk: Byte = 0

    // Define the fields of the struct
    override fun getFieldOrder(): List<String> {
        return listOf("union", "isOk")
    }
}

class ResultUnitPointerUnion : Union() {
    @JvmField var err: Pointer = Pointer(0)
}

class ResultUnitPointer : Structure(), Structure.ByValue {
    @JvmField var union: ResultUnitPointerUnion = ResultUnitPointerUnion()

    @JvmField var isOk: Byte = 0

    // Define the fields of the struct
    override fun getFieldOrder(): List<String> {
        return listOf("union", "isOk")
    }
}
