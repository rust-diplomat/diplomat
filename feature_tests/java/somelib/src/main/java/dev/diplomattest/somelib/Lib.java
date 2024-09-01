package dev.diplomattest.somelib;

import dev.diplomattest.somelib.ntv.*;

import java.lang.foreign.*;
import java.lang.ref.Cleaner;
import java.nio.charset.StandardCharsets;
import java.util.function.Consumer;
import java.util.Arrays;

import static java.lang.foreign.ValueLayout.*;

public class Lib {
    static final Cleaner cleaner = Cleaner.create();
}

class SliceUtils {


    static boolean[] booleanSliceToArray(MemorySegment segment) {
        var data = DiplomatU8View.data(segment);
        var len = DiplomatU8View.len(segment);
        var slice = data.asSlice(0, len * JAVA_BYTE.byteSize());
        var byteArray = slice.toArray(JAVA_BYTE);
        var returnArray = new boolean[(int) len];
        for (var i=0; i < len; i++ ) {
            returnArray[i] = (byteArray[i] == 1);
        }
        return returnArray;
    }

    static byte[] byteSliceToArray(MemorySegment segment) {
        var data = DiplomatU8View.data(segment);
        var len = DiplomatU8View.len(segment);
        var slice = data.asSlice(0, len * JAVA_BYTE.byteSize());
        return slice.toArray(JAVA_BYTE);
    }

    static short[] shortSliceToArray(MemorySegment segment) {
        var data = DiplomatU16View.data(segment);
        var len = DiplomatU16View.len(segment);
        var slice = data.asSlice(0, len * JAVA_SHORT.byteSize());
        return slice.toArray(JAVA_SHORT);
    }
    static int[] intSliceToArray(MemorySegment segment) {
        var data = DiplomatU32View.data(segment);
        var len = DiplomatU32View.len(segment);
        var slice = data.asSlice(0, len * JAVA_INT.byteSize());
        return slice.toArray(JAVA_INT);
    }
    static long[] longSliceToArray(MemorySegment segment) {
        var data = DiplomatU64View.data(segment);
        var len = DiplomatU64View.len(segment);
        var slice = data.asSlice(0, len * JAVA_LONG.byteSize());
        return slice.toArray(JAVA_LONG);
    }
    static float[] floatSliceToArray(MemorySegment segment) {
        var data = DiplomatF32View.data(segment);
        var len = DiplomatF32View.len(segment);
        var slice = data.asSlice(0, len * JAVA_FLOAT.byteSize());
        return slice.toArray(JAVA_FLOAT);
    }

    static double[] doubleSliceToArray(MemorySegment segment) {
        var data = DiplomatF64View.data(segment);
        var len = DiplomatF64View.len(segment);
        var slice = data.asSlice(0, len * JAVA_DOUBLE.byteSize());
        return slice.toArray(JAVA_DOUBLE);
    }

    static MemorySegment byteArrayToSlice(SegmentAllocator arena, byte[] array) {
        var len = array.length;
        var memSeg = arena.allocate(JAVA_BYTE, len);
        for (var i = 0; i < len; i++) {
            memSeg.setAtIndex(JAVA_BYTE, i, array[i]);
        }
        return memSeg;
    }

    static MemorySegment shortArrayToSlice(SegmentAllocator arena, short[] array) {
        var len = array.length;
        var memSeg = arena.allocate(JAVA_SHORT, len);
        for (var i = 0; i < len; i++) {
            memSeg.setAtIndex(JAVA_SHORT, i, array[i]);
        }
        return memSeg;
    }

