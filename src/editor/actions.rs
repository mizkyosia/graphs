use std::collections::HashMap;

use eframe::egui::{Pos2, vec2};
use ulid::Ulid;

use crate::graphs::Graph;

use super::GraphDisplayer;

pub fn copy_nodes(display: &mut GraphDisplayer) {
    if !display.selected_nodes.is_empty() {
        let mut id_map = HashMap::new();

        // Copy all nodes in the temporary graph
        display.temporary.nodes = display
            .selected_nodes
            .iter()
            .map(|id| {
                // Create a new ID for the copied node
                let nid = Ulid::new();
                id_map.insert(*id, nid);
                (
                    nid,
                    display.graphs[display.selected_graph]
                        .nodes
                        .get(id)
                        .unwrap()
                        .clone(),
                )
            })
            .collect();

        // Copy links between selected nodes
        display.temporary.edges = display.graphs[display.selected_graph]
            .edges
            .clone()
            .into_iter()
            .filter_map(|(e, w)| {
                id_map
                    .get(&e.0)
                    .cloned()
                    .zip(id_map.get(&e.1).cloned())
                    .zip(Some(w))
            })
            .collect();
    }
}

pub fn cut_nodes(display: &mut GraphDisplayer) {
    if !display.selected_nodes.is_empty() {
        // Copy links between selected nodes
        display.temporary.edges = display.graphs[display.selected_graph]
            .edges
            .clone()
            .into_iter()
            .filter(|(e, _)| {
                display.selected_nodes.contains(&e.0) && display.selected_nodes.contains(&e.1)
            })
            .collect();

        // Move all selected nodes to the temporary graph
        display.temporary.nodes = display
            .selected_nodes
            .drain()
            .map(|id| {
                // No need to create a new ID since this node was cut, we reuse it
                (
                    id,
                    display.graphs[display.selected_graph]
                        .nodes
                        .remove(&id)
                        .unwrap(),
                )
            })
            .collect();
    }
}

pub fn paste_nodes(display: &mut GraphDisplayer) {
    println!("Graph to paste : {:?}", display.temporary);
    // Clear selection
    display.selected_nodes.clear();

    // Move nodes
    display.graphs[display.selected_graph]
        .nodes
        .extend(display.temporary.nodes.drain().map(|(i, mut n)| {
            // Select the nodes
            display.selected_nodes.insert(i);
            // Slightly move all nodes, as to not appear on top of their originals (if placed in the same graph)
            (i, {
                n.pos += vec2(15.0, 15.0);
                n
            })
        }));

    // And copy edges
    display.graphs[display.selected_graph]
        .edges
        .extend(display.temporary.edges.drain());
}

pub fn delete_nodes(display: &mut GraphDisplayer) {
    for id in display.selected_nodes.drain() {
        display.graphs[display.selected_graph].remove(&id);
    }
    display.context_menu.visible = false;
}

pub fn link_nodes(display: &mut GraphDisplayer, all: bool, double: bool) {
    if all {
        for n1 in display.selected_nodes.iter().by_ref() {
            for n2 in display.selected_nodes.iter() {
                display.graphs[display.selected_graph].link(n1, n2, Default::default());
                if double {
                    display.graphs[display.selected_graph].link(n2, n1, Default::default());
                }
            }
        }
    } else {
        for n in display.selected_nodes.iter() {
            let pos = display.graphs[display.selected_graph]
                .nodes
                .get(n)
                .unwrap()
                .pos;
            let mut closest = Pos2::new(f32::INFINITY, f32::INFINITY);
            let mut target = Ulid(0);
            // Find closest node
            for (id, data) in display.graphs[display.selected_graph].nodes.iter() {
                if *n != *id && data.pos.distance_sq(pos) < closest.distance_sq(pos) {
                    closest = data.pos;
                    target = *id;
                }
            }
            // Link both nodes
            display.graphs[display.selected_graph].link(n, &target, Default::default());
            if double {
                display.graphs[display.selected_graph].link(&target, n, Default::default());
            }
        }
    }
}
