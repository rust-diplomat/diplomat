package dev.diplomattest.somelib;

import dev.diplomattest.somelib.ntv.somelib_h;


import java.lang.foreign.Arena;
import java.lang.foreign.MemorySegment;
import java.lang.ref.Cleaner;
import java.lang.foreign.SegmentAllocator;
import static java.lang.foreign.ValueLayout.*;
import java.nio.charset.StandardCharsets;

public enum ContiguousEnum {
    C,
    D,
    E,
    F,
    ;

    static ContiguousEnum fromInt(int i) {
        switch (i) {
            case 0 -> {
                return ContiguousEnum.C;
            }
            case 1 -> {
                return ContiguousEnum.D;
            }
            case 2 -> {
                return ContiguousEnum.E;
            }
            case 3 -> {
                return ContiguousEnum.F;
            }
            
        }
        throw new RuntimeException("Unexpected int for ContiguousEnum:" + i);
    }

    int toInt() {
        switch (this) {
            case C -> {
                return 0;
            }
            case D -> {
                return 1;
            }
            case E -> {
                return 2;
            }
            case F -> {
                return 3;
            }
            
        }
        throw new RuntimeException("Unexpected variant for ContiguousEnum:" + this);
    }

    
}
