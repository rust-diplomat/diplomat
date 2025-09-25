package dev.diplomattest.somelib

import com.sun.jna.Callback
import com.sun.jna.Library
import com.sun.jna.Native
import com.sun.jna.Pointer
import com.sun.jna.Structure

class MyZst internal constructor (
    ): Exception("Rust error result for MyZst") {

    }
