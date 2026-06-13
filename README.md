# Graph Bellman-Ford

A **single-source shortest-path algorithm** for weighted directed graphs that handles negative edge weights and detects negative cycles — the only common shortest-path algorithm that can do both.

## Why It Matters

Dijkstra's algorithm is faster (O((V+E) log V) vs O(VE)) but fails catastrophically on negative edges — it greedily commits to the first path it finds, which may be suboptimal when a later negative edge provides a shortcut. Bellman-Ford trades speed for generality: it relaxes every edge V-1 times, ensuring correctness regardless of edge weights. This matters for currency arbitrage detection (where negative cycles = infinite profit), network routing (RIP protocol), and constraint systems (difference constraints solved via graph shortest-paths). The negative cycle detection capability is unique — no other shortest-path algorithm provides it.

## How It Works

**Core algorithm**:
```
for i in 1..V-1:                    // V-1 iterations
    for each edge (u, v, w):        // Relax all edges
        if dist[u] + w < dist[v]:
            dist[v] = dist[u] + w

// Negative cycle check: one more pass
for each edge (u, v, w):
    if dist[u] + w < dist[v]:
        return Error("negative cycle detected")
```

**Why V-1 iterations suffice**: A shortest path in a graph with V vertices contains at most V-1 edges. Each relaxation pass extends known shortest paths by one edge. After V-1 passes, the longest possible shortest path has been fully relaxed. This is the Bellman-Ford optimality principle.

**Correctness proof sketch**: By induction. After iteration k, `dist[v]` ≤ the length of the shortest walk from source to v using at most k edges. After V-1 iterations, this equals the true shortest-path distance (since no shortest path exceeds V-1 edges). If an edge can still be relaxed after V-1 iterations, a shorter path exists — which requires a negative cycle.

**Complexity**:
| Operation | Bellman-Ford | Dijkstra |
|-----------|-------------|----------|
| Time | O(V × E) | O((V + E) log V) |
| Space | O(V) | O(V + E) |
| Negative edges | ✓ | ✗ |
| Negative cycle detection | ✓ | ✗ |

**Comparison**: On a dense graph (E ≈ V²), Bellman-Ford is O(V³). On sparse graphs (E ≈ V), it's O(V²), competitive with Dijkstra for small graphs. The SPFA (Shortest Path Faster Algorithm) is a queue-based optimization that averages O(E) but degrades to O(VE) worst-case.

**Implementation** in this crate uses `i64::MAX` as infinity and `Result<Vec<i64>, &str>` for the negative-cycle error path. The edge list representation enables cache-friendly sequential iteration.

## Quick Start

```rust
use graph_bellman_ford::bellman_ford;

let edges = vec![
    Edge { from: 0, to: 1, weight: 1 },
    Edge { from: 1, to: 2, weight: 2 },
    Edge { from: 0, to: 2, weight: 5 },
];

match bellman_ford(3, &edges, 0) {
    Ok(distances) => println!("Distances: {:?}", distances), // [0, 1, 3]
    Err(e) => println!("Error: {}", e),
}
```

## API

| Type | Description |
|------|-------------|
| `Edge { from, to, weight }` | A weighted directed edge |
| `bellman_ford(n, edges, start)` | Compute shortest paths; `Result<Vec<i64>, &str>` |

## Architecture Notes

This crate is part of SuperInstance's graph algorithm suite. Bellman-Ford handles the general case (negative weights, cycle detection) while graph-dijkstra handles the common case (non-negative weights, faster). The choice between them is a **γ + η = C** trade-off: Bellman-Ford's generality (low γ from fewer constraints on input) costs more computation time (higher η). See [Architecture](https://github.com/SuperInstance/SuperInstance/blob/main/ARCHITECTURE.md).

## References

- Bellman, R. "On a Routing Problem," Quarterly of Applied Mathematics (1958).
- Ford, L. & Fulkerson, D. *Flows in Networks*, Princeton UP (1962).
- Cormen, T. et al. *Introduction to Algorithms*, 4th ed., MIT Press (2022). Ch. 22.

## License

MIT
