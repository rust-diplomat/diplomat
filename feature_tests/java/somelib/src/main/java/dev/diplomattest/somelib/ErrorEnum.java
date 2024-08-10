package dev.diplomattest.somelib;

import dev.diplomattest.somelib.ntv.somelib_h;


import java.lang.foreign.Arena;
import java.lang.foreign.MemorySegment;
import java.lang.ref.Cleaner;
import java.lang.foreign.SegmentAllocator;
import static java.lang.foreign.ValueLayout.*;
import java.nio.charset.StandardCharsets;

public enum ErrorEnum {
    Foo,
    Bar,
    ;

    static ErrorEnum fromInt(int i) {
        switch (i) {
            case 0 -> {
                return ErrorEnum.Foo;
            }
            case 1 -> {
                return ErrorEnum.Bar;
            }
            
        }
        throw new RuntimeException("Unexpected int for ErrorEnum:" + i);
    }

    int toInt() {
        switch (this) {
            case Foo -> {
                return 0;
            }
            case Bar -> {
                return 1;
            }
            
        }
        throw new RuntimeException("Unexpected variant for ErrorEnum:" + this);
    }

    
}
