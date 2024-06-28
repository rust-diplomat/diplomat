// Generated by jextract

package dev.diplomattest.somelib.ntv;

import java.lang.invoke.*;
import java.lang.foreign.*;
import java.nio.ByteOrder;
import java.util.*;
import java.util.function.*;
import java.util.stream.*;

import static java.lang.foreign.ValueLayout.*;
import static java.lang.foreign.MemoryLayout.PathElement.*;

/**
 * {@snippet lang=c :
 * struct _OSUnalignedU32 {
 *     volatile uint32_t __val;
 * }
 * }
 */
public class _OSUnalignedU32 {

    _OSUnalignedU32() {
        // Should not be called directly
    }

    private static final GroupLayout $LAYOUT = MemoryLayout.structLayout(
        somelib_h.align(somelib_h.C_INT, 1).withName("__val")
    ).withName("_OSUnalignedU32");

    /**
     * The layout of this struct
     */
    public static final GroupLayout layout() {
        return $LAYOUT;
    }

    private static final OfInt __val$LAYOUT = (OfInt)$LAYOUT.select(groupElement("__val"));

    /**
     * Layout for field:
     * {@snippet lang=c :
     * volatile uint32_t __val
     * }
     */
    public static final OfInt __val$layout() {
        return __val$LAYOUT;
    }

    private static final long __val$OFFSET = 0;

    /**
     * Offset for field:
     * {@snippet lang=c :
     * volatile uint32_t __val
     * }
     */
    public static final long __val$offset() {
        return __val$OFFSET;
    }

    /**
     * Getter for field:
     * {@snippet lang=c :
     * volatile uint32_t __val
     * }
     */
    public static int __val(MemorySegment struct) {
        return struct.get(__val$LAYOUT, __val$OFFSET);
    }

    /**
     * Setter for field:
     * {@snippet lang=c :
     * volatile uint32_t __val
     * }
     */
    public static void __val(MemorySegment struct, int fieldValue) {
        struct.set(__val$LAYOUT, __val$OFFSET, fieldValue);
    }

    /**
     * Obtains a slice of {@code arrayParam} which selects the array element at {@code index}.
     * The returned segment has address {@code arrayParam.address() + index * layout().byteSize()}
     */
    public static MemorySegment asSlice(MemorySegment array, long index) {
        return array.asSlice(layout().byteSize() * index);
    }

    /**
     * The size (in bytes) of this struct
     */
    public static long sizeof() { return layout().byteSize(); }

    /**
     * Allocate a segment of size {@code layout().byteSize()} using {@code allocator}
     */
    public static MemorySegment allocate(SegmentAllocator allocator) {
        return allocator.allocate(layout());
    }

    /**
     * Allocate an array of size {@code elementCount} using {@code allocator}.
     * The returned segment has size {@code elementCount * layout().byteSize()}.
     */
    public static MemorySegment allocateArray(long elementCount, SegmentAllocator allocator) {
        return allocator.allocate(MemoryLayout.sequenceLayout(elementCount, layout()));
    }

    /**
     * Reinterprets {@code addr} using target {@code arena} and {@code cleanupAction} (if any).
     * The returned segment has size {@code layout().byteSize()}
     */
    public static MemorySegment reinterpret(MemorySegment addr, Arena arena, Consumer<MemorySegment> cleanup) {
        return reinterpret(addr, 1, arena, cleanup);
    }

    /**
     * Reinterprets {@code addr} using target {@code arena} and {@code cleanupAction} (if any).
     * The returned segment has size {@code elementCount * layout().byteSize()}
     */
    public static MemorySegment reinterpret(MemorySegment addr, long elementCount, Arena arena, Consumer<MemorySegment> cleanup) {
        return addr.reinterpret(layout().byteSize() * elementCount, arena, cleanup);
    }
}

