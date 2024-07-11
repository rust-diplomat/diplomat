package dev.diplomattest.somelib;

import dev.diplomattest.somelib.ntv.DiplomatStringsView;

import java.lang.foreign.MemorySegment;
import java.lang.foreign.ValueLayout;
import java.lang.ref.Cleaner;
import java.nio.charset.StandardCharsets;
import java.util.ArrayList;

//TIP To <b>Run</b> code, press <shortcut actionId="Run"/> or
// click the <icon src="AllIcons.Actions.Execute"/> icon in the gutter.
public class Main {

    public sealed interface Result<T, E> {
        boolean isOk = false;
    }
    public final class Ok<T, E> implements Result<T, E> {
        boolean isOk = true;
        T value;
        public Ok(T value) {
            value = value;
        }
    }
    public final class Err<T, E> implements Result<T, E> {
        boolean isOk = false;
        E err;
        public Err(E err) {
            err = err;
        }
    }

    class ResBuilder<T, E> {
        ResBuilder() {}
        Ok<T, E> Ok(T value) {
            return Ok(value);
        }
        Err<T, E> Err(E err) {
            return Err(err);
        }
    }

    class Unit {}
    static final Cleaner cleaner = Cleaner.create();
    public static void main(String[] args) {
        var str = "Whaddap G";
        var opaque = Opaque.fromStr(str);
        //TIP Press <shortcut actionId="ShowIntentionActions"/> with your caret at the highlighted text
        // to see how IntelliJ IDEA suggests fixing it.
        System.out.printf("Hello and welcome!");

        for (int i = 1; i <= 5; i++) {
            //TIP Press <shortcut actionId="Debug"/> to start debugging your code. We have set one <icon src="AllIcons.Debugger.Db_set_breakpoint"/> breakpoint
            // for you, but you can always add more by pressing <shortcut actionId="ToggleLineBreakpoint"/>.
            System.out.println("i = " + i);
        }
    }

}

class SliceUtils {
    static byte[] byteSlice(MemorySegment segment) {
        return segment.toArray(ValueLayout.JAVA_BYTE);
    }
    static short[] shortSlice(MemorySegment segment) {
        return segment.toArray(ValueLayout.JAVA_SHORT);
    }
    static int[] intSlice(MemorySegment segment) {
        return segment.toArray(ValueLayout.JAVA_INT);
    }
    static long[] longSlice(MemorySegment segment) {
        return segment.toArray(ValueLayout.JAVA_LONG);
    }
    static float[] floatSlice(MemorySegment segment) {
        return segment.toArray(ValueLayout.JAVA_FLOAT);
    }
    static double[] doubleSlice(MemorySegment segment) {
        return segment.toArray(ValueLayout.JAVA_DOUBLE);
    }
    static String[] stringsSlice(MemorySegment segment) {
        var len = DiplomatStringsView.len(segment);
        String[] returnSlice = new String[(int) len];
        for (var i = 0; i < len; i++) {
            var slice = DiplomatStringsView.asSlice(segment, i);
            returnSlice[i] = slice.getString(0, StandardCharsets.UTF_8);
        }
        return returnSlice;
    }
    static String readUtf8(MemorySegment segment) {
        return segment.getString(0, StandardCharsets.UTF_8);
    }
    static String readUtf16(MemorySegment segment) {
        return segment.getString(0, StandardCharsets.UTF_16);
    }
}