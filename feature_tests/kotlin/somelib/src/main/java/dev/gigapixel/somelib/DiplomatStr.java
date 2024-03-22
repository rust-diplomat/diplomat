package dev.gigapixel.somelib;
import com.sun.jna.Pointer;
import com.sun.jna.Structure;

import java.util.List;

public class DiplomatStr extends Structure implements Structure.ByValue {
    public Pointer data; // Pointer to const char
    public long len; // size_t

    // Define the fields of the struct
    @Override
    protected List<String> getFieldOrder() {
        return List.of("data", "len");
    }
}