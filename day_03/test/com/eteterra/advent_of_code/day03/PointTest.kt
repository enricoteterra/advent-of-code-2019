package com.eteterra.advent_of_code.day03

import org.junit.jupiter.api.Assertions.*
import org.junit.jupiter.api.Test
import kotlin.test.assertFailsWith

internal class PointTest {

    @Test fun `is not initially linked to another point`() {
        assertFalse(Point(0, 0).hasLink())
    }

    @Test fun `can be linked to another point on the same X axis`() {
        assertTrue(Point(0, 0).link(Point(0, 5)).hasLink())
        assertTrue(Point(1, 5).link(Point(0, 5)).hasLink())

        assertFalse(Point(0, 0).link(Point(1, 1)).hasLink())
        assertFalse(Point(0, 0).link(Point(1, 5)).hasLink())
    }

    @Test fun `can be compared for equality`() {
        assertEquals(Point(0, 0), Point(0, 0))
        assertEquals(Point(1, 0), Point(1, 0))
        assertEquals(Point(1, 2), Point(1, 2))

        assertNotEquals(Point(0, 0), Point(1, 0))
        assertNotEquals(Point(0, 0), Point(0, 1))
        assertNotEquals(Point(0, 0), Point(1, 1))
    }

    @Test fun `can be larger or smaller than another point`() {
        assertTrue(Point(0, 0) < Point(0, 1))
        assertTrue(Point(0, 1) > Point(0, 0))

        assertTrue(Point(0, 0) < Point(0, 10))
        assertTrue(Point(0, 10) > Point(0, 0))

        assertTrue(Point(1, 0) < Point(1, 1))
        assertTrue(Point(1, 1) > Point(1, 0))

        assertFalse(Point(0, 0) < Point(0, 0))
        assertFalse(Point(0, 0) > Point(0, 0))

        assertFailsWith<Error> { Point(
            0,
            0
        ) > Point(1, 1)
        }
    }

    @Test fun `can generate a range of points`() {
        val range = Point(0, 0)..Point(0, 10)
        assertEquals(Point(0, 3), range.items()[3])
        assertEquals(Point(0, 8), range.items()[8])
        assertEquals(Point(0, 0), range.items().first())
        assertEquals(Point(0, 10), range.items().last())
        assertEquals(11, range.items().count())

        val reverseRange = Point(0, 0)..Point(0, -10)
        assertEquals(Point(0, -7), reverseRange.items()[3])
        assertEquals(Point(0, -2), reverseRange.items()[8])
        assertEquals(Point(0, -10), reverseRange.items().first())
        assertEquals(Point(0, 0), reverseRange.items().last())
        assertEquals(11, reverseRange.items().count())

        assertFailsWith<Error> { Point(0, 0)..Point(1,1) }
    }
}