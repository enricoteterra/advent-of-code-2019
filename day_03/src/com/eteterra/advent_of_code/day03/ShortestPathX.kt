package com.eteterra.advent_of_code.day03

fun shortestPathIntersection(a: Wire, other: Wire): Point {
    return a.path()
        .filter{ it != Point(0,0)}
        .intersect(other.path())
        .sortedBy { totalSteps(it, a, other) }
        .first()
}

fun totalSteps(intersection: Point, a: Wire, other: Wire): Int {
    return pathTravelledTo0(intersection, a) + pathTravelledTo0(intersection, other)
}

fun pathTravelledTo0(intersection: Point, wire: Wire): Int {
    return wire.path().indexOf(intersection)
}