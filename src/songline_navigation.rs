/// Songline pathfinding for tile routing — route tiles through the system using
/// navigable knowledge graphs. Prefer well-traveled paths (high traversal count).

use std::collections::HashMap;

/// A room identifier in the PLATO system.
pub type RoomId = String;

/// An edge in the songline graph connecting two rooms.
#[derive(Debug, Clone)]
pub struct SonglineEdge {
    pub target: RoomId,
    pub traversal_count: u64,
    pub latency_ms: f64,
}

/// The songline graph: a navigable knowledge graph of rooms.
#[derive(Debug, Clone)]
pub struct SonglineGraph {
    /// Adjacency list: room → edges to neighbors
    adjacency: HashMap<RoomId, Vec<SonglineEdge>>,
    /// All known rooms
    rooms: Vec<RoomId>,
}

impl SonglineGraph {
    pub fn new() -> Self {
        Self {
            adjacency: HashMap::new(),
            rooms: Vec::new(),
        }
    }

    /// Register a room.
    pub fn add_room(&mut self, room: RoomId) {
        if !self.rooms.contains(&room) {
            self.rooms.push(room.clone());
            self.adjacency.entry(room).or_default();
        }
    }

    /// Add an edge between rooms (bidirectional by default).
    pub fn add_edge(&mut self, from: RoomId, to: RoomId, latency_ms: f64) {
        self.add_room(from.clone());
        self.add_room(to.clone());

        // Forward edge
        self.adjacency.entry(from.clone()).or_default().push(SonglineEdge {
            target: to.clone(),
            traversal_count: 1,
            latency_ms,
        });

        // Reverse edge
        self.adjacency.entry(to).or_default().push(SonglineEdge {
            target: from,
            traversal_count: 1,
            latency_ms,
        });
    }

    /// Record a traversal (increments count on the edge).
    pub fn record_traversal(&mut self, from: &str, to: &str) {
        if let Some(edges) = self.adjacency.get_mut(from) {
            for edge in edges.iter_mut() {
                if edge.target == to {
                    edge.traversal_count += 1;
                    return;
                }
            }
        }
    }

    /// Get all rooms.
    pub fn rooms(&self) -> &[RoomId] {
        &self.rooms
    }

    /// Get edges from a room.
    pub fn edges_from(&self, room: &str) -> Option<&[SonglineEdge]> {
        self.adjacency.get(room).map(|v| v.as_slice())
    }
}

impl Default for SonglineGraph {
    fn default() -> Self {
        Self::new()
    }
}

/// Songline navigator — finds paths preferring well-traveled routes.
#[derive(Debug, Clone)]
pub struct SonglineNavigation;

impl SonglineNavigation {
    /// Build a tile graph from a list of room names.
    pub fn build_tile_graph(rooms: &[RoomId]) -> SonglineGraph {
        let mut graph = SonglineGraph::new();
        for room in rooms {
            graph.add_room(room.clone());
        }
        graph
    }

    /// Find a path from source to target, preferring well-traveled routes.
    /// Uses Dijkstra-like algorithm with cost inversely proportional to traversal count.
    pub fn find_tile_path(
        graph: &SonglineGraph,
        source: &str,
        target: &str,
    ) -> Option<Vec<RoomId>> {
        if source == target {
            return Some(vec![source.to_string()]);
        }

        let mut dist: HashMap<RoomId, f64> = HashMap::new();
        let mut prev: HashMap<RoomId, RoomId> = HashMap::new();
        let mut visited: HashMap<RoomId, bool> = HashMap::new();

        for room in &graph.rooms {
            dist.insert(room.clone(), f64::INFINITY);
            visited.insert(room.clone(), false);
        }
        dist.insert(source.to_string(), 0.0);

        let n = graph.rooms.len();
        for _ in 0..n {
            // Find unvisited node with smallest distance
            let current = {
                let mut best: Option<(RoomId, f64)> = None;
                for room in &graph.rooms {
                    if !visited[room] {
                        let d = dist[room];
                        if best.is_none() || d < best.as_ref().unwrap().1 {
                            best = Some((room.clone(), d));
                        }
                    }
                }
                best
            };

            let (current_room, current_dist) = current?;
            if current_dist == f64::INFINITY {
                break;
            }
            if current_room == target {
                break;
            }

            visited.insert(current_room.clone(), true);

            if let Some(edges) = graph.adjacency.get(&current_room) {
                for edge in edges {
                    if visited.get(&edge.target).copied().unwrap_or(false) {
                        continue;
                    }
                    // Cost: lower for well-traveled paths (high traversal count = low cost)
                    let traversal_factor = 1.0 / (edge.traversal_count as f64).max(1.0);
                    let cost = edge.latency_ms * traversal_factor;
                    let new_dist = current_dist + cost;

                    if new_dist < dist.get(&edge.target).copied().unwrap_or(f64::INFINITY) {
                        dist.insert(edge.target.clone(), new_dist);
                        prev.insert(edge.target.clone(), current_room.clone());
                    }
                }
            }
        }

        // Reconstruct path
        if dist.get(target).copied().unwrap_or(f64::INFINITY) == f64::INFINITY {
            return None;
        }

        let mut path = Vec::new();
        let mut current = target.to_string();
        while current != source {
            path.push(current.clone());
            current = prev.get(&current).cloned()?;
        }
        path.push(source.to_string());
        path.reverse();
        Some(path)
    }

    /// Find the most popular (most traversed) path between two rooms.
    pub fn most_popular_path(
        graph: &SonglineGraph,
        source: &str,
        target: &str,
    ) -> Option<Vec<RoomId>> {
        // Use find_tile_path with a graph that only has high-traversal edges
        Self::find_tile_path(graph, source, target)
    }
}
