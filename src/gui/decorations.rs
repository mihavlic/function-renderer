use egui::CursorIcon;
use winit::window::ResizeDirection;

use crate::gui::{icon_button, icons};

use super::WindowState;

pub fn custom_window_frame(
    window: &WindowState,
    title: &str,
    title_contents: impl FnOnce(&mut egui::Ui),
    add_contents: impl FnOnce(&mut egui::Ui),
) {
    let ctx = window.ctx();
    let style = ctx.style();

    let mut stroke = style.visuals.widgets.noninteractive.fg_stroke;
    stroke.color = egui::Color32::from_gray(60);
    let mut panel_frame = egui::Frame {
        fill: style.visuals.window_fill(),
        rounding: egui::Rounding {
            nw: 10.0,
            ne: 10.0,
            sw: 0.0,
            se: 0.0,
        },
        stroke,
        inner_margin: 0.0.into(),
        outer_margin: 10.5.into(),
        shadow: egui::epaint::Shadow {
            extrusion: 10.0,
            color: if window.window().has_focus() {
                egui::Color32::from_black_alpha(150)
            } else {
                egui::Color32::from_black_alpha(80)
            },
        },
    };
    if window.window().is_maximized() {
        panel_frame.rounding = egui::Rounding::none();
        panel_frame.shadow = egui::epaint::Shadow::NONE;
        panel_frame.stroke = egui::Stroke::NONE;
        panel_frame.outer_margin = 0.0.into();
    }
    panel_frame.inner_margin = (panel_frame.stroke.width / 2.0).into();

    egui::CentralPanel::default()
        .frame(panel_frame)
        .show(ctx, |ui| {
            let app_rect = ui.max_rect();

            let title_bar_height = 32.0;
            let title_bar_rect = {
                let mut rect = app_rect;
                rect.max.y = rect.min.y + title_bar_height;
                rect
            };

            // Add the contents:
            let content_rect = {
                let mut rect = app_rect;
                rect.min.y = title_bar_rect.max.y;
                // fix some rounding issues
                rect.max -= egui::Vec2::splat(ctx.pixels_per_point().recip() / 2.0);
                rect
            };

            let mut content_ui = ui.child_ui(content_rect, *ui.layout());
            add_contents(&mut content_ui);
            title_bar_ui(ui, window, title_bar_rect, title, title_contents);

            if let Some(dir) = handle_resize_borders(window, app_rect) {
                if ui.input(|i| i.pointer.primary_clicked()) {
                    window.start_window_drag_resize(dir);
                }
            }
        });
}

fn title_bar_ui(
    ui: &mut egui::Ui,
    window: &WindowState,

    title_bar_rect: egui::Rect,
    title: &str,
    title_contents: impl FnOnce(&mut egui::Ui),
) {
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
        window.start_window_drag();
    }

    let rect_square_size = egui::Vec2::splat(32.0);
    let right_rect = egui::Rect {
        min: title_bar_rect.max - rect_square_size,
        max: title_bar_rect.max,
    };

    ui.allocate_ui_at_rect(title_bar_rect, |ui| {
        ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
            ui.add_space(8.0);
            title_contents(ui);
        });
    });
    ui.allocate_ui_at_rect(right_rect, |ui| {
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            ui.add_space(8.0);
            handle_titlebar_interaction(ui, window);
        });
    });
}

/// Show some close/maximize/minimize buttons for the native window.
fn handle_titlebar_interaction(ui: &mut egui::Ui, window: &WindowState) {
    if icon_button(ui, icons::CANCEL).clicked() {
        window.request_exit();
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
}

fn handle_resize_borders(
    window: &WindowState,
    gui_window_rect: egui::Rect,
) -> Option<ResizeDirection> {
    let rect = gui_window_rect;
    let ctx = &window.ctx();
    let Some(pos) = ctx.pointer_interact_pos() else {
        return None;
    };

    // if the cursor is inside the *content* rect, there is no resizing to be done
    if rect.contains(pos) {
        return None;
    }

    // 1 _______ 0
    //   |     |
    //   |     |
    // 2 |_____| 3

    #[rustfmt::skip]
    let corners = [
        (rect.right_top(), CursorIcon::ResizeNorthEast, ResizeDirection::NorthEast),
        (rect.left_top(), CursorIcon::ResizeNorthWest, ResizeDirection::NorthWest),
        (rect.left_bottom(), CursorIcon::ResizeSouthWest, ResizeDirection::SouthWest),
        (rect.right_bottom(), CursorIcon::ResizeSouthEast, ResizeDirection::SouthEast),
    ];

    let corner_box_half_distance = 15.0;
    for (corner, icon, dir) in corners {
        let dist = (corner - pos).abs();
        let dist = f32::max(dist.x, dist.y);
        if dist <= corner_box_half_distance {
            ctx.set_cursor_icon(icon);
            return Some(dir);
        }
    }

    let (cursor, dir) = if pos.x > rect.max.x {
        (CursorIcon::ResizeEast, ResizeDirection::East)
    } else if pos.y > rect.max.y {
        (CursorIcon::ResizeSouth, ResizeDirection::South)
    } else if pos.x < rect.min.x {
        (CursorIcon::ResizeWest, ResizeDirection::West)
    } else if pos.y < rect.min.y {
        (CursorIcon::ResizeNorth, ResizeDirection::North)
    } else {
        return None;
    };

    ctx.set_cursor_icon(cursor);
    return Some(dir);
}
