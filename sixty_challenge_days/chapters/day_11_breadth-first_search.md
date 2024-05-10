# Day 11 - Breadth-First Search (BFS) - 30-03-2024

## Introduction
BFS is a fundanmental graph transversal algorithm that is usefull to check if a path from a point A to B exists and the find the shortest path for them. The algorithm doesn't check if a Node is more relevante than another (weighted Node), it just finds the shortest path by verifying all the Nodes and **neighboars** (adjacent Nodes). As it can be use in cycle graph, is necessary to tag a Node when it is visited to avoid check its neighboars again.

## Complexity Analysis
### Time Complexity
as V = Number of Vertices (Node) and E = number of Edges (Connection between two Nodes).
The Time Complexit of BFS algorithm is O(V + E) b

### Space Complexity
Is O(V) because BFS uses a queue to keep track of the vertices that need to be visited. In the worst case, the queue can contain all the vertices in the graph.

## Implementations
```js

import assert from 'node:assert'
import test from 'node:test'


class Graph {
    nodes
    constructor() {
        this.nodes = new Map()
    }

    addEdge(from, to) {
        let node = this.nodes.get(from.value)
        if (!node) {
            from.neighboars.push(to)
            to.neighboars.push(from)
            this.nodes.set(from.value, from)
        } else {
            to.neighboars.push(from)
            node.neighboars.push(to)
        }
    }


}

/**
 * Performs a breadth-first search (BFS) on a graph to find the shortest path from a source node to a target node.
 * @param {Graph} graph - The graph to perform the BFS on.
 * @param {Node} source - The source node.
 * @param {string} target - The value of the target node.
 * @returns {string} - The shortest path from the source node to the target node, represented as a string.
 */
function bfs(graph, source, target) {
    let visiteds = new Set([source.value])
    let nodesToExplore = []
    let node = graph.nodes.get(source.value)
    let paths = new Map([source.value, null])
    nodesToExplore.push(...node.neighboars)
    while (nodesToExplore.length > 0) {
        const node = nodesToExplore.shift()
        if (node.value === target) {
            paths.push(...node.neighboars.filter(node => paths[paths.length - 1].neighboars.some(n => n.value == node.value)))
            paths.push(node)
            const shortestPath = paths.reduce((str, node) => `${str.value || str} -> ${node.value}`)
            return `${shortestPath}`
        }
        if (!visiteds.has(node.value)) {
            if (nodesToExplore.length <= 1) {
                paths.push(node)
            }

            visiteds.add(node.value)
            const notVisiteds = node.neighboars.filter(node => {
                return !visiteds.has(node.value)
            })

            nodesToExplore.push(...notVisiteds)
        }
    }
    return ''
}

class Node {
    value
    neighboars

    constructor(value) {
        this.value = value
        this.neighboars = []
    }
}


test("should find the shortest path from a graph", t => {
    const graph = new Graph()
    const lucas = new Node("Lucas")
    const matheus = new Node("Matheus")
    const paulo = new Node("Paulo")
    const barbara = new Node("Barbara")
    const diego = new Node("Diego")
    const rosana = new Node("Rosana")
    graph.addEdge(lucas, matheus)
    graph.addEdge(matheus, paulo)
    graph.addEdge(matheus, barbara)
    graph.addEdge(paulo, barbara)
    graph.addEdge(barbara, rosana)
    graph.addEdge(paulo, rosana)
    graph.addEdge(rosana, diego)

    const testCases = [{
        graph,
        source: lucas,
        target: 'Diego',
        output: 'Lucas -> Matheus -> Paulo -> Rosana -> Diego'
    }, {
        graph,
        source: barbara,
        target: 'Lucas',
        output: 'Barbara -> Matheus -> Lucas'
    }, {
        graph,
        source: lucas,
        target: 'Otavio',
        output: ''
    }]

    for (const { graph, source, target, output } of testCases) {
        const result = bfs(graph, source, target)
        assert.strictEqual(result, output, `Expect result ${result} to be equal ${output} for target ${target}`)
    }
})
```

## References
1. https://visualgo.net/en/sorting
2. Sweigart, Al; The Recursive Book of Recursion 1ed. Chapter 6: Divide-and-Conquer Algorithms.
https://www.geeksforgeeks.org/breadth-first-search-or-bfs-for-a-graph/