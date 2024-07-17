package dev.diplomattest.somelib;

import dev.diplomattest.somelib.ntv.*;


import java.lang.foreign.Arena;
import java.lang.foreign.MemorySegment;
import java.lang.ref.Cleaner;
import static java.lang.foreign.ValueLayout.*;
import java.nio.charset.StandardCharsets;

public class MyString {

    MemorySegment internal;
    Cleaner.Cleanable cleanable;

    static class MyStringCleaner implements Runnable {

        MemorySegment segment;
        MyStringCleaner(MemorySegment segment) {
            this.segment = segment;
        }

        public void run() {
            somelib_h.MyString_destroy(this.segment);
        }
    }

    MyString() {}

    public MyString(String v) {
        try (var arena = Arena.ofConfined()) {
            var vMemSeg = arena.allocateFrom(v, StandardCharsets.UTF_8);
            var vLen = vMemSeg.byteSize();
            var nativeVal = somelib_h.MyString_new(vMemSeg, vLen - 1);
            this.internal = nativeVal;
            var cleaner = new MyString.MyStringCleaner(nativeVal);
            this.cleanable = Lib.cleaner.register(this, cleaner);
        }
    }

    public static MyString newUnsafe(String v) {
        try (var arena = Arena.ofConfined()) {
            var vMemSeg = arena.allocateFrom(v, StandardCharsets.UTF_8);
            var vLen = vMemSeg.byteSize();
            var nativeVal = somelib_h.MyString_new_unsafe(vMemSeg, vLen - 1);
            var returnVal = new MyString();
            returnVal.internal = nativeVal;
            var cleaner = new MyString.MyStringCleaner(nativeVal);
            returnVal.cleanable = Lib.cleaner.register(returnVal, cleaner);
            return returnVal;
        }
    }

    public static MyString newOwned(String v) {
        var vMemSeg = Arena.global().allocateFrom(v, StandardCharsets.UTF_8);
        var vLen = vMemSeg.byteSize();
        var nativeVal = somelib_h.MyString_new_owned(vMemSeg, vLen - 1);
        var returnVal = new MyString();
        returnVal.internal = nativeVal;
        var cleaner = new MyString.MyStringCleaner(nativeVal);
        returnVal.cleanable = Lib.cleaner.register(returnVal, cleaner);
        return returnVal;
    }

    public static MyString newFromFirst(String [] v) {
        try (var arena = Arena.ofConfined()) {
            var vData = SliceUtils.strs8(arena, v);
            var vLen = v.length;
            var nativeVal = somelib_h.MyString_new_from_first(vData, vLen);
            var returnVal = new MyString();
            returnVal.internal = nativeVal;
            var cleaner = new MyString.MyStringCleaner(nativeVal);
            returnVal.cleanable = Lib.cleaner.register(returnVal, cleaner);
            return returnVal;
        }
    }


    public void setStr(String newStr) {
        try (var arena = Arena.ofConfined()) {
            var newStrMemSeg = arena.allocateFrom(newStr, StandardCharsets.UTF_8);
            var newStrLen = newStrMemSeg.byteSize();
            somelib_h.MyString_set_str(internal, newStrMemSeg, newStrLen - 1);

        }
    }

    public String getStr() {

        var writeable = somelib_h.diplomat_buffer_write_create(0);
        somelib_h.MyString_get_str(internal, writeable);
        var buffer = DiplomatWrite.buf(writeable);
        var string = buffer.getString(0, StandardCharsets.UTF_8);
        somelib_h.diplomat_buffer_write_destroy(writeable);
        return string;
    }

    public String getBoxedStr() {
        var boxArena = Arena.ofConfined();

        var nativeVal = somelib_h.MyString_get_boxed_str(boxArena, internal);
        return SliceUtils.readUtf8(nativeVal);
    }

}
