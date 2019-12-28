package com.eteterra.adventofcode.day_06

class TreeNode(override val label: String) : ITreeNode {
    override var children = mutableListOf<TreeNode>()

    override fun addChild(other: TreeNode) : TreeNode {
        children.add(other)
        return this
    }

    override fun addChildren(others: List<TreeNode>): TreeNode {
        others.map{ addChild(it) }
        return this
    }

    override fun equals(other: Any?): Boolean {
        if (this === other) return true
        if (javaClass != other?.javaClass) return false
        other as TreeNode
        if (label != other.label) return false
        return true
    }

    override fun hashCode(): Int {
        return label.hashCode()
    }

    override fun toString(): String {
        return "TreeNode(label='$label', children=$children)"
    }

    companion object {
        fun create(labels: List<String>): List<TreeNode> {
            return labels.map{ TreeNode(it) }
        }
    }
}