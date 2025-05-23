mod graph_a;

use graph_a::Graph;

fn main() {
    let mut graph = Graph::new(5);

    graph.add_edge(0, 1, 2.0);
    graph.add_edge(0, 2, 4.0);
    graph.add_edge(1, 2, 1.0);
    graph.add_edge(1, 3, 7.0);
    graph.add_edge(2, 3, 3.0);
    graph.add_edge(2, 4, 5.0);
    graph.add_edge(3, 4, 2.0);

    println!("DFS:");
    let dfs_result = graph.dfs(0);
    println!("DFS traversal starting from 0: {:?}\n", dfs_result);

    println!("BFS:");
    let bfs_result = graph.bfs(0);
    println!("BFS traversal starting from 0: {:?}\n", bfs_result);

    println!("Dijkstra's algorithm:");
    let dijkstra_result = graph.dijkstra(0);
    println!("Shortest distances from 0: {:?}\n", dijkstra_result);

    println!("Kruskal's algorithm:");
    let kruskal_result = graph.kruskal();
    println!("Minimum Spanning Tree edges: {:?}\n", kruskal_result);

    println!("Prim's algorithm:");
    let prim_result = graph.prim();
    println!("Minimum Spanning Tree edges: {:?}\n", prim_result);

    println!("Floyd-Warshall algorithm:");
    let floyd_result = graph.floyd_warshall();
    println!("All-pairs shortest paths:");
    for (i, row) in floyd_result.iter().enumerate() {
        println!("From vertex {}: {:?}", i, row);
    }
}
