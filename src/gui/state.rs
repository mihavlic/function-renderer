use crate::{
    hotreaload::AsyncEvent,
    parse::{math_into_glsl, MAX_MARGIN, MIN_MARGIN},
    FrameData,
};
use egui::{Color32, Layout, Ui, Widget};
use glam::Vec3;
use std::{
    any::TypeId,
    sync::{mpsc::Sender, Arc, Mutex},
};

fn clickable_text(ui: &mut egui::Ui, text: &str) -> egui::Response {
    egui::Label::new(egui::RichText::new(text).size(14.0))
        .sense(egui::Sense::click())
        .ui(ui)
}

trait FunctionIntervalControl: 'static {
    fn init(&mut self, min: Vec3, max: Vec3);
    fn ui(&mut self, ui: &mut Ui);
    fn output(&self) -> (Vec3, Vec3);
}

#[derive(Default)]
struct IntervalControl {
    locked: bool,
    min: Vec3,
    max: Vec3,
}

impl FunctionIntervalControl for IntervalControl {
    fn init(&mut self, min: Vec3, max: Vec3) {
        self.min = min;
        self.max = max;
    }

    fn ui(&mut self, ui: &mut Ui) {
        egui::Grid::new("interval grid").show(ui, |ui| {
            ui.checkbox(&mut self.locked, "locked");
            ui.end_row();

            let locked = self.locked;

            ui.label("min");
            ui.add(egui::DragValue::new(&mut self.min.x).speed(0.1));
            ui.add_enabled(!locked, egui::DragValue::new(&mut self.min.y).speed(0.1));
            ui.add_enabled(!locked, egui::DragValue::new(&mut self.min.z).speed(0.1));
            ui.end_row();

            ui.label("max");
            ui.add(egui::DragValue::new(&mut self.max.x).speed(0.1));
            ui.add_enabled(!locked, egui::DragValue::new(&mut self.max.y).speed(0.1));
            ui.add_enabled(!locked, egui::DragValue::new(&mut self.max.z).speed(0.1));

            if locked {
                self.max = Vec3::splat(self.max.x);
                self.min = Vec3::splat(self.min.x);
            }
        });
    }

    fn output(&self) -> (Vec3, Vec3) {
        (self.min, self.max)
    }
}

#[derive(Default)]
struct CenterControl {
    center: Vec3,
    half: f32,
}

impl FunctionIntervalControl for CenterControl {
    fn init(&mut self, min: Vec3, max: Vec3) {
        self.center = (min + max) / 2.0;
        let halves = (max - min) / 2.0;
        self.half = halves.max_element();
    }

    fn ui(&mut self, ui: &mut Ui) {
        egui::Grid::new("center grid").show(ui, |ui| {
            ui.label("pos");
            ui.add(egui::DragValue::new(&mut self.center.x).speed(0.1));
            ui.add(egui::DragValue::new(&mut self.center.y).speed(0.1));
            ui.add(egui::DragValue::new(&mut self.center.z).speed(0.1));
            if clickable_text(ui, "↺").clicked() {
                self.center = Vec3::ZERO;
            }
            ui.end_row();

            ui.label("half");
            ui.add(
                egui::DragValue::new(&mut self.half)
                    .speed(0.1)
                    .clamp_range(0.01..=f32::INFINITY),
            );
            ui.end_row();
        });
    }

