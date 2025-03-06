use std::{collections::HashMap, fmt::Debug, ops::Add};

use eframe::egui::Rect;
use ulid::Ulid;

pub mod node;
pub mod oriented;
pub use node::*;
pub use oriented::*;

pub const POINT_RADIUS: f32 = 8.0;

// Horrendous trait alias implementation because FUCK BOILERPLATES
pub trait GraphWeight:
    Ord + PartialOrd + PartialEq + Default + Clone + Add<Output = Self> + Debug
{
}
impl<T: Ord + PartialOrd + PartialEq + Default + Clone + Add<Output = Self> + Debug> GraphWeight
    for T
{
}

pub struct EdgeData {
    cost: f32,
    capacity: i32,
}

pub enum GraphEdge {
    Simple(EdgeData),
    Multiple(Vec<EdgeData>),
}

pub enum GraphType {
    Oriented(OrientedGraph),
    MultiOriented,
}

#[allow(dead_code)]
pub trait Graph<W = i32>
where
    W: GraphWeight,
{
    fn new(nodes: HashMap<Ulid, Node>, edges: HashMap<(Ulid, Ulid), W>) -> Self;
    fn empty() -> Self;

    fn node_count(&self) -> usize;
    fn edge_count(&self) -> usize;
    fn bounding_rect(&self) -> Rect;

    fn clear(&mut self);
    fn insert(&mut self, node: Node) -> Ulid;
    fn insert_with_edges(
        &mut self,
        node: Node,
        edges: impl IntoIterator<Item = ((Ulid, Ulid), W)>,
    ) -> Ulid;
    fn remove(&mut self, node: &Ulid) -> Option<Node>;

    fn link(&mut self, node1: &Ulid, node2: &Ulid, weight: W);
    fn neighbors_in(&self, node: &Ulid) -> Vec<(Ulid, W)>;
    fn neighbors_out(&self, node: &Ulid) -> Vec<(Ulid, W)>;

    /// Checks if a link Node1 -> Node2 exists
    fn linked(&self, node1: &Ulid, node2: &Ulid) -> bool;
    /// Tries to find a path from `start` to `end`, with the smallest weight possible. Returns the path, along with its total weight
    fn dijkstra(&self, start: &Ulid, end: &Ulid) -> Option<(Vec<Ulid>, W)>;
}
