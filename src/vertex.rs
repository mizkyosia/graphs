use eframe::egui::{pos2, Color32, Pos2};

#[derive(Debug, Default, Clone)]
pub struct Node {
    pub name: String,
    pub color: Color32,
    pub pos: Pos2
}

impl Node {
    pub fn new(x: f32, y: f32, color: Color32) -> Self {
        Self { color, pos: pos2(x, y), name: "Vertex :3".into() }
    }
}
