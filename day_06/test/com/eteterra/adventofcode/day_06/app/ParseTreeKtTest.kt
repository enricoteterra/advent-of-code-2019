package com.eteterra.adventofcode.day_06.app

import com.eteterra.adventofcode.day_06.TreeNode
import org.junit.jupiter.api.Test
import kotlin.test.assertEquals

internal class ParseTreeKtTest {

    @Test
    fun `parses simple orbital map`() {
        assertEquals(
            TreeNode("COM").addChild(TreeNode("B")).toString(),
            parseTree("COM)B").toString()
        )
    }

    @Test
    fun `parses extended orbital map`() {
        val expectedTree = TreeNode("COM").addChild(TreeNode("B").addChild(TreeNode("C").addChild(TreeNode("D").addChild(TreeNode("E").addChild(TreeNode("F"))))))
        assertEquals(
            expectedTree.toString(),
            parseTree("""
                COM)B
                B)C
                C)D
                D)E
                E)F
            """.trimIndent()).toString()
        )
    }

    @Test
    fun `parses orbital map with multiple paths`() {
        val expectedTree = TreeNode("COM").addChild(
            TreeNode("B").addChildren(listOf(
                TreeNode("C").addChild(
                    TreeNode("D").addChildren(listOf(
                        TreeNode("E").addChildren(listOf(
                            TreeNode("F"),
                            TreeNode("J").addChild(TreeNode("K").addChild(TreeNode("L")))
                        )),
                        TreeNode("I")
                    ))),
                TreeNode("G").addChild(TreeNode("H"))
            )))
        assertEquals(
            expectedTree.toString(),
            parseTree("""
                COM)B
                B)C
                C)D
                D)E
                E)F
                B)G
                G)H
                D)I
                E)J
                J)K
                K)L
            """.trimIndent()).toString()
        )
    }
}