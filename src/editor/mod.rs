pub mod actions;
pub mod inputs;

use std::collections::HashSet;

use eframe::egui::{self, Context, Pos2, Rect, Visuals, pos2, vec2};
use inputs::graph_keyboard_inputs;
use ulid::Ulid;

use crate::{
    graphs::{Graph, OrientedGraph},
    ui::{self, context_menu::*, inspector::GraphInspector},
};

#[derive(Debug, PartialEq, Eq, Default)]
pub enum GraphTools {
    #[default]
    Look,
    Nodes,
    Links,
}

pub struct GraphDisplayer {
    pub graphs: Vec<OrientedGraph>,
    pub selected_graph: usize,
    pub selected_nodes: HashSet<Ulid>,
    pub temporary: OrientedGraph,
    pub rect: egui::Rect,
    pub tool: GraphTools,
    pub selection_rect: Rect,
    pub context_menu: ContextMenu,
    pub inspector: GraphInspector,
    pub last_hovered_position: Pos2,
}

impl Default for GraphDisplayer {
    fn default() -> Self {
        Self {
            graphs: vec![OrientedGraph::empty()],
            selected_graph: 0,
            rect: Rect::from_center_size(pos2(0.0, 0.0), vec2(1000.0, 1000.0)),
            selected_nodes: HashSet::new(),
            temporary: OrientedGraph::empty(),
            tool: GraphTools::Look,
            selection_rect: Rect::ZERO,
            context_menu: ContextMenu {
                just_opened: false,
                visible: false,
                position: pos2(0.0, 0.0),
            },
            inspector: GraphInspector {
                message: ":3".into(),
            },
            last_hovered_position: Pos2::default(),
        }
    }
}

impl eframe::App for GraphDisplayer {
    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        ctx.set_visuals(Visuals::dark());

        let inputs = ctx.input(|i| i.clone());

        // Register & apply keyboard inputs
        graph_keyboard_inputs(self, &inputs);

        // Show side-panel inspector
        ui::inspector::show_graph_inspector(ctx, self);

        // Show miscellaneous graph tools selection
        ui::misc::show_graph_selector(ctx, self);
        ui::misc::show_graph_tools(ctx, &mut self.tool);

        // Plot the actual graph onto the frame
        ui::plot::plot_graph(ctx, &inputs, self);

        // Show the context (right-click) menu
        show_context_menu(self, ctx);
    }
}
