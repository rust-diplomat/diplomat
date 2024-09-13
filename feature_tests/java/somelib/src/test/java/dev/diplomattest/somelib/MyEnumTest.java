package dev.diplomattest.somelib;

import org.junit.jupiter.api.Test;

import static org.junit.jupiter.api.Assertions.assertEquals;

public class MyEnumTest {
    @Test
    public void testGetA() {
        assertEquals(MyEnum.getA(), MyEnum.A);
    }
}
