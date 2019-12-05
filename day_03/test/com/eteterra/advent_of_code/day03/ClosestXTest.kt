package com.eteterra.advent_of_code.day03

import org.junit.jupiter.api.Assertions.*
import org.junit.jupiter.api.Test

internal class ClosestXTest {

    @Test
    fun `find the closest place where the wires cross`() {
        val firstWire = Wire.create(
            Point(0, 0),
            "R8,U5,L5,D3"
        )
        val secondWire = Wire.create(
            Point(0, 0),
            "U7,R6,D4,L4"
        )
        assertEquals(Point(3, 3), closestIntersection(firstWire, secondWire))
    }

    @Test
    fun `find the distance to the closest place where the wires cross given more complex wiring diagram`() {
        val firstWire = Wire.create(
            Point(0, 0),
            "R75,D30,R83,U83,L12,D49,R71,U7,L72"
        )
        val secondWire = Wire.create(
            Point(0, 0),
            "U62,R66,U55,R34,D71,R55,D58,R83"
        )
        assertEquals(159, manhattanDistanceTo0(closestIntersection(firstWire, secondWire)))
    }
}