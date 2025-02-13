use eframe::egui::{self, vec2, Align2, Context, Layout, Pos2};

use crate::{tools::GraphTools, Display};

pub(crate) struct ContextMenu {
    pub just_opened: bool,
    pub visible: bool,
    pub position: Pos2,
}

pub fn show_context_menu(display: &mut Display, context: &Context) {
    egui::Window::new("Context")
    .title_bar(false)
    .resizable(false)
    .fixed_pos(display.context_menu.position)
    .pivot(Align2::LEFT_BOTTOM)
    .show(context, |ui| {
        let response = ui.response();

        if response.clicked_elsewhere() && !display.context_menu.just_opened {
            display.context_menu.visible = false;
            return;
        }

        ui.with_layout(Layout::top_down_justified(egui::Align::LEFT), |ui| {
            match display.tool {
                GraphTools::Look => {
                    ui.label("WIP...");
                }
                GraphTools::Nodes => {
                    ui.heading("Graph edit");
                    ui.indent("node_actions", |ui| {
                        if ui.selectable_label(false, "📄 Copy").clicked()
                            && !display.selected_nodes.is_empty()
                        {
                            display.copied_nodes = display
                                .selected_nodes
                                .iter()
                                .filter_map(|id| {
                                    display.graphs[display.selected_graph]
                                        .nodes
                                        .get(&id)
                                        .cloned()
                                })
                                .collect();
                        }
                        if ui.selectable_label(false, "✂ Cut").clicked()
                            && !display.selected_nodes.is_empty()
                        {
                            display.copied_nodes = display
                                .selected_nodes
                                .drain()
                                .filter_map(|id| {
                                    display.graphs[display.selected_graph]
                                        .nodes
                                        .remove(&id)
                                })
                                .collect();
                        }
                        if ui.selectable_label(false, "📋 Paste").clicked() {
                            for mut n in display.copied_nodes.drain(..) {
                                n.pos += vec2(0.5, 0.5);
                                display.graphs[display.selected_graph].insert(n);
                            }
                        }
                    });

                    if !display.selected_nodes.is_empty() {
                        if ui.selectable_label(false, "🗑Delete").clicked() {
                            for id in display.selected_nodes.drain() {
                                display.graphs[display.selected_graph].remove(id);
                            }
                            display.context_menu.visible = false;
                        }
                    }
                }
                _ => {}
            }
        });
    });
}
