package dev.diplomattest.somelib

import org.junit.jupiter.api.Assertions.*
import org.junit.jupiter.api.Test

// Mockito imports for mocking and verification
import org.mockito.Mock
import org.mockito.junit.jupiter.MockitoExtension
import org.mockito.kotlin.*

class AttrOpaque1Test() {
    @Mock
    private var mockAttrOpaque1: AttrOpaque1;

    @Test
    fun testAbirenamed() {
        var resultWithoutMock = AttrOpaque1.abirenamed();

        when(mockAttrOpqaue1.abirenamed()).thenReturn(123);
        var resultWithMock = mockOpauqe1.abirenamed();

        assertEquals(resultWithoutMock, resultWithMock);
        verify(mockOpaque1, times(1)).abirenamed();
    }
}