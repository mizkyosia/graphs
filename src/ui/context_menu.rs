use eframe::egui::{self, Align2, Context, Layout, Pos2};

use crate::{
    GraphDisplayer,
    editor::{GraphTools, actions::*},
};

#[derive(Debug)]
pub struct ContextMenu {
    pub just_opened: bool,
    pub visible: bool,
    pub position: Pos2,
}

pub fn show_context_menu(display: &mut GraphDisplayer, context: &Context) {
    if display.context_menu.visible {
        egui::Window::new("Context")
            .title_bar(false)
            .resizable(false)
            .default_width(0.0)
            .fixed_pos(display.context_menu.position)
            .pivot(Align2::LEFT_BOTTOM)
            .show(context, |ui| {
                let response = ui.response();

                if response.clicked_elsewhere() && !display.context_menu.just_opened {
                    display.context_menu.visible = false;
                    return;
                }

                ui.with_layout(
                    Layout::top_down_justified(egui::Align::LEFT),
                    |ui| match display.tool {
                        GraphTools::Look => {
                            ui.label("WIP...");
                        }
                        GraphTools::Nodes => {
                            ui.heading("Graph edit");
                            ui.indent("node_actions", |ui| {
                                if ui.selectable_label(false, "📄 Copy").clicked() {
                                    copy_nodes(display);
                                }
                                if ui.selectable_label(false, "✂ Cut").clicked() {
                                    cut_nodes(display);
                                }
                                if ui.selectable_label(false, "📋 Paste").clicked() {
                                    paste_nodes(display);
                                }
                            });

                            if !display.selected_nodes.is_empty()
                                && ui.selectable_label(false, "🗑Delete").clicked()
                            {
                                delete_nodes(display);
                            }
                        }
                        _ => {}
                    },
                );
            });

        // Now, the menu hasn't been opened just before
        display.context_menu.just_opened = false;
    }
}
