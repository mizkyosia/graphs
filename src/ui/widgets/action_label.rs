use eframe::egui::{self, NumExt, Response, Sense, TextStyle, Ui, Widget, WidgetText, vec2};

pub const MIN_SPACE: f32 = 5.0;

pub struct ActionLabel {
    text: WidgetText,
    shortcut: WidgetText,
}

impl ActionLabel {
    pub fn new(text: impl Into<WidgetText>, shortcut: impl Into<WidgetText>) -> Self {
        Self {
            text: text.into(),
            shortcut: {
                let mut s = shortcut.into();
                if let WidgetText::RichText(ref mut t) = s {
                    *t = t.clone().small_raised();
                }
                s
            },
        }
    }
}

impl Widget for ActionLabel {
    fn ui(self, ui: &mut Ui) -> Response {
        // Unwrap self
        let Self { text, shortcut } = self;

        // Define padding
        let button_padding = ui.spacing().button_padding;
        let total_extra = button_padding + button_padding;
        let wrap_width = ui.available_width() - total_extra.x;

        // Build the galleys from the given texts
        let text_galley = text.into_galley(ui, None, wrap_width, TextStyle::Button);
        let shortcut_galley = shortcut.into_galley(ui, None, wrap_width, TextStyle::Button);

        // Calculate real available space
        let mut desired_size =
            total_extra + text_galley.size() + vec2(shortcut_galley.size().x + MIN_SPACE, 0.0);
        desired_size.y = desired_size.y.at_least(ui.spacing().interact_size.y);

        // Allocate necessary size for the response
        let (rect, response) = ui.allocate_at_least(desired_size, Sense::click());

        // Get associated style
        let style = ui.style().interact_selectable(&response, false);

        // Position text
        let text_pos = ui
            .layout()
            .align_size_within_rect(text_galley.size(), rect.shrink2(button_padding))
            .min;

        // If interacted with, paint the button
        if response.hovered() || response.clicked() || response.has_focus() {
            let rect = rect.expand(style.expansion);
            ui.painter().rect(
                rect,
                style.corner_radius,
                style.weak_bg_fill,
                style.bg_stroke,
                egui::StrokeKind::Inside,
            );
        }

        let line_height = text_galley.size().y;

        // Paint the actual text
        ui.painter()
            .galley(text_pos, text_galley, style.text_color());
        ui.painter().galley(
            text_pos
                + vec2(
                    wrap_width - shortcut_galley.size().x,
                    (line_height - shortcut_galley.size().y) / 2.0,
                ),
            shortcut_galley,
            style.text_color().gamma_multiply(0.7),
        );

        response
    }
}
