use eframe::egui::{self, Context};

use crate::{
    editor::{GraphDisplayer, GraphTools},
    graphs::{Graph, OrientedGraph},
};

pub fn show_graph_tools(ctx: &Context, tool: &mut GraphTools) {
    // Lists the different available graph tools
    egui::TopBottomPanel::top("GraphTools")
        .min_height(30.0)
        .show(ctx, |ui| {
            ui.horizontal_centered(|ui| {
                ui.selectable_value(tool, GraphTools::Look, "👁 Look");
                ui.selectable_value(tool, GraphTools::Nodes, "🟢 Edit nodes");
                ui.selectable_value(tool, GraphTools::Links, "🔗 Edit links");
            });
        });
}

pub fn show_graph_selector(ctx: &Context, displayer: &mut GraphDisplayer) {
    egui::TopBottomPanel::bottom("GraphSelector").show(ctx, |ui| {
        ui.horizontal(|ui| {
            for i in 0..displayer.graphs.len() {
                let mut btn = ui.button(format!("{i}"));
                if i == displayer.selected_graph {
                    btn = btn.highlight();
                }
                if btn.clicked() {
                    displayer.selected_graph = i;
                }
            }

            // Add a new empty graph to the list
            if ui.button("+").clicked() {
                displayer.graphs.push(OrientedGraph::empty());
            }
        });
    });
}
