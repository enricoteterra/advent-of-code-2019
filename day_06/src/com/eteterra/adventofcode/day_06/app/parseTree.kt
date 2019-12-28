package com.eteterra.adventofcode.day_06.app

import com.eteterra.adventofcode.day_06.DepthSearch
import com.eteterra.adventofcode.day_06.INVALID_NODE
import com.eteterra.adventofcode.day_06.TreeNode

const val NEWLINE = "\n"
const val ORBIT_DELIMITER = ")"

fun parseTree(input: String): TreeNode {
    val parse = input.split(NEWLINE).map { line -> line.split(ORBIT_DELIMITER) }

    val root: TreeNode = TreeNode("COM")
    val tree = DepthSearch(root)

    parse.forEach { (a, b) ->
        run {
            val node = tree.find(TreeNode(a))
            if (node == INVALID_NODE) root.addChild(TreeNode(a))
            tree.find(TreeNode(a)).addChild(TreeNode(b))
        }
    }
    return root
}