use crate::gui::{icon_button, icons};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GuiResult {
    None,
    Exit,
}

pub fn custom_window_frame(
    ctx: &egui::Context,
    window: &winit::window::Window,
    title: &str,
    title_contents: impl FnOnce(&mut egui::Ui),
    add_contents: impl FnOnce(&mut egui::Ui),
) -> GuiResult {
    use egui::*;

    let panel_frame = egui::Frame {
        fill: ctx.style().visuals.window_fill(),
        rounding: Rounding {
            nw: 10.0,
            ne: 10.0,
            sw: 0.0,
            se: 0.0,
        },
        stroke: ctx.style().visuals.widgets.noninteractive.fg_stroke,
        outer_margin: 0.5.into(), // so the stroke is within the bounds
        ..Default::default()
    };

    CentralPanel::default()
        .frame(panel_frame)
        .show(ctx, |ui| {
            let app_rect = ui.max_rect();

            let title_bar_height = 32.0;
            let title_bar_rect = {
                let mut rect = app_rect;
                rect.max.y = rect.min.y + title_bar_height;
                rect
            };
            let result = title_bar_ui(ui, window, title_bar_rect, title, title_contents);

            // Add the contents:
            let content_rect = {
                let mut rect = app_rect;
                rect.min.y = title_bar_rect.max.y;
                // fix some rounding issues
                rect.max -= egui::Vec2::splat(ctx.pixels_per_point() / 2.0);
                rect
            };

            let mut content_ui = ui.child_ui(content_rect, *ui.layout());
            add_contents(&mut content_ui);

            result
        })
        .inner
}

fn title_bar_ui(
    ui: &mut egui::Ui,
    window: &winit::window::Window,
    title_bar_rect: egui::Rect,
    title: &str,
    title_contents: impl FnOnce(&mut egui::Ui),
) -> GuiResult {
    use egui::*;

    let painter = ui.painter();

    let title_bar_response = ui.interact(
        title_bar_rect,
        Id::new("title_bar"),
        Sense::click_and_drag(),
    );

    // Paint the title:
    if !title.is_empty() {
        painter.text(
            title_bar_rect.center(),
            Align2::CENTER_CENTER,
            title,
            FontId::proportional(20.0),
            ui.style().visuals.text_color(),
        );
    }

    // Paint the line under the title:
    painter.line_segment(
        [
            title_bar_rect.left_bottom() + vec2(1.0, 0.0),
            title_bar_rect.right_bottom() + vec2(-1.0, 0.0),
        ],
        ui.visuals().widgets.noninteractive.bg_stroke,
    );

    // Interact with the title bar (drag to move window):
    if title_bar_response.double_clicked() {
        window.set_maximized(!window.is_maximized());
    } else if title_bar_response.drag_started_by(PointerButton::Primary) {
        window.drag_window();
    }

    let mut result = GuiResult::None;
    let right_rect = egui::Rect {
        min: title_bar_rect.right_top() - egui::vec2(32.0, 0.0),
        max: title_bar_rect.max,
    };
    let left_vec = egui::Rect {
        min: title_bar_rect.min,
        max: right_rect.left_bottom(),
    };
    ui.allocate_ui_at_rect(right_rect, |ui| {
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            ui.add_space(8.0);
            result = close_maximize_minimize(ui, window);
        });
    });
    ui.allocate_ui_at_rect(left_vec, |ui| {
        ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
            ui.add_space(8.0);
            title_contents(ui);
        });
    });
    result
}

/// Show some close/maximize/minimize buttons for the native window.
fn close_maximize_minimize(ui: &mut egui::Ui, window: &winit::window::Window) -> GuiResult {
    let mut result = GuiResult::None;

    if icon_button(ui, icons::CANCEL).clicked() {
        result = GuiResult::Exit;
    }

    // if window.is_maximized() {
    //     if icon_button(ui, icons::WINDOW_MINIMIZE).clicked() {
    //         window.set_maximized(false);
    //     }
    // } else {
    //     if icon_button(ui, icons::WINDOW_MAXIMIZE).clicked() {
    //         window.set_maximized(true);
    //     }
    // }

    // if icon_button(ui, icons::WINDOW_MINIMIZE).clicked() {
    //     window.set_minimized(true);
    // }

    result
}
