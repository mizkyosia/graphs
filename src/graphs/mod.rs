use std::collections::HashMap;

use eframe::egui::Rect;
use ulid::Ulid;

pub mod node;
pub mod oriented;
pub use node::*;
pub use oriented::*;

pub const POINT_RADIUS: f32 = 8.0;

pub enum GraphType<W = f32>
where
    W: PartialOrd + PartialEq,
{
    Oriented(OrientedGraph<W>),
}

#[allow(dead_code)]
pub trait Graph<E, W = f32>
where
    W: PartialOrd + PartialEq,
{
    fn new(nodes: HashMap<Ulid, Node>, edges: HashMap<E, W>) -> Self;
    fn empty() -> Self;

    fn node_count(&self) -> usize;
    fn edge_count(&self) -> usize;
    fn bounding_rect(&self) -> Rect;

    fn clear(&mut self);
    fn insert(&mut self, node: Node) -> Ulid;
    fn insert_with_edges(&mut self, node: Node, edges: impl IntoIterator<Item = (E, W)>) -> Ulid;
    fn remove(&mut self, node: &Ulid) -> Option<Node>;

    fn link(&mut self, node1: &Ulid, node2: &Ulid, weight: W);
    fn neighbors_in(&self, node: &Ulid) -> Vec<Ulid>;
    fn neighbors_out(&self, node: &Ulid) -> Vec<Ulid>;

    /// Checks if a link Node1 -> Node2 exists
    fn linked(&self, node1: &Ulid, node2: &Ulid) -> bool;
    /// Tries to find a path from `start` to `end`, with the smallest weight possible
    fn djikstra(&self, start: &Ulid, end: &Ulid) -> Option<Vec<Ulid>>;
}
