package dev.diplomattest.somelib;

import com.sun.jna.JNIEnv
import com.sun.jna.Library
import com.sun.jna.Memory
import com.sun.jna.Native
import com.sun.jna.Pointer
import com.sun.jna.Structure
import com.sun.jna.Union
import java.util.Collections


// We spawn a cleaner for the library which is responsible for cleaning opaque types.
val CLEANER = java.lang.ref.Cleaner.create()


interface DiplomatWriteLib: Library {
    fun diplomat_buffer_write_create(size: Long): Pointer 
    fun diplomat_buffer_write_get_bytes(diplomatWrite: Pointer): Pointer
    fun diplomat_buffer_write_len(diplomatWrite: Pointer): Long
    fun diplomat_buffer_write_destroy(diplomatWrite: Pointer)
}

object DW {

    val libClass: Class<DiplomatWriteLib> = DiplomatWriteLib::class.java
    val lib: DiplomatWriteLib = Native.load("somelib", libClass)

    fun writeToString (write: Pointer): String {
        try {
            val pointer = lib.diplomat_buffer_write_get_bytes(write)
            if (pointer == null) {
                throw OutOfMemoryError();
            }
            val len = lib.diplomat_buffer_write_len(write)
            val bytes = pointer.getByteArray(0, len.toInt())
            return bytes.decodeToString();
        } finally {
            lib.diplomat_buffer_write_destroy(write);
        }
    }
}

internal interface DiplomatJVMRuntimeLib: Library {
    fun create_rust_jvm_cookie(env: JNIEnv, obj: Object): Pointer
    fun destroy_rust_jvm_cookie(obj_pointer: Pointer): Unit
}

internal class DiplomatJVMRuntime {
    companion object {
        val libClass: Class<DiplomatJVMRuntimeLib> = DiplomatJVMRuntimeLib::class.java
        val lib: DiplomatJVMRuntimeLib = Native.load("somelib", libClass, Collections.singletonMap(Library.OPTION_ALLOW_OBJECTS, true))

        fun buildRustCookie(obj: Object): Pointer {
            return lib.create_rust_jvm_cookie(JNIEnv.CURRENT, obj);
        }

        fun dropRustCookie(obj_pointer: Pointer): Unit {
            lib.destroy_rust_jvm_cookie(obj_pointer);
        }
    }
}


internal object PrimitiveArrayTools {

    fun allocateMemory(size: Long): Memory? {
        // we can't use the Memory constructor for a memory of size 0
        // so, if the size is zero, then we return null
        if (size > 0L)
            return Memory(size)
        else
            return null
    }

    fun native(boolArray: BooleanArray): Pair<Memory?, Slice> {
        val mem = allocateMemory(boolArray.size.toLong())
        val byteArray = boolArray.map {if (it) 1.toByte() else 0.toByte() }.toByteArray()
        val slice = Slice()
        slice.data = if (mem != null) {
            val ptr = mem.share(0)
            ptr.write(0, byteArray, 0, byteArray.size)
            ptr
        } else {
            Pointer(0)
        }
        slice.len = FFISizet(byteArray.size.toLong().toULong())
        return Pair(mem, slice)
    }

    fun native(byteArray: ByteArray):  Pair<Memory?, Slice> {
        val mem = allocateMemory(byteArray.size.toLong())
        val slice = Slice()
        slice.data = if (mem != null) { 
            val ptr = mem.share(0)
            ptr.write(0, byteArray, 0, byteArray.size)
            ptr
        } else {
            Pointer(0)
        }
        slice.len = FFISizet(byteArray.size.toLong().toULong())
        return Pair(mem, slice)
    }

    @ExperimentalUnsignedTypes
    fun native(uByteArray: UByteArray): Pair<Memory?, Slice> {
        val byteArray = uByteArray.asByteArray()
        val mem = allocateMemory(byteArray.size.toLong())
        val slice = Slice()
        slice.data = if (mem != null) {
            val ptr = mem.share(0)
            ptr.write(0, byteArray, 0, byteArray.size)
            ptr
        } else {
            Pointer(0)
        }
        slice.len = FFISizet(uByteArray.size.toLong().toULong())
        return Pair(mem, slice)
    }

    fun native(shortArray: ShortArray): Pair<Memory?, Slice> {
        val mem = allocateMemory(Short.SIZE_BYTES * shortArray.size.toLong())
        val slice = Slice()
        slice.data = if (mem != null) {
            val ptr = mem.share(0)
            ptr.write(0, shortArray, 0, shortArray.size)
            ptr
        } else {
            Pointer(0)
        }
        slice.len = FFISizet(shortArray.size.toLong().toULong())
        return Pair(mem, slice)
    }

