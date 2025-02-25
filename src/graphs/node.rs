use eframe::egui::{Color32, Pos2};

#[derive(Debug, Default, Clone)]
pub struct Node {
    pub name: String,
    pub color: Color32,
    pub pos: Pos2,
}

impl Node {
    pub fn new(pos: Pos2, color: Color32, name: impl Into<String>) -> Self {
        Self {
            color,
            pos,
            name: name.into(),
        }
    }

    pub fn at_pos(pos: Pos2) -> Self {
        Self {
            name: "Nowode :3".into(),
            color: Color32::GRAY,
            pos,
        }
    }
}
