package com.eteterra.advent_of_code.day03

import org.junit.jupiter.api.Assertions.*
import org.junit.jupiter.api.Test

internal class ShortestPathXTest {

    @Test fun `find the intersection of 2 wires with the shortest path`() {
        val firstWire = Wire.create(
            Point(0, 0),
            "R8,U5,L5,D3"
        )
        val secondWire = Wire.create(
            Point(0, 0),
            "U7,R6,D4,L4"
        )
        assertEquals(Point(6, 5), shortestPathIntersection(firstWire, secondWire))
    }

    @Test fun `find the intersection of 2 wires with the shortest path given more complex example`() {
        val firstWire = Wire.create(
            Point(0, 0),
            "R75,D30,R83,U83,L12,D49,R71,U7,L72"
        )
        val secondWire = Wire.create(
            Point(0, 0),
            "U62,R66,U55,R34,D71,R55,D58,R83"
        )
        assertEquals(610, totalSteps(shortestPathIntersection(firstWire, secondWire), firstWire, secondWire))
    }
}