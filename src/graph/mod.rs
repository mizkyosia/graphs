use std::collections::{HashMap, HashSet};

use eframe::egui::{Color32, Rect, pos2};
use rand::Rng;
use ulid::Ulid;

pub mod node;
pub use node::*;

pub const POINT_RADIUS: f32 = 8.0;

pub struct Graph {
    pub nodes: HashMap<Ulid, Node>,
    pub edges: HashSet<(Ulid, Ulid)>,
}

#[allow(dead_code)]
impl Graph {
    pub fn new(vertices: HashMap<Ulid, Node>, edges: HashSet<(Ulid, Ulid)>) -> Self {
        Graph {
            nodes: vertices,
            edges,
        }
    }

    pub fn empty() -> Self {
        Graph {
            nodes: HashMap::new(),
            edges: HashSet::new(),
        }
    }

    pub fn clear(&mut self) {
        self.nodes.clear();
        self.edges.clear();
    }

    pub fn size(&self) -> usize {
        self.nodes.len()
    }

    pub fn bounding_rect(&self) -> Rect {
        if self.size() == 0 {
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

    pub fn insert(&mut self, vertex: Node) -> Ulid {
        let id = Ulid::new();
        self.nodes.insert(id, vertex);
        id
    }

    pub fn insert_with_edges(&mut self, vertex: Node, edges: Vec<(Ulid, Ulid)>) -> Ulid {
        self.edges.extend(edges);
        self.insert(vertex)
    }

    pub fn remove(&mut self, node: Ulid) {
        self.edges.retain(|e| e.0 != node && e.1 != node);
        self.nodes.remove(&node);
    }

    pub fn link(&mut self, v1: Ulid, v2: Ulid, double: bool) {
        self.edges.insert((v1, v2));
        if double {
            self.edges.insert((v2, v1));
        }
    }

    pub fn get_neighbors(&self, vertex: Ulid) -> Vec<Ulid> {
        self.edges
            .iter()
            .filter_map(|edge| if edge.0 == vertex { Some(edge.1) } else { None })
            .collect()
    }

    pub fn color(&mut self, order: Vec<Ulid>) -> u32 {
        if order.len() != self.size() {
            panic!("Given order does not contain the whole graph")
        }

        let mut max_col: u32 = 1;

        let mut colors: HashMap<Ulid, u32> = HashMap::new();
        let mut generator = rand::rng();

        for i in order {
            let mut neighbor_colors: Vec<&u32> = self
                .get_neighbors(i)
                .iter()
                .filter_map(|n| colors.get(n))
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
