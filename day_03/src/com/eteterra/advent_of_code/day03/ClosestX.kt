package com.eteterra.advent_of_code.day03

fun closestIntersection(a:Wire, other: Wire): Point {
    return a.path()
        .filter{ it != Point(0,0)}
        .intersect(other.path())
        .sortedBy { manhattanDistanceTo0(it) }
        .first()
}

fun manhattanDistanceTo0(other: Point): Int {
    return Math.abs(other.x) + Math.abs(other.y)
}