    static MemorySegment intArrayToSlice(SegmentAllocator arena, int[] array) {
        var len = array.length;
        var memSeg = arena.allocate(JAVA_INT, len);
        for (var i = 0; i < len; i++) {
            memSeg.setAtIndex(JAVA_INT, i, array[i]);
        }
        return memSeg;
    }
    static MemorySegment longArrayToSlice(SegmentAllocator arena, long[] array) {
        var len = array.length;
        var memSeg = arena.allocate(JAVA_LONG, len);
        for (var i = 0; i < len; i++) {
            memSeg.setAtIndex(JAVA_LONG, i, array[i]);
        }
        return memSeg;
    }
    static MemorySegment floatArrayToSlice(SegmentAllocator arena, float[] array) {
        var len = array.length;
        var memSeg = arena.allocate(JAVA_FLOAT, len);
        for (var i = 0; i < len; i++) {
            memSeg.setAtIndex(JAVA_FLOAT, i, array[i]);
        }
        return memSeg;
    }

    static MemorySegment doubleArrayToSlice(SegmentAllocator arena, double[] array) {
        return arena.allocateFrom(JAVA_DOUBLE, array);
    }

    static String readUtf8(MemorySegment segment) {
        var data = DiplomatStringView.data(segment);
        var len = DiplomatStringView.len(segment);
        var strData = data.asSlice(0, len);
        var bytes = strData.toArray(JAVA_BYTE);
        var string = new String(bytes, StandardCharsets.UTF_8);
        return string;
    }

    static String readUtf16(MemorySegment segment) {
        var data = DiplomatStringView.data(segment);
        var len = DiplomatStringView.len(segment);
        var strData = data.asSlice(0, len);
        var bytes = strData.toArray(JAVA_BYTE);
        var string = new String(bytes, StandardCharsets.UTF_16);
        return string;
    }
  // for parameter conversion
    static MemorySegment strs16(SegmentAllocator arena, String [] strings) {
        var diplomatStrsData = DiplomatStringView.allocateArray(strings.length, arena);
        var layout = DiplomatStringView.layout();
        diplomatStrsData.elements(layout).forEach(new Consumer<MemorySegment>() {
            int i = 0;
            @Override
            public void accept(MemorySegment memorySegment) {
                var str = strings[i];
                var bytes = str.getBytes(StandardCharsets.UTF_16);
                var data = arena.allocateFrom(JAVA_BYTE, bytes);
                DiplomatStringView.data(memorySegment, data);
                DiplomatStringView.len(memorySegment, bytes.length);
                i++;
            }
        });
        var diplomatStrsView = DiplomatStrings16View.allocate(arena);
        DiplomatStrings16View.len(diplomatStrsView, strings.length);
        DiplomatStrings16View.data(diplomatStrsView, diplomatStrsData);
        return diplomatStrsView;
    }

    static MemorySegment strs8(SegmentAllocator arena, String [] strings) {
        var diplomatStrsData = DiplomatStringView.allocateArray(strings.length, arena);
        var layout = DiplomatStringView.layout();
        diplomatStrsData.elements(layout).forEach(new Consumer<MemorySegment>() {
            int i = 0;
            @Override
            public void accept(MemorySegment memorySegment) {
                var str = strings[i];
                var bytes = str.getBytes(StandardCharsets.UTF_8);
                var data = arena.allocateFrom(JAVA_BYTE, bytes);
                DiplomatStringView.data(memorySegment, data);
                DiplomatStringView.len(memorySegment, bytes.length);
                i++;
            }
        });

        var diplomatStrsView = DiplomatStringsView.allocate(arena);
        DiplomatStringsView.len(diplomatStrsView, strings.length);
        DiplomatStringsView.data(diplomatStrsView, diplomatStrsData);
        return diplomatStrsView;
    }

    static String readUtf8FromWriteable(MemorySegment writeable) {
        var buffer = DiplomatWrite.buf(writeable);
        var len = DiplomatWrite.len(writeable);
        var bytes = new byte[(int) len];
        for (var i = 0; i < len; i++) {
            bytes[i] = buffer.get(JAVA_BYTE, i);
        }
        return new String(bytes, StandardCharsets.UTF_8);
    }
}