    @ExperimentalUnsignedTypes
    fun native(uShortArray: UShortArray): Pair<Memory?, Slice> {
        val shortArray = uShortArray.asShortArray()
        val mem = allocateMemory(Short.SIZE_BYTES * shortArray.size.toLong())
        val slice = Slice()
        slice.data = if (mem != null) {
            val ptr = mem.share(0)
            ptr.write(0, shortArray, 0, shortArray.size)
            ptr
        } else {
            Pointer(0)
        }
        slice.len = FFISizet(uShortArray.size.toLong().toULong())
        return Pair(mem, slice)
    }

    fun native(intArray: IntArray): Pair<Memory?, Slice> {
        val mem = allocateMemory(Int.SIZE_BYTES * intArray.size.toLong())
        val slice = Slice()
        slice.data = if (mem != null) {
            val ptr = mem.share(0)
            ptr.write(0, intArray, 0, intArray.size)
            ptr
        } else {
            Pointer(0)
        }
        slice.len = FFISizet(intArray.size.toLong().toULong())
        return Pair(mem, slice)
    }

    @ExperimentalUnsignedTypes
    fun native(uIntArray: UIntArray): Pair<Memory?, Slice> {
        val intArray = uIntArray.asIntArray()
        val mem = allocateMemory(Int.SIZE_BYTES * intArray.size.toLong())
        val slice = Slice()
        slice.data = if (mem != null) {
            val ptr = mem.share(0)
            ptr.write(0, intArray, 0, intArray.size)
            ptr
        } else {
            Pointer(0)
        }
        slice.len = FFISizet(uIntArray.size.toLong().toULong())
        return Pair(mem, slice)
    }


    fun native(longArray: LongArray): Pair<Memory?, Slice> {
        val mem = allocateMemory(Long.SIZE_BYTES * longArray.size.toLong())
        val slice = Slice()
        slice.data = if (mem != null) {
            val ptr = mem.share(0)
            ptr.write(0, longArray, 0, longArray.size)
            ptr
        } else {
            Pointer(0)
        }
        slice.len = FFISizet(longArray.size.toLong().toULong())
        return Pair(mem, slice)
    }

    @ExperimentalUnsignedTypes
    fun native(uLongArray: ULongArray): Pair<Memory?, Slice> {
        val shortArray = uLongArray.asLongArray()
        val mem = allocateMemory(Short.SIZE_BYTES * shortArray.size.toLong())
        val slice = Slice()
        slice.data = if (mem != null) {
            val ptr = mem.share(0)
            ptr.write(0, shortArray, 0, shortArray.size)
            ptr
        } else {
            Pointer(0)
        }
        slice.len = FFISizet(uLongArray.size.toLong().toULong())
        return Pair(mem, slice)
    }

    fun native(floatArray: FloatArray): Pair<Memory?, Slice> {
        val mem = allocateMemory(Float.SIZE_BYTES * floatArray.size.toLong())
        val slice = Slice()
        slice.data = if (mem != null) {
            val ptr = mem.share(0)
            ptr.write(0, floatArray, 0, floatArray.size)
            ptr
        } else {
            Pointer(0)
        }
        slice.len = FFISizet(floatArray.size.toLong().toULong())
        return Pair(mem, slice)
    }

