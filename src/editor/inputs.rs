use eframe::egui::{Event, InputState, Key};

use super::{GraphDisplayer, actions::*};

pub fn graph_keyboard_inputs(display: &mut GraphDisplayer, inputs: &InputState) {
    for event in inputs.events.iter() {
        match *event {
            Event::Copy => copy_nodes(display),
            Event::Cut => cut_nodes(display),
            Event::Paste(_) => paste_nodes(display),
            Event::Key { key, modifiers, .. } => match key {
                Key::A => {
                    if modifiers.command {
                        display.selected_nodes = display.graphs[display.selected_graph]
                            .nodes
                            .keys()
                            .filter_map(|id| {
                                if modifiers.shift && display.selected_nodes.contains(id) {
                                    None
                                } else {
                                    Some(*id)
                                }
                            })
                            .collect();
                    }
                }
                Key::L => link_nodes(display, inputs.modifiers.command, inputs.modifiers.shift),
                Key::Delete | Key::Backspace => delete_nodes(display),
                _ => {}
            },
            _ => {}
        }
    }
}
