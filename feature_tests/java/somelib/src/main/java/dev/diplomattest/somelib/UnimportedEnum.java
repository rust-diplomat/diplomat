package dev.diplomattest.somelib;

import dev.diplomattest.somelib.ntv.somelib_h;


import java.lang.foreign.Arena;
import java.lang.foreign.MemorySegment;
import java.lang.ref.Cleaner;
import java.lang.foreign.SegmentAllocator;
import static java.lang.foreign.ValueLayout.*;
import java.nio.charset.StandardCharsets;

public enum UnimportedEnum {
    A,
    B,
    C,
    ;

    static UnimportedEnum fromInt(int i) {
        switch (i) {
            case 0 -> {
                return UnimportedEnum.A;
            }
            case 1 -> {
                return UnimportedEnum.B;
            }
            case 2 -> {
                return UnimportedEnum.C;
            }
            
        }
        throw new RuntimeException("Unexpected int for UnimportedEnum:" + i);
    }

    int toInt() {
        switch (this) {
            case A -> {
                return 0;
            }
            case B -> {
                return 1;
            }
            case C -> {
                return 2;
            }
            
        }
        throw new RuntimeException("Unexpected variant for UnimportedEnum:" + this);
    }

    
}
