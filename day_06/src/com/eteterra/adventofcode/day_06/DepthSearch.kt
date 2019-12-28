package com.eteterra.adventofcode.day_06

val INVALID_NODE = TreeNode("INVALID_NODE")

class DepthSearch(private val tree: ITreeNode) : ISearchStrategy {
    override fun findAll(): List<ITreeNode> {
        return discover(tree)
    }

    override fun find(node: ITreeNode): ITreeNode {
        val path = pathTo(node)
        return if (path.isNotEmpty()) path.last() else INVALID_NODE
    }

    fun depthOf(target: ITreeNode): Int {
        return pathTo(target).count()
    }

    private fun pathTo(target: ITreeNode, current: ITreeNode = tree, path:List<ITreeNode> = emptyList()): List<ITreeNode> {
        if (current == target) return path + listOf(current)
        if (current.children.isEmpty()) return emptyList()
        return current.children.flatMap{ child -> pathTo(target, child, path + listOf(current)) }
    }

    private fun discover(current: ITreeNode): List<ITreeNode> {
        if (current.children.isEmpty()) return listOf(current)
        return listOf(current) + current.children.flatMap{ child -> discover(child) }
    }
}