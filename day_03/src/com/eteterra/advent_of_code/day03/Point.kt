package com.eteterra.advent_of_code.day03

class Point(val x: Int, val y: Int): Comparable<Point> {

    var link: Point = this

    fun hasLink(): Boolean { return link != this}

    fun link(other: Point): Point {
        if(!onSameX(other) && !onSameY(other)) return this

        link = other
        return this // fluent
    }

    private fun onSameX(other: Point): Boolean { return x == other.x }
    private fun onSameY(other: Point): Boolean { return y == other.y }

    override fun toString(): String {
        return "Point(x=$x, y=$y)"
    }

    override fun equals(other: Any?): Boolean {
        if (this === other) return true
        if (javaClass != other?.javaClass) return false

        other as Point

        if (x != other.x) return false
        if (y != other.y) return false

        return true
    }

    override fun hashCode(): Int {
        var result = x
        result = 31 * result + y
        return result
    }

    override fun compareTo(other: Point): Int {
        return when {
            this.x == other.x && this.y > other.y -> 1
            this.x == other.x && this.y < other.y -> -1
            this.y == other.y && this.x > other.x -> 1
            this.y == other.y && this.x < other.x -> -1
            this == other -> 0
            else -> throw Error("$this and $other do not share a common axis, cannot compare relatively")
        }
    }

    operator fun rangeTo(other: Point): PointRange {
        if(!onSameX(other) && !onSameY(other))
            throw Error("$this and $other do not share a common axis, cannot create range from them")

        return when(this < other) {
            true -> PointRange(this, other)
            false -> PointRange(other, this)
        }
    }

    inner class PointRange(
        override val start: Point,
        override val endInclusive: Point
    ) : ClosedRange<Point> {
        fun items(): List<Point> {
            val items = mutableListOf<Point>()
            when(start.x == endInclusive.x) {
                true -> for(y in start.y..endInclusive.y) {
                    items.add(Point(start.x, y))
                }
                false -> for(x in start.x..endInclusive.x) {
                    items.add(Point(x, start.y))
                }
            }
            return items
        }
    }
}