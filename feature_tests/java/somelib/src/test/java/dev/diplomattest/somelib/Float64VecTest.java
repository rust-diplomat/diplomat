package dev.diplomattest.somelib;

import org.junit.jupiter.api.Test;

import static org.junit.jupiter.api.Assertions.*;

class Float64VecTest {
    @Test
    void testFloat64Vec() {
        double[] dArray = {0.1, 0.2, 0.3, 0.4};
        var float64Vec = Float64Vec.new_(dArray);
        var outArray = float64Vec.asSLice();
        assertEquals(dArray.length, outArray.length);
        for (var i = 0; i < dArray.length; i++) {
            assertEquals(dArray[i], outArray[i]);
        }
    }

}