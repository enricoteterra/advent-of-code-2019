package com.eteterra.adventofcode.day_06

interface ITreeNode {
    val label: String
    var children: MutableList<TreeNode>
    fun addChild(other: TreeNode) : TreeNode
    fun addChildren(others: List<TreeNode>): TreeNode
}