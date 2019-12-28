package com.eteterra.adventofcode.day_06

import org.junit.jupiter.api.Test
import kotlin.test.assertEquals

internal class DepthSearchTest {

    @Test
    fun `it finds all connected nodes in a tree`() {
        val tree = TreeNode("COM").addChildren(TreeNode.create(listOf("A", "B", "C")))
        assertEquals(
            TreeNode.create(listOf("COM", "A", "B", "C")),
            DepthSearch(tree).findAll())
    }

    @Test
    fun `it finds a specific node in a tree`() {
        val tree = TreeNode("COM").addChildren(TreeNode.create(listOf("A", "B", "C")))
        assertEquals(
            TreeNode("A"),
            DepthSearch(tree).find(TreeNode("A")))
    }

    @Test
    fun `it returns a null object when node cannot be found`() {
        val tree = TreeNode("COM")
        assertEquals(
            INVALID_NODE,
            DepthSearch(tree).find(TreeNode("A")))
    }

    @Test
    fun `it finds the depth of a node in the tree`() {
        val tree = TreeNode("COM").addChildren(TreeNode.create(
            listOf("A", "B", "C")) + TreeNode("D").addChild(TreeNode("E")))

        assertEquals(1, DepthSearch(tree).depthOf(TreeNode("COM")))
        assertEquals(2, DepthSearch(tree).depthOf(TreeNode("A")))
        assertEquals(3, DepthSearch(tree).depthOf(TreeNode("E")))
    }
}