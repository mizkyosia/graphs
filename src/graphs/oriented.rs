use std::collections::{BinaryHeap, HashMap};

use eframe::egui::{Color32, Rect, pos2};
use rand::Rng;
use ulid::Ulid;

use super::{Graph, GraphWeight, Node, POINT_RADIUS};

pub struct OrientedGraph<W = i32>
where
    W: GraphWeight,
{
    pub nodes: HashMap<Ulid, Node>,
    pub edges: HashMap<(Ulid, Ulid), W>,
}

#[derive(PartialEq, Eq)]
struct NodeState<W>
where
    W: GraphWeight,
{
    pub cost: W,
    pub node: Ulid,
}

impl<W> Ord for NodeState<W>
where
    W: GraphWeight,
{
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.node.cmp(&other.node))
    }
}

impl<W> PartialOrd for NodeState<W>
where
    W: GraphWeight,
{
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl<W> Graph<W> for OrientedGraph<W>
where
    W: GraphWeight,
{
    fn new(nodes: HashMap<Ulid, Node>, edges: HashMap<(Ulid, Ulid), W>) -> Self {
        OrientedGraph { nodes, edges }
    }

    fn empty() -> Self {
        OrientedGraph {
            nodes: HashMap::new(),
            edges: HashMap::new(),
        }
    }

    fn clear(&mut self) {
        self.nodes.clear();
        self.edges.clear();
    }

    fn node_count(&self) -> usize {
        self.nodes.len()
    }

    fn edge_count(&self) -> usize {
        self.edges.len()
    }

    fn bounding_rect(&self) -> Rect {
        if self.node_count() == 0 {
            return Rect::ZERO;
        }

        let mut min = pos2(f32::INFINITY, f32::INFINITY);
        let mut max = pos2(f32::NEG_INFINITY, f32::NEG_INFINITY);

        for (_, v) in self.nodes.iter() {
            min = min.min(v.pos);
            max = max.max(v.pos);
        }
        min.x -= POINT_RADIUS * 2.0;
        min.y -= POINT_RADIUS * 2.0;
        max.x += POINT_RADIUS * 2.0;
        max.y += POINT_RADIUS * 2.0;
        Rect { min, max }
    }

    fn insert(&mut self, vertex: Node) -> Ulid {
        let id = Ulid::new();
        self.nodes.insert(id, vertex);
        id
    }

    fn insert_with_edges(
        &mut self,
        vertex: Node,
        edges: impl IntoIterator<Item = ((Ulid, Ulid), W)>,
    ) -> Ulid {
        self.edges.extend(edges);
        self.insert(vertex)
    }

    fn remove(&mut self, node: &Ulid) -> Option<Node> {
        self.edges.retain(|e, _| e.0 != *node && e.1 != *node);
        self.nodes.remove(&node)
    }

    fn link(&mut self, v1: &Ulid, v2: &Ulid, weight: W) {
        self.edges.insert((*v1, *v2), weight);
    }

    fn linked(&self, node1: &Ulid, node2: &Ulid) -> bool {
        self.edges.contains_key(&(*node1, *node2))
    }

    fn neighbors_out(&self, node: &Ulid) -> Vec<(Ulid, W)> {
        self.edges
            .iter()
            .filter_map(|(edge, w)| {
                if edge.0 == *node {
                    Some((edge.1, w.clone()))
                } else {
                    None
                }
            })
            .collect()
    }

    fn neighbors_in(&self, node: &Ulid) -> Vec<(Ulid, W)> {
        self.edges
            .iter()
            .filter_map(|(edge, w)| {
                if edge.1 == *node {
                    Some((edge.0, w.clone()))
                } else {
                    None
                }
            })
            .collect()
    }

    fn dijkstra(&self, start: &Ulid, end: &Ulid) -> Option<(Vec<Ulid>, W)> {
        let mut frontier: BinaryHeap<NodeState<W>> = BinaryHeap::new();
        let mut from: HashMap<Ulid, Ulid> = HashMap::new();
        let mut cost_so_far: HashMap<Ulid, W> = HashMap::new();

        // Add the first node in the frontier, with a weight of 0
        frontier.push(NodeState {
            cost: W::default(),
            node: *start,
        });
        cost_so_far.insert(*start, W::default());

        loop {
            // Try to get the next node in the frontier
            if let Some(current) = frontier.pop() {
                // Path found !!!!
                if current.node == *end {
                    let mut path = vec![*end];
                    let mut current = *end;

                    // Backtrack
                    while current != *start {
                        current = *from.get(&current).unwrap();
                        path.push(current);
                    }

                    // Reverse the order, so that it goes from start to end
                    path.reverse();

                    // Return value
                    break Some((path, cost_so_far.get(end).unwrap().clone()));
                } else {
                    for (n, w) in self.neighbors_out(&current.node) {
                        let new_cost = cost_so_far.get(&current.node).unwrap().clone() + w;

                        // If we don't have a cost for this node, or the cost is higher than the new one, insert the new cost
                        if let Some(cost) = cost_so_far.get_mut(&n) {
                            if new_cost < *cost {
                                *cost = new_cost.clone();
                            }
                        } else {
                            cost_so_far.insert(n, new_cost.clone());
                        }

                        // Finally, add the neighbor to the frontier
                        frontier.push(NodeState {
                            cost: new_cost,
                            node: n,
                        });

                        // Add path
                        from.insert(n, current.node);
                    }
                }
            } else {
                // No path between the 2 exists :(
                break None;
            }
        }
    }
}

impl<W> OrientedGraph<W>
where
    W: GraphWeight,
{
    pub fn color(&mut self, order: Vec<Ulid>) -> u32 {
        if order.len() != self.node_count() {
            panic!("Given order does not contain the whole graph")
        }

        let mut max_col: u32 = 1;

        let mut colors: HashMap<Ulid, u32> = HashMap::new();
        let mut generator = rand::rng();

        for i in order {
            let mut neighbor_colors: Vec<&u32> = self
                .neighbors_in(&i)
                .iter()
                .filter_map(|n| colors.get(&n.0))
                .collect();
            neighbor_colors.sort();

            let mut col = 1;
            for c in neighbor_colors {
                if *c == col {
                    col += 1;
                }
            }

            colors.insert(i, col);
            max_col = max_col.max(col);
        }

        let colors_vec: Vec<Color32> = (0..=max_col)
            .map(|_| {
                Color32::from_rgb(
                    generator.random_range(0..=255),
                    generator.random_range(0..=255),
                    generator.random_range(0..=255),
                )
            })
            .collect();

        for (v, c) in colors.iter() {
            self.nodes.get_mut(v).unwrap().color = colors_vec[*c as usize];
        }
        max_col
    }
}
