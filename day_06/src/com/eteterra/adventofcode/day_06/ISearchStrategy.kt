package com.eteterra.adventofcode.day_06

interface ISearchStrategy {
    fun findAll(): List<ITreeNode>
    fun find(node: ITreeNode): ITreeNode
}