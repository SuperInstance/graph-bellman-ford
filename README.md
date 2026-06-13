# Bellman-Ford Shortest Path

**A Rust implementation of the Bellman-Ford algorithm** for single-source shortest paths in weighted graphs with **negative edge detection** — handling the case Dijkstra cannot.

## Why It Matters

Bellman-Ford is essential whenever graphs contain negative-weight edges: currency arbitrage detection (negative cycles = profit opportunities), constraint satisfaction (difference constraints in scheduling), and network routing with cost discounts. While Dijkstra requires non-negative weights, Bellman-Ford handles any real-valued weights and — crucially — **detects negative-weight cycles** that make shortest paths undefined. The trade-off is speed: Bellman-Ford runs in **O(VE)** compared to Dijkstra's **O((V+E) log V)**, so it's used when negative weights or cycle detection is required.

## How It Works

The algorithm performs `V-1` relaxation passes over all edges. In each pass, for every edge `(u, v, w)`, if `dist[u] + w < dist[v]`, the distance is updated. After `V-1` passes, all shortest paths (which use at most `V-1` edges) have been found. A final pass checks for negative cycles: if any edge can still be relaxed, a negative cycle exists and the algorithm returns an error.

The implementation uses `i64::MAX` as infinity sentinel and skips edges whose source is unreachable (`dist[from] == MAX`), avoiding overflow. After relaxation, the algorithm returns the distance vector or an error string indicating the negative cycle.

## Quick Start

```rust
// cargo run
// Graph: 0 --1--> 1 --2--> 2, 0 --5--> 2
// Output: Distances: [0, 1, 3]

// With a negative cycle:
// let edges = vec![Edge{from:0,to:1,weight:-1}, Edge{from:1,to:0,weight:-1}];
// → "negative cycle detected"
```

## API

The implementation is in `main.rs`. Key function:

| Function | Complexity | Description |
|---|---|---|
| `bellman_ford(n, edges, start)` | **O(VE)** | Shortest paths with negative-cycle detection |

## Architecture Notes

Part of the SuperInstance graph algorithms collection. Companion crates: `graph-dijkstra`, `graph-astar`, `graph-bfs`, `graph-dfs`, `graph-coloring`. See the [Architecture Guide](https://github.com/SuperInstance/SuperInstance/blob/main/ARCHITECTURE.md).

## License

MIT
