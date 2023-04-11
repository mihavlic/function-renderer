use crate::{
    gui::egui_icon_font_family,
    hotreaload::AsyncEvent,
    parse::{parse_math, MAX_MARGIN, MIN_MARGIN},
    ApplicationState,
};
use egui::{Color32, Ui, Widget};
use glam::Vec3;
use graph::storage::DefaultAhashRandomstate;
use std::{
    hash::{BuildHasher, Hash, Hasher},
    sync::{mpsc::Sender, Arc, Mutex},
};

use super::{decorations::custom_window_frame, icons, WindowState};

pub fn icon_button(ui: &mut egui::Ui, icon: char) -> egui::Response {
    egui::Label::new(icon_text(icon, 12.5))
        .sense(egui::Sense::click())
        .ui(ui)
}

pub fn icon_text(icon: char, size: f32) -> egui::RichText {
    egui::RichText::new(icon).font(egui::FontId::new(size, egui_icon_font_family()))
}

#[derive(Default)]
struct CenterControl {
    density: bool,
    center: Vec3,
    half: f32,
}

impl CenterControl {
    fn ui(&mut self, ui: &mut Ui, sender: &Sender<AsyncEvent>) {
        egui::Grid::new("center grid")
            .min_col_width(0.0)
            // .spacing((5.5, 3.0))
            .show(ui, |ui| {
                ui.label("pos");
                egui::DragValue::new(&mut self.center.x).speed(0.1).ui(ui);
                egui::DragValue::new(&mut self.center.y).speed(0.1).ui(ui);
                egui::DragValue::new(&mut self.center.z).speed(0.1).ui(ui);
                if icon_button(ui, icons::CCW).clicked() {
                    self.center = Vec3::ZERO;
                }
                ui.end_row();

                ui.label("half");
                egui::DragValue::new(&mut self.half)
                    .speed(0.1)
                    .clamp_range(0.01..=f32::INFINITY)
                    .ui(ui);

                ui.allocate_space(egui::Vec2::ZERO);
                ui.allocate_space(egui::Vec2::ZERO);

                if icon_button(ui, icons::CCW).clicked() {
                    self.half = 16.0;
                }
                ui.end_row();
            });
        if ui.checkbox(&mut self.density, "thickness").changed() {
            _ = sender.send(AsyncEvent::GenerateThickness(self.density));
        }
    }

    fn output(&self) -> (Vec3, Vec3) {
        (self.center - self.half, self.center + self.half)
    }
}

pub struct GuiOuput {
    pub inner_size: [u32; 2],
    pub drag_delta: egui::Vec2,
    pub save_requested: bool,
}

pub struct GuiControl {
    prev_finished_edit_hash: Option<u64>,
    edit: String,
    sender: Sender<AsyncEvent>,
    error: Option<String>,
    history: Vec<String>,
    history_index: usize,
    control: CenterControl,
    open_settings: bool,

    state: Arc<Mutex<ApplicationState>>,
}

