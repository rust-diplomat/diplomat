package dev.diplomattest.somelib;
import com.sun.jna.Callback
import com.sun.jna.Library
import com.sun.jna.Native
import com.sun.jna.Pointer
import com.sun.jna.Structure

internal interface OptionOpaqueLib: Library {
    fun OptionOpaque_destroy(handle: Pointer)
    fun OptionOpaque_new(i: Int): Pointer?
    fun OptionOpaque_new_none(): Pointer?
    fun OptionOpaque_returns(): OptionOptionStructNative
    fun OptionOpaque_option_isize(handle: Pointer): OptionFFIIsizet
    fun OptionOpaque_option_usize(handle: Pointer): OptionFFISizet
    fun OptionOpaque_option_i32(handle: Pointer): OptionInt
    fun OptionOpaque_option_u32(handle: Pointer): OptionFFIUint32
    fun OptionOpaque_new_struct(): OptionStructNative
    fun OptionOpaque_new_struct_nones(): OptionStructNative
    fun OptionOpaque_returns_none_self(handle: Pointer): Pointer?
    fun OptionOpaque_returns_some_self(handle: Pointer): Pointer?
    fun OptionOpaque_assert_integer(handle: Pointer, i: Int): Unit
    fun OptionOpaque_option_opaque_argument(arg: Pointer?): Byte
    fun OptionOpaque_accepts_option_u8(arg: OptionFFIUint8, sentinel: FFIUint8): OptionFFIUint8
    fun OptionOpaque_accepts_option_enum(arg: OptionInt, sentinel: FFIUint8): OptionInt
    fun OptionOpaque_accepts_borrowing_option_struct(arg: BorrowingOptionStructNative): Unit
    fun OptionOpaque_accepts_multiple_option_enum(sentinel1: FFIUint8, arg1: OptionInt, arg2: OptionInt, arg3: OptionInt, sentinel2: FFIUint8): OptionInt
    fun OptionOpaque_accepts_option_input_struct(arg: OptionOptionInputStructNative, sentinel: FFIUint8): OptionOptionInputStructNative
    fun OptionOpaque_returns_option_input_struct(): OptionInputStructNative
}

