use eframe::{NativeOptions, egui::Color32};
use rand::{rng, seq::SliceRandom};

use editor::GraphDisplayer;
use graphs::{Graph, Node, OrientedGraph};
use ulid::Ulid;

mod editor;
pub(crate) mod graphs;
pub mod ui;

fn main() {
    // unsafe { env::set_var("RUST_BACKTRACE", "full") };
    println!("Hello, world!");

    let mut graph = OrientedGraph::<f32>::empty();

    let mut prev_id: Ulid = Ulid(0);

    for i in 0..4 {
        let cur_id = graph.insert(Node::new(
            i as f32 * 50.0 - 100.0,
            50.0 * i as f32,
            Color32::GRAY,
        ));
        if i > 0 {
            graph.link(&prev_id, &cur_id, 0.0);
        }
        prev_id = cur_id;
    }

    let mut vec: Vec<Ulid> = graph.nodes.keys().copied().collect();
    vec.shuffle(&mut rng());

    let _res = eframe::run_native(
        "Graphs :3",
        NativeOptions::default(),
        Box::new(|cc| {
            // This gives us image support:
            egui_extras::install_image_loaders(&cc.egui_ctx);

            Ok(Box::<GraphDisplayer>::new(GraphDisplayer {
                graphs: vec![graph],
                ..Default::default()
            }))
        }),
    );
}
