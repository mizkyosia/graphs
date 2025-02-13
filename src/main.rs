use eframe::{NativeOptions, egui::Color32};
use rand::{rng, seq::SliceRandom};

use display::Display;
use graph::Graph;
use ulid::Ulid;
use vertex::Node;

mod display;
pub(crate) mod context_menu;
pub(crate) mod graph;
pub(crate) mod tools;
pub(crate) mod vertex;

fn main() {
    // unsafe { env::set_var("RUST_BACKTRACE", "full") };
    println!("Hello, world!");

    let mut graph = Graph::empty();

    let mut prev_id: Ulid = Ulid(0);

    for i in 0..4 {
        let cur_id = graph.insert(Node::new(
            i as f32 * 50.0 - 100.0,
            50.0 * i as f32,
            Color32::GRAY,
        ));
        if i > 0 {
            graph.link(prev_id, cur_id, true);
        }
        prev_id = cur_id;
    }

    let mut vec: Vec<Ulid> = graph.nodes.keys().map(|k| *k).collect();
    vec.shuffle(&mut rng());

    let _res = eframe::run_native(
        "Graphs :3",
        NativeOptions::default(),
        Box::new(|cc| {
            // This gives us image support:
            egui_extras::install_image_loaders(&cc.egui_ctx);

            Ok(Box::<Display>::new({
                let mut dis = Display::default();
                dis.graphs = vec![graph];
                dis
            }))
        }),
    );
}
