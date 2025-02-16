use eframe::egui::{self, Context, DragValue};
use egui_extras::{Column, TableBuilder};
use rand::{rng, seq::SliceRandom};
use ulid::Ulid;

use crate::{graphs::Graph, GraphDisplayer};

pub struct GraphInspector {
    pub message: String,
}

pub fn show_graph_inspector(ctx: &Context, displayer: &mut GraphDisplayer) {
    egui::SidePanel::left("Inspector").show(ctx, |ui| {
        ui.heading("Graph editor");
        ui.label(format!(
            "Graph size : {}",
            displayer.graphs[displayer.selected_graph].node_count()
        ));

        // Lists all the current graph's nodes
        ui.collapsing("Graph nodes", |ui| {
            let table = TableBuilder::new(ui)
                .striped(true)
                .columns(Column::auto(), 3);

            table
                .header(20.0, |mut header| {
                    header.col(|ui| {
                        ui.strong("Name");
                    });
                    header.col(|ui| {
                        ui.strong("Position");
                    });
                    header.col(|ui| {
                        ui.strong("Color");
                    });
                })
                .body(|body| {
                    let size = displayer.graphs[displayer.selected_graph].node_count();
                    let mut nodes = displayer.graphs[displayer.selected_graph].nodes.iter_mut();

                    body.rows(20.0, size, |mut rows| {
                        let (id, v) = nodes.next().unwrap();
                        // If the node is selected in the editor
                        let selected = displayer.selected_nodes.contains(id);

                        rows.set_selected(selected);
                        rows.col(|ui| {
                            ui.text_edit_singleline(&mut v.name);
                        });

                        rows.set_selected(selected);
                        rows.col(|ui| {
                            ui.horizontal(|ui| {
                                ui.add(DragValue::new(&mut v.pos.x));
                                ui.add(DragValue::new(&mut v.pos.y));
                            });
                        });

                        rows.set_selected(selected);
                        rows.col(|ui| {
                            ui.color_edit_button_srgba(&mut v.color);
                        });
                    });
                });
        });

        if ui.button("Color graph").clicked() {
            let mut order: Vec<Ulid> = displayer.graphs[displayer.selected_graph]
                .nodes
                .keys()
                .copied()
                .collect();
            order.shuffle(&mut rng());

            displayer.graphs[displayer.selected_graph].color(order);
        }
    });
}
