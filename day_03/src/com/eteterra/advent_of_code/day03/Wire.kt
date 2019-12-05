package com.eteterra.advent_of_code.day03

class Wire(val start: Point) {

    val end get() = run {
        var point = start
        while (point.hasLink()) { point = point.link }
        point
    }

    fun extendTo(point: Point): Wire {
        end.link(point)
        return this // fluent
    }

    fun path(): List<Point> {
        val path = mutableListOf<Point>(start)
        var point = start
        while (point.hasLink()) {
            when {
                point.x == point.link.x -> {
                    val smallerY = listOf(point.y, point.link.y).min()!!
                    val largerY = listOf(point.y, point.link.y).max()!!
                    when(smallerY == point.y) {
                        true -> for(y in smallerY+1..largerY) { path.add(Point(point.x, y)) }
                        false -> for(y in largerY-1 downTo smallerY) { path.add(Point(point.x, y)) }
                    }
                }
                point.y == point.link.y -> {
                    val smallerX = listOf(point.x, point.link.x).min()!!
                    val largerX = listOf(point.x, point.link.x).max()!!
                    when(smallerX == point.x) {
                        true -> for(x in smallerX+1..largerX) { path.add(Point(x, point.y)) }
                        false -> for(x in largerX-1 downTo smallerX) { path.add(Point(x, point.y)) }
                    }

                }
            }
            point = point.link
        }
        return path
    }

    companion object {
        enum class DIRECTION(val code:Char) {
            UP('U'), DOWN('D'), LEFT('L'), RIGHT('R')
        }
        fun create (start:Point, input: String): Wire {
            val wire = Wire(start)
            input.split(',').forEach { when(it[0]) {
                Companion.DIRECTION.UP.code -> wire.extendTo(Point(wire.end.x, wire.end.y + parse(
                    it
                )
                ))
                Companion.DIRECTION.DOWN.code -> wire.extendTo(Point(wire.end.x, wire.end.y - parse(
                    it
                )
                ))
                Companion.DIRECTION.LEFT.code -> wire.extendTo(Point(wire.end.x - parse(
                    it
                ), wire.end.y))
                Companion.DIRECTION.RIGHT.code -> wire.extendTo(Point(wire.end.x + parse(
                    it
                ), wire.end.y))
            }}
            return wire
        }

        private fun parse(input: String): Int {
            return input.replace(Regex("""[UDLR]"""), "").toInt()
        }
    }
}