use eframe::{
    egui::{self, Color32, Context, InputState, Rect, Sense, Shape, Stroke, Vec2},
    emath,
};

use crate::{
    editor::{GraphDisplayer, GraphTools},
    graph::POINT_RADIUS,
};

use super::context_menu::ContextMenu;

pub fn plot_graph(ctx: &Context, inputs: &InputState, displayer: &mut GraphDisplayer) {
    egui::CentralPanel::default().show(ctx, |panel| {
        let reference_rect = panel.response().rect;

        let mut copied_rect = displayer.rect;

        egui::Scene::new().show(panel, &mut copied_rect, |ui| {
            let bg_response = ui.interact(
                displayer.rect,
                ui.id(),
                if displayer.tool != GraphTools::Look {
                    Sense::all()
                } else {
                    Sense::click()
                },
            );

            let mut dragged_node = false;
            let mut clicked_node = false;

            let nodes: Vec<Shape> = displayer.graphs[displayer.selected_graph]
                .nodes
                .iter_mut()
                .enumerate()
                .map(|(i, (id, node))| {
                    let mut color: Color32 = node.color;
                    let size = Vec2::splat(2.0 * POINT_RADIUS);

                    if displayer.tool != GraphTools::Look {
                        let point_rect = Rect::from_center_size(node.pos, size);
                        let point_id = bg_response.id.with(i);
                        let point_response = ui.interact(point_rect, point_id, Sense::all());

                        clicked_node = clicked_node || point_response.clicked();
                        dragged_node = dragged_node || point_response.dragged();

                        match displayer.tool {
                            GraphTools::Nodes => {
                                node.pos += point_response.drag_delta();

                                if point_response.clicked() {
                                    let already_selected = displayer.selected_nodes.contains(id);
                                    let len = displayer.selected_nodes.len();
                                    if inputs.modifiers.command {
                                        if already_selected {
                                            displayer.selected_nodes.remove(id);
                                        } else {
                                            displayer.selected_nodes.insert(*id);
                                        }
                                    } else {
                                        displayer.selected_nodes.clear();
                                        if !already_selected || len > 1 {
                                            displayer.selected_nodes.insert(*id);
                                        }
                                    }
                                } else if point_response.hovered() {
                                    color = color + ui.style().interact(&point_response).bg_fill;
                                }

                                if displayer.selected_nodes.contains(id) {
                                    color = color + ui.style().interact(&point_response).bg_fill;
                                }
                            }
                            GraphTools::Links => {
                                todo!()
                            }
                            _ => {}
                        }
                    }

                    Shape::circle_filled(node.pos, POINT_RADIUS, color)
                })
                .collect();

            let to_screen = emath::RectTransform::from_to(bg_response.rect, reference_rect);

            if bg_response.secondary_clicked() {
                displayer.context_menu = ContextMenu {
                    just_opened: true,
                    visible: true,
                    position: to_screen.transform_pos(bg_response.interact_pointer_pos().unwrap()),
                };
            } else
            // If clicked on the background, deselect all nodes
            if bg_response.clicked() && !clicked_node {
                displayer.selected_nodes.clear();
            }

            if displayer.tool != GraphTools::Look {
                if bg_response.dragged_by(egui::PointerButton::Primary) {
                    let pos = bg_response.interact_pointer_pos().unwrap();
                    // If the `Ctrl` key is not down, deselect previous nodes
                    if !inputs.modifiers.command {
                        displayer.selected_nodes.clear();
                    }

                    // If dragged started this frame, set origin
                    if bg_response.drag_started() {
                        displayer.selection_rect.min = pos;
                    }
                    displayer.selection_rect.max = pos;
                }

                // If drag ended, apply selection
                if bg_response.drag_stopped() {
                    // Create the actual rect, with the right min/max points
                    let actual_rect = Rect::from_two_pos(
                        displayer.selection_rect.min,
                        displayer.selection_rect.max,
                    );

                    // Selects all nodes in the rect, or toggles them if the `Shift` modifier is selected
                    for (id, node) in displayer.graphs[displayer.selected_graph].nodes.iter() {
                        if actual_rect.contains(node.pos) {
                            // If `Shift` is pressed, toggles the selection instead of forcing it
                            if inputs.modifiers.shift && displayer.selected_nodes.contains(id) {
                                displayer.selected_nodes.remove(id);
                            } else {
                                displayer.selected_nodes.insert(*id);
                            }
                        }
                    }

                    // Remove selection rect
                    displayer.selection_rect = Rect::ZERO;
                }
            }

            let lines: Vec<Shape> = displayer.graphs[displayer.selected_graph]
                .edges
                .iter()
                .map(|e| {
                    Shape::line_segment(
                        [
                            displayer.graphs[displayer.selected_graph]
                                .nodes
                                .get(&e.0)
                                .unwrap()
                                .pos,
                            displayer.graphs[displayer.selected_graph]
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
            painter.add(Shape::rect_filled(
                Rect::from_two_pos(displayer.selection_rect.min, displayer.selection_rect.max),
                0,
                Color32::from_rgba_premultiplied(255, 200, 0, 35),
            ));
        });

        displayer.rect = copied_rect;
    });
}
