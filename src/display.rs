use std::collections::HashSet;

use eframe::{
    egui::{
        self, Color32, Context, DragValue, Rect, Sense, Shape, Stroke, Vec2,
        Visuals, pos2, vec2,
    },
    emath,
};
use egui_extras::{Column, TableBuilder};
use rand::{rng, seq::SliceRandom};
use ulid::Ulid;

use crate::{
    context_menu::{ContextMenu, show_context_menu},
    graph::{Graph, POINT_RADIUS},
    tools::GraphTools,
    vertex::Node,
};

pub(crate) struct Display {
    pub graphs: Vec<Graph>,
    pub selected_graph: usize,
    pub selected_nodes: HashSet<Ulid>,
    pub copied_nodes: Vec<Node>,
    pub message: String,
    pub rect: egui::Rect,
    pub tool: GraphTools,
    pub context_menu: ContextMenu,
}

impl Default for Display {
    fn default() -> Self {
        Self {
            graphs: vec![Graph::empty()],
            selected_graph: 0,
            message: ":3".into(),
            rect: Rect::from_center_size(pos2(0.0, 0.0), vec2(100.0, 100.0)),
            selected_nodes: HashSet::new(),
            tool: GraphTools::Look,
            copied_nodes: Vec::new(),
            context_menu: ContextMenu {
                just_opened: false,
                visible: false,
                position: pos2(0.0, 0.0),
            },
        }
    }
}

impl Display {
    pub fn inspector(&mut self, ctx: &Context) {
        egui::SidePanel::left("Inspector").show(ctx, |ui| {
            ui.heading("Graph editor");
            ui.label(&self.message);
            ui.label(format!(
                "Graph size : {}",
                self.graphs[self.selected_graph].size()
            ));
            if ui.button("Add graph").clicked() {
                self.graphs.push(Graph::empty());
            }

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
                        let size = self.graphs[self.selected_graph].size();
                        let mut nodes = self.graphs[self.selected_graph].nodes.iter_mut();

                        body.rows(20.0, size, |mut rows| {
                            let (id, v) = nodes.next().unwrap();
                            let selected = self.selected_nodes.contains(id);

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
                let mut order: Vec<Ulid> = self.graphs[self.selected_graph]
                    .nodes
                    .keys()
                    .map(|k| *k)
                    .collect();
                order.shuffle(&mut rng());

                self.graphs[self.selected_graph].color(order);
            }
        });
    }

    pub fn graph_tools(&mut self, ctx: &Context) {
        egui::TopBottomPanel::top("GraphTools").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.radio_value(&mut self.tool, GraphTools::Nodes, "Edit nodes");
                ui.radio_value(&mut self.tool, GraphTools::Delete, "Delete nodes & links");
                ui.radio_value(&mut self.tool, GraphTools::Links, "Edit links");
            });
        });
    }

    pub fn graph_selector(&mut self, ctx: &Context) {
        egui::TopBottomPanel::bottom("GraphSelector").show(ctx, |ui| {
            ui.horizontal(|ui| {
                for i in 0..self.graphs.len() {
                    let mut btn = ui.button(format!("{i}"));
                    if i == self.selected_graph {
                        btn = btn.highlight();
                    }
                    if btn.clicked() {
                        self.selected_graph = i;
                    }
                }
            });
        });
    }

    pub fn plot(&mut self, ctx: &Context) {
        egui::CentralPanel::default().show(ctx, |panel| {
            let reference_rect = panel.response().rect;

            let inputs = panel.input(|i| i.clone());

            egui::Scene::new().show(panel, &mut self.rect, |ui| {
                let response = ui.response();

                let mut dragged_node = false;
                let mut clicked_node = false;

                let nodes: Vec<Shape> = self.graphs[self.selected_graph]
                    .nodes
                    .iter_mut()
                    .enumerate()
                    .map(|(i, (id, node))| {
                        let mut color: Color32 = node.color;
                        let size = Vec2::splat(2.0 * POINT_RADIUS);

                        if self.tool != GraphTools::Look {
                            let point_rect = Rect::from_center_size(node.pos, size);
                            let point_id = response.id.with(i);
                            let point_response = ui.interact(point_rect, point_id, Sense::all());

                            clicked_node = clicked_node || point_response.clicked();
                            dragged_node = dragged_node || point_response.dragged();

                            match self.tool {
                                GraphTools::Nodes => {
                                    node.pos += point_response.drag_delta();

                                    if point_response.clicked() {
                                        let already_selected = self.selected_nodes.contains(id);
                                        let len = self.selected_nodes.len();
                                        if inputs.modifiers.shift {
                                            if already_selected {
                                                self.selected_nodes.remove(id);
                                            } else {
                                                self.selected_nodes.insert(*id);
                                            }
                                        } else {
                                            self.selected_nodes.clear();
                                            if !already_selected || len > 1 {
                                                self.selected_nodes.insert(*id);
                                            }
                                        }
                                    } else if point_response.hovered() {
                                        color =
                                            color + ui.style().interact(&point_response).bg_fill;
                                    }

                                    if self.selected_nodes.contains(id) {
                                        color =
                                            color + ui.style().interact(&point_response).bg_fill;
                                    }
                                }
                                _ => {}
                            }
                        }

                        Shape::circle_filled(node.pos, POINT_RADIUS, color)
                    })
                    .collect();

                let to_screen = emath::RectTransform::from_to(response.rect, reference_rect);

                if response.secondary_clicked() {
                    self.context_menu = ContextMenu {
                        just_opened: true,
                        visible: true,
                        position: to_screen.transform_pos(response.interact_pointer_pos().unwrap()),
                    };
                } else if response.clicked() && !clicked_node {
                    self.selected_nodes.clear();
                }

                if response.dragged() {
                    // println!("Dragged !")
                }

                let lines: Vec<Shape> = self.graphs[self.selected_graph]
                    .edges
                    .iter()
                    .map(|e| {
                        Shape::line_segment(
                            [
                                self.graphs[self.selected_graph]
                                    .nodes
                                    .get(&e.0)
                                    .unwrap()
                                    .pos,
                                self.graphs[self.selected_graph]
                                    .nodes
                                    .get(&e.1)
                                    .unwrap()
                                    .pos,
                            ],
                            Stroke::new(1.0, Color32::GREEN),
                        )
                    })
                    .collect();

                let painter = ui.painter();
                painter.extend(lines);
                painter.extend(nodes);
                // painter.add(Shape::rect_filled(
                //     response.rect,
                //     0,
                //     Color32::from_rgba_premultiplied(255, 0, 0, 100),
                // ));
            });

            if self.context_menu.visible {
                show_context_menu(self, ctx);
            }
        });
    }
}

impl eframe::App for Display {
    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        ctx.set_visuals(Visuals::dark());

        self.inspector(ctx);
        self.graph_selector(ctx);
        self.graph_tools(ctx);
        self.plot(ctx);
    }
}
