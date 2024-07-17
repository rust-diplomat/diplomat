package dev.diplomattest.somelib;

import org.junit.jupiter.api.Test;

import java.util.Arrays;
import java.util.Comparator;

import static org.junit.jupiter.api.Assertions.*;

class MyStringTest {

    @Test
    void newFromFirst() {
        var str = "Hi There";
        var str1 = "Hi There 1";
        var str2 = "Hi There 2";
        var myStr0 = new MyString(str);
        assertEquals(myStr0.getStr(), str);
        var strs = new String[]{str, str1, str2};
        var myStr = MyString.newFromFirst(strs);
        assertEquals(myStr.getStr(), str);
        assertEquals(myStr.getBoxedStr(), str);

    }
}