package dev.gigapixel.somelib;
import com.sun.jna.Library
import com.sun.jna.Native
import com.sun.jna.Pointer


interface OtherOpaqueLib: Library {
    fun OtherOpaque_destroy(handle: Long)
    fun OtherOpaque_from_usize(i: Long): Long
    fun OtherOpaque_get_len_and_add(handle: Long, i: Long): Long
}

class OtherOpaque internal constructor (internal val handle: Long) {

    private class OtherOpaqueCleaner(val handle: Long, val lib: OtherOpaqueLib) : Runnable {
        override fun run() {
            lib.OtherOpaque_destroy(handle)
        }
    }

    companion object {
        val libClass: Class<OtherOpaqueLib> = OtherOpaqueLib::class.java
        val lib: OtherOpaqueLib = Native.load("somelib", libClass)

        fun fromUsize(i: Long): OtherOpaque {
            val returnVal = lib.OtherOpaque_from_usize(i);
            
            val handle = returnVal
            val returnOpaque = OtherOpaque(handle)
            CLEANER.register(returnOpaque, OtherOpaqueCleaner(returnOpaque.handle, OtherOpaque.lib));
            return returnOpaque
        }
    }

    fun getLenAndAdd(i: Long): Long {
        val returnVal = lib.OtherOpaque_get_len_and_add(handle, i);
        return returnVal
    }

}
