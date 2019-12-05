package com.eteterra.advent_of_code.day03

import org.junit.jupiter.api.Test
import kotlin.test.assertEquals

internal class WireTest {

    @Test fun `it has an end`() {
        assertEquals(Point(0,0), Wire(Point(0, 0)).end)
        assertEquals(Point(0,8), Wire(Point(0, 0)).extendTo(Point(0,8)).end)
        assertEquals(Point(6,0), Wire(Point(0, 0)).extendTo(Point(6,0)).end)
        assertEquals(
            Point(6,8),
            Wire(Point(0, 0)).extendTo(Point(6,0)).extendTo(Point(6,8)).end)
    }

    @Test
    fun `it creates wiring path from diagram` () {
        val wire = Wire.create(Point(0,0), "R8,U5,L5,D3")
        assertEquals(Point(0,0), wire.path()[0])
        assertEquals(Point(1,0), wire.path()[1])
        assertEquals(Point(8,0), wire.path()[8])
        assertEquals(Point(8,1), wire.path()[9])
        assertEquals(Point(8,5), wire.path()[13])
        assertEquals(Point(3,5), wire.path()[18])
        assertEquals(Point(3,2), wire.path()[21])

        assertEquals(22, wire.path().count())
        assertEquals(Point(3, 2), wire.path().last())
    }
}