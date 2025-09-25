package dev.diplomattest.somelib

import com.sun.jna.Callback
import com.sun.jna.Library
import com.sun.jna.Native
import com.sun.jna.Pointer
import com.sun.jna.Structure

internal interface TraitWrapperLib: Library {
    fun TraitWrapper_test_with_trait(t: DiplomatTrait_TesterTrait_Wrapper_Native, x: Int): Int
    fun TraitWrapper_test_trait_with_struct(t: DiplomatTrait_TesterTrait_Wrapper_Native): Int
}

internal class TraitWrapperNative: Structure(), Structure.ByValue {
    @JvmField
    internal var cantBeEmpty: Byte = 0;

    // Define the fields of the struct
    override fun getFieldOrder(): List<String> {
        return listOf("cantBeEmpty")
    }
}

class TraitWrapper internal constructor (
    internal val nativeStruct: TraitWrapperNative) {
    val cantBeEmpty: Boolean = nativeStruct.cantBeEmpty > 0

    companion object {
        internal val libClass: Class<TraitWrapperLib> = TraitWrapperLib::class.java
        internal val lib: TraitWrapperLib = Native.load("somelib", libClass)
        val NATIVESIZE: Long = Native.getNativeSize(TraitWrapperNative::class.java).toLong()
        @JvmStatic
        
        fun testWithTrait(t: TesterTrait, x: Int): Int {
            
            val returnVal = lib.TraitWrapper_test_with_trait(DiplomatTrait_TesterTrait_Wrapper.fromTraitObj(t).nativeStruct, x);
            return (returnVal)
        }
        @JvmStatic
        
        fun testTraitWithStruct(t: TesterTrait): Int {
            
            val returnVal = lib.TraitWrapper_test_trait_with_struct(DiplomatTrait_TesterTrait_Wrapper.fromTraitObj(t).nativeStruct);
            return (returnVal)
        }
    }

}
