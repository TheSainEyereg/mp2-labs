use std::{
    cmp::Ordering,
    collections::{BinaryHeap, VecDeque},
};

#[derive(Debug, Clone)]
pub struct Edge {
    pub to: usize,
    pub weight: f64,
}

#[derive(Debug, Clone)]
pub struct Graph {
    pub vertices: usize,
    pub adjacency_list: Vec<Vec<Edge>>,
}

impl Graph {
    pub fn new(vertices: usize) -> Self {
        Graph {
            vertices,
            adjacency_list: vec![Vec::new(); vertices],
        }
    }

    pub fn add_edge(&mut self, from: usize, to: usize, weight: f64) {
        self.adjacency_list[from].push(Edge { to, weight });
        // Для неориентированного графа добавляем обратное ребро
        self.adjacency_list[to].push(Edge { to: from, weight });
    }

    pub fn dfs(&self, start: usize) -> Vec<usize> {
        let mut visited = vec![false; self.vertices];
        let mut result = Vec::new();
        self.dfs_util(start, &mut visited, &mut result);
        result
    }

    fn dfs_util(&self, vertex: usize, visited: &mut [bool], result: &mut Vec<usize>) {
        visited[vertex] = true;
        result.push(vertex);

        for edge in &self.adjacency_list[vertex] {
            if !visited[edge.to] {
                self.dfs_util(edge.to, visited, result);
            }
        }
    }

    pub fn bfs(&self, start: usize) -> Vec<usize> {
        let mut visited = vec![false; self.vertices];
        let mut queue = VecDeque::new();
        let mut result = Vec::new();

        visited[start] = true;
        queue.push_back(start);

        while let Some(vertex) = queue.pop_front() {
            result.push(vertex);

            for edge in &self.adjacency_list[vertex] {
                if !visited[edge.to] {
                    visited[edge.to] = true;
                    queue.push_back(edge.to);
                }
            }
        }

        result
    }

    pub fn dijkstra(&self, start: usize) -> Vec<f64> {
        let mut visited = vec![false; self.vertices];
        let mut distances = vec![f64::INFINITY; self.vertices];
        distances[start] = 0.0;

        loop {
            let current = (0..self.vertices)
                .filter(|&v| !visited[v])
                .min_by(|&a, &b| distances[a].partial_cmp(&distances[b]).unwrap());

            match current {
                Some(v) => {
                    visited[v] = true;

                    for edge in &self.adjacency_list[v] {
                        let new_distance = distances[v] + edge.weight;
                        if new_distance < distances[edge.to] {
                            distances[edge.to] = new_distance;
                        }
                    }
                }
                None => break,
            }
        }

        distances
    }

    pub fn kruskal(&self) -> Vec<(usize, usize, f64)> {
        let mut edges = Vec::new();
        for (from, edges_list) in self.adjacency_list.iter().enumerate() {
            for edge in edges_list {
                if from < edge.to {
                    // Избегаем дублирования рёбер
                    edges.push((from, edge.to, edge.weight));
                }
            }
        }
        edges.sort_by(|a, b| a.2.partial_cmp(&b.2).unwrap());

        let mut parent = (0..self.vertices).collect::<Vec<_>>();
        let mut rank = vec![0; self.vertices];
        let mut mst = Vec::new();

        for (from, to, weight) in edges {
            let from_root = self.find(&mut parent, from);
            let to_root = self.find(&mut parent, to);

            if from_root != to_root {
                mst.push((from, to, weight));
                self.union(&mut parent, &mut rank, from_root, to_root);
            }
        }

        mst
    }

    fn find(&self, parent: &mut [usize], x: usize) -> usize {
        if parent[x] != x {
            parent[x] = self.find(parent, parent[x]);
        }
        parent[x]
    }

    fn union(&self, parent: &mut [usize], rank: &mut [usize], x: usize, y: usize) {
        let x_root = self.find(parent, x);
        let y_root = self.find(parent, y);

        if x_root == y_root {
            return;
        }

        if rank[x_root] < rank[y_root] {
            parent[x_root] = y_root;
        } else if rank[x_root] > rank[y_root] {
            parent[y_root] = x_root;
        } else {
            parent[y_root] = x_root;
            rank[x_root] += 1;
        }
    }

    pub fn prim(&self) -> Vec<(usize, usize, f64)> {
        let mut visited = vec![false; self.vertices];
        let mut mst = Vec::new();
        let mut heap = BinaryHeap::new();

        // Начинаем с вершины 0
        visited[0] = true;
        for edge in &self.adjacency_list[0] {
            heap.push(EdgeState {
                cost: -edge.weight, // Отрицательный вес для max-heap
                from: 0,
                to: edge.to,
            });
        }

        while let Some(EdgeState { cost, from, to }) = heap.pop() {
            if visited[to] {
                continue;
            }

            visited[to] = true;
            mst.push((from, to, -cost)); // Возвращаем положительный вес

            for edge in &self.adjacency_list[to] {
                if !visited[edge.to] {
                    heap.push(EdgeState {
                        cost: -edge.weight,
                        from: to,
                        to: edge.to,
                    });
                }
            }
        }

        mst
    }

    pub fn floyd_warshall(&self) -> Vec<Vec<f64>> {
        let mut dist = vec![vec![f64::INFINITY; self.vertices]; self.vertices];

        for i in 0..self.vertices {
            dist[i][i] = 0.0;
            for edge in &self.adjacency_list[i] {
                dist[i][edge.to] = edge.weight;
            }
        }

        for k in 0..self.vertices {
            for i in 0..self.vertices {
                for j in 0..self.vertices {
                    if dist[i][k] != f64::INFINITY && dist[k][j] != f64::INFINITY {
                        dist[i][j] = dist[i][j].min(dist[i][k] + dist[k][j]);
                    }
                }
            }
        }

        dist
    }
}

#[derive(Copy, Clone)]
struct EdgeState {
    cost: f64,
    from: usize,
    to: usize,
}

impl PartialEq for EdgeState {
    fn eq(&self, other: &Self) -> bool {
        self.from == other.from && self.to == other.to
    }
}

impl Eq for EdgeState {}

impl Ord for EdgeState {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.partial_cmp(&self.cost).unwrap()
    }
}

impl PartialOrd for EdgeState {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
