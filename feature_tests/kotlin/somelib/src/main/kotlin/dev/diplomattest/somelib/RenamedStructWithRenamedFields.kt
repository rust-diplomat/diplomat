package dev.diplomattest.somelib

import com.sun.jna.Callback
import com.sun.jna.Library
import com.sun.jna.Native
import com.sun.jna.Pointer
import com.sun.jna.Structure

internal interface RenamedStructWithRenamedFieldsLib: Library {
    fun namespace_StructWithRenamedFields_with_opt_inner(nativeStruct: RenamedStructWithRenamedFieldsNative, optInner: OptionRenamedStructWithAttrsNative): RenamedStructWithRenamedFieldsNative
}

internal class RenamedStructWithRenamedFieldsNative: Structure(), Structure.ByValue {
    @JvmField
    internal var inner: RenamedStructWithAttrsNative = RenamedStructWithAttrsNative();
    @JvmField
    internal var optInner: OptionRenamedStructWithAttrsNative = OptionRenamedStructWithAttrsNative.none();
    @JvmField
    internal var attrEnum: Int = RenamedAttrEnum.default().toNative();
    @JvmField
    internal var optAttrEnum: OptionInt = OptionInt.none();

    // Define the fields of the struct
    override fun getFieldOrder(): List<String> {
        return listOf("inner", "optInner", "attrEnum", "optAttrEnum")
    }
}




internal class OptionRenamedStructWithRenamedFieldsNative constructor(): Structure(), Structure.ByValue {
    @JvmField
    internal var value: RenamedStructWithRenamedFieldsNative = RenamedStructWithRenamedFieldsNative()

    @JvmField
    internal var isOk: Byte = 0

    // Define the fields of the struct
    override fun getFieldOrder(): List<String> {
        return listOf("value", "isOk")
    }

    internal fun option(): RenamedStructWithRenamedFieldsNative? {
        if (isOk == 1.toByte()) {
            return value
        } else {
            return null
        }
    }


    constructor(value: RenamedStructWithRenamedFieldsNative, isOk: Byte): this() {
        this.value = value
        this.isOk = isOk
    }

    companion object {
        internal fun some(value: RenamedStructWithRenamedFieldsNative): OptionRenamedStructWithRenamedFieldsNative {
            return OptionRenamedStructWithRenamedFieldsNative(value, 1)
        }

        internal fun none(): OptionRenamedStructWithRenamedFieldsNative {
            return OptionRenamedStructWithRenamedFieldsNative(RenamedStructWithRenamedFieldsNative(), 0)
        }
    }

}

class RenamedStructWithRenamedFields (var inner: RenamedStructWithAttrs, var optInner: RenamedStructWithAttrs?, var attrEnum: RenamedAttrEnum, var optAttrEnum: RenamedAttrEnum?) {
    companion object {

        internal val libClass: Class<RenamedStructWithRenamedFieldsLib> = RenamedStructWithRenamedFieldsLib::class.java
        internal val lib: RenamedStructWithRenamedFieldsLib = Native.load("diplomat_feature_tests", libClass)
        val NATIVESIZE: Long = Native.getNativeSize(RenamedStructWithRenamedFieldsNative::class.java).toLong()

        internal fun fromNative(nativeStruct: RenamedStructWithRenamedFieldsNative): RenamedStructWithRenamedFields {
            val inner: RenamedStructWithAttrs = RenamedStructWithAttrs.fromNative(nativeStruct.inner)
            val optInner: RenamedStructWithAttrs? = nativeStruct.optInner.option()?.let { RenamedStructWithAttrs.fromNative(it) }
            val attrEnum: RenamedAttrEnum = RenamedAttrEnum.fromNative(nativeStruct.attrEnum)
            val optAttrEnum: RenamedAttrEnum? = nativeStruct.optAttrEnum.option()?.let { RenamedAttrEnum.fromNative(it) }

            return RenamedStructWithRenamedFields(inner, optInner, attrEnum, optAttrEnum)
        }

    }
    internal fun toNative(): RenamedStructWithRenamedFieldsNative {
        var native = RenamedStructWithRenamedFieldsNative()
        native.inner = this.inner.toNative()
        native.optInner = this.optInner?.let { OptionRenamedStructWithAttrsNative.some(it.toNative()) } ?: OptionRenamedStructWithAttrsNative.none()
        native.attrEnum = this.attrEnum.toNative()
        native.optAttrEnum = this.optAttrEnum?.let { OptionInt.some(it.toNative()) } ?: OptionInt.none()
        return native
    }

    
    fun withOptInner(optInner: RenamedStructWithAttrs?): RenamedStructWithRenamedFields {
        
        val returnVal = lib.namespace_StructWithRenamedFields_with_opt_inner(this.toNative(), optInner?.let { OptionRenamedStructWithAttrsNative.some(it.toNative()) } ?: OptionRenamedStructWithAttrsNative.none());
        val returnStruct = RenamedStructWithRenamedFields.fromNative(returnVal)
        return returnStruct
    }
}