    fun native(doubleArray: DoubleArray): Pair<Memory?, Slice> {
        val mem = allocateMemory(Double.SIZE_BYTES * doubleArray.size.toLong())
        val slice = Slice()
        slice.data = if (mem != null) {
            val ptr = mem.share(0)
            ptr.write(0, doubleArray, 0, doubleArray.size)
            ptr
        } else {
            Pointer(0)
        }
        slice.len = FFISizet(doubleArray.size.toLong().toULong())
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

    fun getShortArray(slice: Slice): ShortArray{
        return slice.data.getShortArray(0, slice.len.toInt())
    }

    @ExperimentalUnsignedTypes
    fun getUShortArray(slice: Slice): UShortArray{
        return slice.data.getShortArray(0, slice.len.toInt()).asUShortArray()
    }

    fun getLongArray (slice: Slice): LongArray {
        return slice.data.getLongArray(0, slice.len.toInt())
    }

    @ExperimentalUnsignedTypes
    fun getULongArray (slice: Slice): ULongArray {
        return slice.data.getLongArray(0, slice.len.toInt()).asULongArray()
    }

    fun getFloatArray (slice: Slice): FloatArray {
        return slice.data.getFloatArray(0, slice.len.toInt())
    }

    fun getDoubleArray (slice: Slice): DoubleArray {
        return slice.data.getDoubleArray(0, slice.len.toInt())
    }

    fun readUtf8(str: String): Pair<Memory?, Slice> {
        return native(str.toByteArray())
    }

    @ExperimentalUnsignedTypes
    fun readUtf16(str: String): Pair<Memory?, Slice> {
        return native(str.map {it.code.toUShort()}.toUShortArray())
    }

    fun getUtf8(slice: Slice): String {
        val byteArray = slice.data.getByteArray(0, slice.len.toInt())

        return byteArray.decodeToString()
    }

    fun getUtf16(slice: Slice): String {
        val shortArray = slice.data.getShortArray(0, slice.len.toInt())
        val charArray = shortArray.map { it.toInt().toChar() }.joinToString(  "")

        return charArray
    }

    fun readUtf8s(array: Array<String>): Pair<List<Memory?>, Slice> {
        val sliceSize = Slice.SIZE
        val mem = allocateMemory(sliceSize * array.size.toLong())
        val ptr = if (mem != null) {
            mem.share(0)
        } else {
            Pointer(0)
        }
        val mems: List<Memory?> = array.zip(0..array.size.toLong()).map { (str, idx) ->
            val (mem, slice) = readUtf8(str)
            ptr.setPointer(idx * sliceSize, slice.data)
            ptr.setLong(idx * sliceSize + Long.SIZE_BYTES, slice.len.toLong())
            mem
        }
        val slice = Slice()
        slice.data = ptr
        slice.len = FFISizet(array.size.toLong().toULong())
        return Pair(mems + mem, slice)
    }

    fun readUtf16s(array: Array<String>): Pair<List<Memory?>, Slice> {
        val sliceSize = Slice.SIZE
        val mem = allocateMemory(sliceSize * array.size.toLong())
        val ptr = if (mem != null) {
            mem.share(0)
        } else {
            Pointer(0)
        }
        val mems: List<Memory?> = array.zip(0..array.size.toLong()).map { (str, idx) ->
            val (mem, slice) = readUtf16(str)
            ptr.setPointer(idx * sliceSize, slice.data)
            ptr.setLong(idx * sliceSize + Long.SIZE_BYTES, slice.len.toLong())
            mem
        }
        val slice = Slice()
        slice.data = ptr
        slice.len = FFISizet(array.size.toLong().toULong())
        return Pair(mems + mem, slice)
    }

    fun getUtf16s(slice: Slice): List<String> {
        return (0..slice.len.toInt()).map { idx ->
            val thisSlice = Slice()
            val thisPtr = Pointer(slice.data.getLong(idx * Slice.SIZE))
            val thisLen = slice.data.getLong(idx * Slice.SIZE + Long.SIZE_BYTES)
            thisSlice.data = thisPtr
            thisSlice.len = FFISizet(thisLen.toULong())
            getUtf16(thisSlice)
        }
    }

    fun getUtf8s(slice: Slice): List<String> {
        return (0..slice.len.toInt()).map { idx ->
            val thisSlice = Slice()
            val thisPtr = Pointer(slice.data.getLong(idx * Slice.SIZE))
            val thisLen = slice.data.getLong(idx * Slice.SIZE + Long.SIZE_BYTES)
            thisSlice.data = thisPtr
            thisSlice.len = FFISizet(thisLen.toULong())
            getUtf8(thisSlice)
        }
    }
}

class FFISizet(val value: ULong = 0u): com.sun.jna.IntegerType(Native.SIZE_T_SIZE, value.toLong(), true)  {
    override fun toByte(): Byte = this.toLong().toByte()
    override fun toChar(): Char = this.toLong().toInt().toChar()
    override fun toShort(): Short = this.toLong().toShort()
    fun toULong(): ULong = this.toLong().toULong()
    constructor(): this(0u)
}

class FFIIsizet(val value: Long = 0): com.sun.jna.IntegerType(Native.SIZE_T_SIZE, value, true)  {
    override fun toByte(): Byte = this.toLong().toByte()
    override fun toChar(): Char = this.toLong().toInt().toChar()
    override fun toShort(): Short = this.toLong().toShort()
}

class FFIUint8(val value: UByte = 0u): com.sun.jna.IntegerType(1, value.toByte().toLong(), true)  {
    override fun toByte(): Byte = this.toLong().toByte()
    override fun toChar(): Char = this.toLong().toInt().toChar()
    override fun toShort(): Short = this.toLong().toShort()
    fun toUByte(): UByte = this.toByte().toUByte()
    constructor(): this(0u)
}

class FFIUint16(val value: UShort = 0u): com.sun.jna.IntegerType(2, value.toShort().toLong(), true)  {
    override fun toByte(): Byte = this.toLong().toByte()
    override fun toChar(): Char = this.toLong().toInt().toChar()
    override fun toShort(): Short = this.toLong().toShort()
    fun toUShort(): UShort = this.toShort().toUShort()
    constructor(): this(0u)
}

class FFIUint32(val value: UInt = 0u): com.sun.jna.IntegerType(4, value.toInt().toLong(), true)  {
    override fun toByte(): Byte = this.toLong().toByte()
    override fun toChar(): Char = this.toLong().toInt().toChar()
    override fun toShort(): Short = this.toLong().toShort()
    fun toUInt(): UInt = this.toInt().toUInt()
    constructor(): this(0u)
}

class FFIUint64(val value: ULong = 0u): com.sun.jna.IntegerType(8, value.toLong(), true)  {
    override fun toByte(): Byte = this.toLong().toByte()
    override fun toChar(): Char = this.toLong().toInt().toChar()
    override fun toShort(): Short = this.toLong().toShort()
    fun toULong(): ULong = this.toLong().toULong()
    constructor(): this(0u)
}

class Slice: Structure(), Structure.ByValue {

