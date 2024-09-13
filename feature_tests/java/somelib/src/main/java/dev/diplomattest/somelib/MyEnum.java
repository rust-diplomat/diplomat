package dev.diplomattest.somelib;

import dev.diplomattest.somelib.ntv.somelib_h;


import java.lang.foreign.Arena;
import java.lang.foreign.MemorySegment;
import java.lang.ref.Cleaner;
import java.lang.foreign.SegmentAllocator;
import static java.lang.foreign.ValueLayout.*;
import java.nio.charset.StandardCharsets;

public enum MyEnum {
    A,
    B,
    C,
    D,
    E,
    F,
    ;

    static MyEnum fromInt(int i) {
        switch (i) {
            case -2 -> {
                return MyEnum.A;
            }
            case -1 -> {
                return MyEnum.B;
            }
            case 0 -> {
                return MyEnum.C;
            }
            case 1 -> {
                return MyEnum.D;
            }
            case 2 -> {
                return MyEnum.E;
            }
            case 3 -> {
                return MyEnum.F;
            }
            
        }
        throw new RuntimeException("Unexpected int for MyEnum:" + i);
    }

    int toInt() {
        switch (this) {
            case A -> {
                return -2;
            }
            case B -> {
                return -1;
            }
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
        throw new RuntimeException("Unexpected variant for MyEnum:" + this);
    }

    
    public static MyEnum getA() {
        
        var nativeVal = somelib_h.MyEnum_get_a();
        
        var returnVal = MyEnum.fromInt(nativeVal);
        return returnVal;
                
    }
    
}