class OptionOpaque internal constructor (
    internal val handle: Pointer,
    // These ensure that anything that is borrowed is kept alive and not cleaned
    // up by the garbage collector.
    internal val selfEdges: List<Any>,
)  {

    internal class OptionOpaqueCleaner(val handle: Pointer, val lib: OptionOpaqueLib) : Runnable {
        override fun run() {
            lib.OptionOpaque_destroy(handle)
        }
    }

    companion object {
        internal val libClass: Class<OptionOpaqueLib> = OptionOpaqueLib::class.java
        internal val lib: OptionOpaqueLib = Native.load("diplomat_feature_tests", libClass)
        @JvmStatic
        
        fun new_(i: Int): OptionOpaque? {
            
            val returnVal = lib.OptionOpaque_new(i);
            val selfEdges: List<Any> = listOf()
            val handle = returnVal ?: return null
            val returnOpaque = OptionOpaque(handle, selfEdges)
            CLEANER.register(returnOpaque, OptionOpaque.OptionOpaqueCleaner(handle, OptionOpaque.lib));
            return returnOpaque
        }
        @JvmStatic
        
        fun newNone(): OptionOpaque? {
            
            val returnVal = lib.OptionOpaque_new_none();
            val selfEdges: List<Any> = listOf()
            val handle = returnVal ?: return null
            val returnOpaque = OptionOpaque(handle, selfEdges)
            CLEANER.register(returnOpaque, OptionOpaque.OptionOpaqueCleaner(handle, OptionOpaque.lib));
            return returnOpaque
        }
        @JvmStatic
        
        fun returns(): OptionStruct? {
            
            val returnVal = lib.OptionOpaque_returns();
            
            val intermediateOption = returnVal.option() ?: return null
            val returnStruct = OptionStruct.fromNative(intermediateOption)
            return returnStruct
                                    
        }
        @JvmStatic
        
        fun newStruct(): OptionStruct {
            
            val returnVal = lib.OptionOpaque_new_struct();
            val returnStruct = OptionStruct.fromNative(returnVal)
            return returnStruct
        }
        @JvmStatic
        
        fun newStructNones(): OptionStruct {
            
            val returnVal = lib.OptionOpaque_new_struct_nones();
            val returnStruct = OptionStruct.fromNative(returnVal)
            return returnStruct
        }
        @JvmStatic
        
        fun optionOpaqueArgument(arg: OptionOpaque?): Boolean {
            
            val returnVal = lib.OptionOpaque_option_opaque_argument(arg?.handle);
            return (returnVal > 0)
        }
        @JvmStatic
        
        fun acceptsOptionU8(arg: UByte?, sentinel: UByte): UByte? {
            
            val returnVal = lib.OptionOpaque_accepts_option_u8(arg?.let { OptionFFIUint8.some(FFIUint8(it)) } ?: OptionFFIUint8.none(), FFIUint8(sentinel));
            return returnVal.option()?.toUByte()
        }
        @JvmStatic
        
        fun acceptsOptionEnum(arg: OptionEnum?, sentinel: UByte): OptionEnum? {
            
            val returnVal = lib.OptionOpaque_accepts_option_enum(arg?.let { OptionInt.some(it.toNative()) } ?: OptionInt.none(), FFIUint8(sentinel));
            
            val intermediateOption = returnVal.option() ?: return null
            return OptionEnum.fromNative(intermediateOption)
        }
        @JvmStatic
        
        fun acceptsBorrowingOptionStruct(arg: BorrowingOptionStruct): Unit {
            val temporaryEdgeArena: MutableList<Any> = mutableListOf()
            
            val returnVal = lib.OptionOpaque_accepts_borrowing_option_struct(arg.toNative(aAppendArray = arrayOf(temporaryEdgeArena)));
            
        }
        @JvmStatic
        
        fun acceptsMultipleOptionEnum(sentinel1: UByte, arg1: OptionEnum?, arg2: OptionEnum?, arg3: OptionEnum?, sentinel2: UByte): OptionEnum? {
            
            val returnVal = lib.OptionOpaque_accepts_multiple_option_enum(FFIUint8(sentinel1), arg1?.let { OptionInt.some(it.toNative()) } ?: OptionInt.none(), arg2?.let { OptionInt.some(it.toNative()) } ?: OptionInt.none(), arg3?.let { OptionInt.some(it.toNative()) } ?: OptionInt.none(), FFIUint8(sentinel2));
            
            val intermediateOption = returnVal.option() ?: return null
            return OptionEnum.fromNative(intermediateOption)
        }
        @JvmStatic
        
        fun acceptsOptionInputStruct(arg: OptionInputStruct?, sentinel: UByte): OptionInputStruct? {
            
            val returnVal = lib.OptionOpaque_accepts_option_input_struct(arg?.let { OptionOptionInputStructNative.some(it.toNative()) } ?: OptionOptionInputStructNative.none(), FFIUint8(sentinel));
            
            val intermediateOption = returnVal.option() ?: return null
            val returnStruct = OptionInputStruct.fromNative(intermediateOption)
            return returnStruct
                                    
        }
        @JvmStatic
        
        fun returnsOptionInputStruct(): OptionInputStruct {
            
            val returnVal = lib.OptionOpaque_returns_option_input_struct();
            val returnStruct = OptionInputStruct.fromNative(returnVal)
            return returnStruct
        }
    }
    
    fun optionIsize(): Long? {
        
        val returnVal = lib.OptionOpaque_option_isize(handle);
        return returnVal.option()?.toLong()
    }
    
    fun optionUsize(): ULong? {
        
        val returnVal = lib.OptionOpaque_option_usize(handle);
        return returnVal.option()?.toULong()
    }
    
    fun optionI32(): Int? {
        
        val returnVal = lib.OptionOpaque_option_i32(handle);
        return returnVal.option()
    }
    
    fun optionU32(): UInt? {
        
        val returnVal = lib.OptionOpaque_option_u32(handle);
        return returnVal.option()?.toUInt()
    }
    
    fun returnsNoneSelf(): OptionOpaque? {
        // This lifetime edge depends on lifetimes: 'a
        val aEdges: MutableList<Any> = mutableListOf(this);
        
        val returnVal = lib.OptionOpaque_returns_none_self(handle);
        val selfEdges: List<Any> = listOf(this)
        val handle = returnVal ?: return null
        val returnOpaque = OptionOpaque(handle, selfEdges)
        return returnOpaque
    }
    
    fun returnsSomeSelf(): OptionOpaque? {
        // This lifetime edge depends on lifetimes: 'a
        val aEdges: MutableList<Any> = mutableListOf(this);
        
        val returnVal = lib.OptionOpaque_returns_some_self(handle);
        val selfEdges: List<Any> = listOf(this)
        val handle = returnVal ?: return null
        val returnOpaque = OptionOpaque(handle, selfEdges)
        return returnOpaque
    }
    
    fun assertInteger(i: Int): Unit {
        
        val returnVal = lib.OptionOpaque_assert_integer(handle, i);
        
    }

}