    fn output(&self) -> (Vec3, Vec3) {
        (self.center - self.half, self.center + self.half)
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum ControlKind {
    Center,
    Interval,
}

pub struct GuiControl {
    edit: String,
    sender: Sender<AsyncEvent>,
    error: Option<String>,
    history: Vec<String>,
    history_index: usize,
    frame: Arc<Mutex<FrameData>>,
    control: Box<dyn FunctionIntervalControl>,
    control_id: ControlKind,
}

impl GuiControl {
    pub fn new(
        sender: Sender<AsyncEvent>,
        frame: Arc<Mutex<FrameData>>,
        initial_history: &[&str],
    ) -> Self {
        if let Some(last) = initial_history.last() {
            let (density, gradient) =
                math_into_glsl(last).unwrap_or_else(|_| math_into_glsl("-1.0").unwrap());
            sender.send(AsyncEvent::NewFunction { density, gradient });
        }

        Self {
            edit: initial_history.last().copied().unwrap_or("").to_owned(),
            sender,
            error: None,
            history: initial_history
                .iter()
                .copied()
                .map(ToOwned::to_owned)
                .collect(),
            history_index: initial_history.len().saturating_sub(1),
            frame,
            control: Box::new(CenterControl {
                center: Vec3::splat(0.0),
                half: 16.0,
            }),
            control_id: ControlKind::Center,
        }
    }
    pub fn ui(&mut self, ctx: &egui::Context) {
        egui::Window::new("")
            .id(egui::Id::new("Control"))
            .fixed_pos((8.0, 8.0))
            .auto_sized()
            .title_bar(false)
            .show(ctx, |ui| {
                let id = ui.make_persistent_id("window inner thing");
                let mut state = egui::collapsing_header::CollapsingState::load_with_default_open(
                    ui.ctx(),
                    id,
                    true,
                );

                fn circle_icon(ui: &mut egui::Ui, openness: f32, response: &egui::Response) {
                    let stroke = ui.style().interact(&response).fg_stroke;
                    let radius = egui::lerp(5.0..=8.0, openness);
                    // ui.label(egui::RichText::new("⚙").color(stroke.color).size(7.0));
                    ui.painter().text(
                        response.rect.center(),
                        egui::Align2::CENTER_CENTER,
                        "⚙",
                        egui::FontId {
                            size: 16.0,
                            family: egui::FontFamily::Proportional,
                        },
                        stroke.color,
                    );
                }

                let mut new_index = None;

                ui.horizontal(|ui| {
                    state.show_toggle_button(ui, circle_icon);
                    if state.is_open() {
                        if ui.text_edit_singleline(&mut self.edit).lost_focus() {
                            match math_into_glsl(&self.edit) {
                                Ok((density, gradient)) => {
                                    self.history_index = self.history.len();
                                    self.history.push(self.edit.clone());

                                    self.sender
                                        .send(AsyncEvent::NewFunction { density, gradient });
                                    self.error = None;
                                }
                                Err(e) => {
                                    self.error = Some(e.to_string());
                                }
                            }
                        }
                        if clickable_text(ui, "⮪").clicked() {
                            new_index = self.history_index.checked_sub(1);
                        }
                        if clickable_text(ui, "⮫").clicked() {
                            new_index = self.history_index.checked_add(1);
                        }
                    }
                });

                state.show_body_unindented(ui, |ui| {
                    ui.with_layout(Layout::top_down(egui::Align::Min), |ui| {
                        self.control.ui(ui);
                        let (min, max) = self.control.output();
                        let mut frame = self.frame.lock().unwrap();
                        // don't ask me how this works
                        let scale = (60.0 + MIN_MARGIN + MAX_MARGIN) / 60.0;
                        let scale2 = MIN_MARGIN / (60.0 + MIN_MARGIN + MAX_MARGIN);
                        let extent = max - min;
                        let scaled_min = min - scale2 * extent;
                        let scaled_max = scaled_min + scale * extent;
                        frame.rect_min = scaled_min;
                        frame.rect_max = scaled_max;
                    });

                    if let Some(new) = new_index {
                        if let Some(f) = self.history.get(new) {
                            self.edit = f.clone();
                            self.history_index = new;

                            match math_into_glsl(&self.edit) {
                                Ok((density, gradient)) => {
                                    self.sender
                                        .send(AsyncEvent::NewFunction { density, gradient });
                                    self.error = None;
                                }
                                Err(e) => {
                                    self.error = Some(e.to_string());
                                }
                            }
                        }
                    }

                    if let Some(e) = self.error.as_ref() {
                        ui.colored_label(Color32::RED, e);
                    }
                });
            });
    }
}