impl GuiControl {
    pub fn new(
        sender: Sender<AsyncEvent>,
        state: Arc<Mutex<ApplicationState>>,
        initial_history: &[&str],
    ) -> Self {
        if let Some(last) = initial_history.last() {
            let expr = if parse_math(last).is_ok() {
                last
            } else {
                "-1.0"
            };

            _ = sender.send(AsyncEvent::NewFunction(expr.to_owned()));
        }

        Self {
            prev_finished_edit_hash: None,
            edit: initial_history.last().copied().unwrap_or("").to_owned(),
            sender,
            error: None,
            history: initial_history
                .iter()
                .copied()
                .map(ToOwned::to_owned)
                .collect(),
            history_index: initial_history.len().saturating_sub(1),
            state,
            control: CenterControl {
                center: Vec3::new(0.0, 0.0, 16.0506),
                half: 16.0,
                density: true,
            },
            open_settings: false,
        }
    }
    pub fn ui(&mut self, window: &WindowState) -> GuiOuput {
        let mut new_index = None;
        let mut size = None;
        let mut drag = egui::Vec2::ZERO;
        let mut cog_corner = egui::Pos2::ZERO;
        let mut save_requested = false;

        custom_window_frame(
            window,
            "",
            |ui| {
                let all = ui.available_rect_before_wrap();
                let rect_square_size = egui::Vec2::splat(32.0);
                let left_rect = egui::Rect {
                    min: all.min,
                    max: all.min + rect_square_size,
                };
                let middle = all.shrink2((32.0, 5.0).into());

                ui.allocate_ui_at_rect(left_rect, |ui| {
                    let cog_response = icon_button(ui, icons::COG);
                    cog_corner = cog_response.rect.left_bottom();
                    if cog_response.clicked() {
                        self.open_settings ^= true;
                    }

                    if icon_button(ui, icons::DOWNLOAD).clicked() {
                        save_requested = true;
                    }
                });

                ui.allocate_ui_at_rect(middle, |ui| {
                    ui.with_layout(
                        egui::Layout::left_to_right(egui::Align::Center).with_cross_justify(true),
                        |ui| {
                            if self.error.is_some() {
                                // make the text outline red if there is an error
                                ui.style_mut().visuals.selection.stroke.color = Color32::LIGHT_RED;
                            }

                            ui.add_space((middle.width() - 320.0) / 2.0);
                            let id = egui::Id::new("function_text_input");
                            if egui::TextEdit::singleline(&mut self.edit)
                                .id(id)
                                .font(egui::TextStyle::Monospace)
                                .vertical_align(egui::Align::Center)
                                .show(ui)
                                .response
                                .lost_focus()
                            {
                                // if we lost focus due to the user pressing enter, take the focus again
                                // since they aren't probably finished with editing
                                if ui.input(|i| i.key_pressed(egui::Key::Enter)) {
                                    ui.memory_mut(|m| m.request_focus(id));
                                }

                                let hash = {
                                    let mut state = DefaultAhashRandomstate.build_hasher();
                                    self.edit.trim().hash(&mut state);
                                    state.finish()
                                };

                                // do not update the function unless it actually changed (Response::changed doesn't work for TextEdit)
                                if Some(hash) != self.prev_finished_edit_hash {
                                    self.prev_finished_edit_hash = Some(hash);

                                    match parse_math(&self.edit) {
                                        Ok(_) => {
                                            self.history_index = self.history.len();
                                            self.history.push(self.edit.clone());

                                            _ = self
                                                .sender
                                                .send(AsyncEvent::NewFunction(self.edit.clone()));
                                            self.error = None;
                                        }
                                        Err(e) => {
                                            self.error = Some(e.to_string());
                                            self.open_settings = true;
                                        }
                                    }
                                }
                            }
                            if icon_button(ui, icons::LEFT).clicked() {
                                new_index = self.history_index.checked_sub(1);
                            }
                            if icon_button(ui, icons::RIGHT).clicked() {
                                new_index = self.history_index.checked_add(1);
                            }
                        },
                    );
                });
            },
            |ui| {
                let all = ui.available_rect_before_wrap();
                let remaining = egui::Rect {
                    min: ui.painter().round_pos_to_pixels(all.min),
                    max: ui.painter().round_pos_to_pixels(all.max),
                };
                size = Some(remaining);
                let mut rect = egui::Mesh::with_texture(egui::TextureId::User(0));
                rect.add_rect_with_uv(
                    remaining,
                    egui::Rect::from_x_y_ranges(0.0..=1.0, 0.0..=1.0),
                    egui::Color32::WHITE,
                );
                ui.painter().add(rect);

                let response = ui.interact(
                    all,
                    egui::Id::new("drag_area"),
                    egui::Sense::click_and_drag(),
                );
                drag = response.drag_delta();
            },
        );

        let ctx = window.ctx();
        let mut state = egui::collapsing_header::CollapsingState::load_with_default_open(
            ctx,
            egui::Id::new("settings_popup"),
            true,
        );
        state.set_open(self.open_settings);
        let openness = state.openness(ctx);

        let frame = egui::containers::Frame {
            stroke: egui::Stroke::NONE,
            shadow: egui::epaint::Shadow {
                extrusion: 8.0,
                color: egui::Color32::from_rgba_premultiplied(0, 0, 0, 50),
            },
            ..egui::Frame::window(&ctx.style())
        }
        .multiply_with_opacity(openness);

        if openness > 0.01 {
            egui::Window::new("")
                .id(egui::Id::new("settings"))
                .fixed_pos(cog_corner + egui::vec2(-3.5, 13.0))
                .auto_sized()
                .title_bar(false)
                .open(&mut self.open_settings)
                .frame(frame)
                .show(ctx, |ui| {
                    if let Some(e) = self.error.as_ref() {
                        ui.colored_label(ui.style().visuals.error_fg_color, e);
                    }
                    state.show_body_unindented(ui, |ui| {
                        self.control.ui(ui, &self.sender);
                    });
                });
        }

        if let Some(new) = new_index {
            if let Some(f) = self.history.get(new) {
                self.edit = f.clone();
                self.history_index = new;

                match parse_math(&self.edit) {
                    Ok(_) => {
                        _ = self.sender.send(AsyncEvent::NewFunction(self.edit.clone()));
                        self.error = None;
                    }
                    Err(e) => {
                        self.error = Some(e.to_string());
                    }
                }
            }
        }

        {
            let (min, max) = self.control.output();
            let mut frame = self.state.lock().unwrap();
            // don't ask me how this works
            let scale = (60.0 + MIN_MARGIN + MAX_MARGIN) / 60.0;
            let scale2 = MIN_MARGIN / (60.0 + MIN_MARGIN + MAX_MARGIN);
            let extent = max - min;
            let scaled_min = min - scale2 * extent;
            let scaled_max = scaled_min + scale * extent;
            frame.rect_min = scaled_min;
            frame.rect_max = scaled_max;
        }

        let size = size.unwrap().size();
        let size = [size.x.round() as u32, size.y.round() as u32];

        GuiOuput {
            inner_size: size,
            drag_delta: drag,
            save_requested,
        }
    }
}
