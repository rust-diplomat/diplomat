package dev.diplomattest.somelib;

import org.junit.jupiter.api.Test;

import java.util.Arrays;

import static org.junit.jupiter.api.Assertions.*;

class Float64VecTest {
    @Test
    void testFloat64Vec() {
        long[] lArray = {1, 2, 3, 4};

        double[] dArray = {1, 2, 3, 4};
        var float64Vec = Float64Vec.newIsize(lArray);
        var outArray = float64Vec.asSlice();
        assertEquals(dArray.length, outArray.length);
        for (var i = 0; i < dArray.length; i++) {
            assertEquals(dArray[i], outArray[i]);
        }
    }

}