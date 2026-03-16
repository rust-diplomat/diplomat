package dev.diplomattest.somelib

import com.sun.jna.Callback
import com.sun.jna.Library
import com.sun.jna.Native
import com.sun.jna.Pointer
import com.sun.jna.Structure

/** Diplomat will prepend this whole block to the start of attrs.rs,
*but we currently cannot do the same for proc_macro (until we hit MSRV >= 1.88).
*So the workaround is to use the path to the module whenever referring to the imported type (as seen above).
*/
class RenamedNonMacroMixin () {
    companion object {

    }
}