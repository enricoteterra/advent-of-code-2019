package com.eteterra.adventofcode.day_06

import org.junit.jupiter.api.Assertions.*
import org.junit.jupiter.api.Test

internal class TreeNodeTest {

    @Test fun `it can be compared with other nodes`() {
        assertEquals(TreeNode("COM"), TreeNode("COM"))
        assertNotEquals(TreeNode("A"), TreeNode("COM"))
    }

    @Test fun `it has children`() {
        assertTrue(TreeNode("COM").children.isEmpty())
        assertTrue(TreeNode("COM").addChild(TreeNode("A")).children.isNotEmpty())
        assertEquals(3, TreeNode("COM").addChildren(
            listOf(TreeNode("A"), TreeNode("B"), TreeNode("C"))).children.size)
    }
}