    @JvmField var data: Pointer = Pointer(0)// Pointer to const char
    @JvmField var len: FFISizet = FFISizet() // FFISizet of 0

    // Define the fields of the struct
    override fun getFieldOrder(): List<String> {
        return listOf("data", "len")
    }

    companion object {
        var SIZE: Long = Native.getNativeSize(Slice::class.java).toLong()
    }
}


internal fun <T> T.ok(): Result<T> {
    return Result.success(this)
}

internal fun <T> Throwable.err(): Result<T> {
    return Result.failure(this)
}

class UByteError internal constructor(internal val value: UByte): Exception("Rust error result for UByte") {
    override fun toString(): String {
        return "UByte error with value " + value
    }

    fun getValue(): UByte = value
}

class ByteError internal constructor(internal val value: Byte): Exception("Rust error result for Byte") {
    override fun toString(): String {
        return "Byte error with value " + value
    }

    fun getValue(): Byte = value
}

class UShortError internal constructor(internal val value: UShort): Exception("Rust error result for UShort") {
    override fun toString(): String {
        return "UShort error with value " + value
    }

    fun getValue(): UShort = value
}

class ShortError internal constructor(internal val value: Short): Exception("Rust error result for Short") {
    override fun toString(): String {
        return "Short error with value " + value
    }

    fun getValue(): Short = value
}

class UIntError internal constructor(internal val value: UInt): Exception("Rust error result for UInt") {
    override fun toString(): String {
        return "UInt error with value " + value
    }

    fun getValue(): UInt = value
}

class IntError internal constructor(internal val value: Int): Exception("Rust error result for Int") {
    override fun toString(): String {
        return "Int error with value " + value
    }

    fun getValue(): Int = value
}

class ULongError internal constructor(internal val value: ULong): Exception("Rust error result for ULong") {
    override fun toString(): String {
        return "ULong error with value " + value
    }

    fun getValue(): ULong = value
}

class LongError internal constructor(internal val value: Long): Exception("Rust error result for Long") {
    override fun toString(): String {
        return "Long error with value " + value
    }

    fun getValue(): Long = value
}

class FloatError internal constructor(internal val value: Float): Exception("Rust error result for Float") {
    override fun toString(): String {
        return "Float error with value " + value
    }

    fun getValue(): Float = value
}

class DoubleError internal constructor(internal val value: Double): Exception("Rust error result for Double") {
    override fun toString(): String {
        return "Double error with value " + value
    }

    fun getValue(): Double = value
}

class CharError internal constructor(internal val value: Char): Exception("Rust error result for Char") {
    override fun toString(): String {
        return "Char error with value " + value
    }

    fun getValue(): Char = value
}

class BooleanError internal constructor(internal val value: Boolean): Exception("Rust error result for Boolean") {
    override fun toString(): String {
        return "Boolean error with value " + value
    }

    fun getValue(): Boolean = value
}

class UnitError internal constructor(): Exception("Rust error result for Unit") {
    override fun toString(): String {
        return "Unit error"
    }
}
           
internal class ResultPointerUnitUnion: Union() {
    @JvmField
    internal var ok: Pointer = Pointer(0)
}

class ResultPointerUnit: Structure(), Structure.ByValue  {
    @JvmField
    internal var union: ResultPointerUnitUnion = ResultPointerUnitUnion()

    @JvmField
    internal var isOk: Byte = 0

    // Define the fields of the struct
    override fun getFieldOrder(): List<String> {
        return listOf("union", "isOk")
    }
}
internal class ResultUnitUnitUnion: Union() {
}

class ResultUnitUnit: Structure(), Structure.ByValue  {
    @JvmField
    internal var union: ResultUnitUnitUnion = ResultUnitUnitUnion()

    @JvmField
    internal var isOk: Byte = 0

    // Define the fields of the struct
    override fun getFieldOrder(): List<String> {
        return listOf("union", "isOk")
    }
}



