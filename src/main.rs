struct Edge { from: usize, to: usize, weight: i64 }

fn bellman_ford(n: usize, edges: &[Edge], start: usize) -> Result<Vec<i64>, &'static str> {
    let mut dist = vec![i64::MAX; n];
    dist[start] = 0;
    for _ in 0..n - 1 {
        for e in edges {
            if dist[e.from] != i64::MAX && dist[e.from] + e.weight < dist[e.to] {
                dist[e.to] = dist[e.from] + e.weight;
            }
        }
    }
    for e in edges {
        if dist[e.from] != i64::MAX && dist[e.from] + e.weight < dist[e.to] {
            return Err("negative cycle detected");
        }
    }
    Ok(dist)
}

fn main() {
    let edges = vec![Edge{from:0,to:1,weight:1}, Edge{from:1,to:2,weight:2}, Edge{from:0,to:2,weight:5}];
    match bellman_ford(3, &edges, 0) {
        Ok(dist) => println!("Distances: {:?}", dist),
        Err(e) => println!("Error: {}", e),
    }
}
