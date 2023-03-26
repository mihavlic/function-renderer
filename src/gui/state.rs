use crate::{
    gui::egui_icon_font_family,
    hotreaload::AsyncEvent,
    parse::{math_into_glsl, parse_math, MAX_MARGIN, MIN_MARGIN},
    FrameData,
};
use egui::{Color32, Layout, Ui, Widget};
use glam::Vec3;
use std::{
    any::TypeId,
    sync::{mpsc::Sender, Arc, Mutex},
};

use super::icons;

fn icon_button(ui: &mut egui::Ui, icon: char) -> egui::Response {
    egui::Label::new(icon_text(icon, 12.5))
        .sense(egui::Sense::click())
        .ui(ui)
}

fn icon_text(icon: char, size: f32) -> egui::RichText {
    egui::RichText::new(icon).font(egui::FontId::new(size, egui_icon_font_family()))
}

trait FunctionIntervalControl: 'static {
    fn init(&mut self, min: Vec3, max: Vec3);
    fn ui(&mut self, ui: &mut Ui);
    fn output(&self) -> (Vec3, Vec3);
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

    open_settings: bool,
}

impl GuiControl {
    pub fn new(
        sender: Sender<AsyncEvent>,
        frame: Arc<Mutex<FrameData>>,
        initial_history: &[&str],
    ) -> Self {
        if let Some(last) = initial_history.last() {
            let expr = if parse_math(last).is_ok() {
                last
            } else {
                "-1.0"
            };

            sender.send(AsyncEvent::NewFunction(expr.to_owned()));
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
            open_settings: false,
        }
    }
    pub fn ui(&mut self, ctx: &egui::Context) {
        fn gear_icon(ui: &mut egui::Ui, openness: f32, response: &egui::Response) {
            let stroke = ui.style().interact(&response).fg_stroke;
            ui.painter().text(
                response.rect.center(),
                egui::Align2::CENTER_CENTER,
                super::icons::COG,
                egui::FontId {
                    size: 15.0,
                    family: egui_icon_font_family(),
                },
                stroke.color,
            );
        }

        let mut new_index = None;
        egui::TopBottomPanel::new(egui::panel::TopBottomSide::Top, "title bar").show_animated(
            ctx,
            true,
            |ui| {
                ui.horizontal(|ui| {
                    if egui::Label::new(icon_text(icons::COG, 12.5))
                        .sense(egui::Sense::click())
                        .ui(ui)
                        .clicked()
                    {
                        self.open_settings ^= true;
                    }

                    if egui::TextEdit::singleline(&mut self.edit)
                        .font(egui::TextStyle::Monospace)
                        .show(ui)
                        .response
                        .lost_focus()
                    {
                        match parse_math(&self.edit) {
                            Ok(_) => {
                                self.history_index = self.history.len();
                                self.history.push(self.edit.clone());

                                self.sender.send(AsyncEvent::NewFunction(self.edit.clone()));
                                self.error = None;
                            }
                            Err(e) => {
                                self.error = Some(e.to_string());
                            }
                        }
                    }
                    if icon_button(ui, icons::LEFT).clicked() {
                        new_index = self.history_index.checked_sub(1);
                    }
                    if icon_button(ui, icons::RIGHT).clicked() {
                        new_index = self.history_index.checked_add(1);
                    }
                });
            },
        );

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
        .multiply_with_opacity(state.openness(ctx));

        if state.openness(ctx) > 0.01 {
            egui::Window::new("")
                .id(egui::Id::new("settings"))
                .anchor(egui::Align2::LEFT_TOP, (4.0, 4.0))
                .auto_sized()
                .title_bar(false)
                .frame(frame)
                .open(&mut self.open_settings)
                .show(ctx, |ui| {
                    state.show_body_unindented(ui, |ui| {
                        ui.with_layout(Layout::top_down(egui::Align::Min), |ui| {
                            self.control.ui(ui);
                        });

                        if let Some(e) = self.error.as_ref() {
                            ui.colored_label(Color32::RED, e);
                        }
                    });
                });
        }

        if let Some(new) = new_index {
            if let Some(f) = self.history.get(new) {
                self.edit = f.clone();
                self.history_index = new;

                match parse_math(&self.edit) {
                    Ok(_) => {
                        self.sender.send(AsyncEvent::NewFunction(self.edit.clone()));
                        self.error = None;
                    }
                    Err(e) => {
                        self.error = Some(e.to_string());
                    }
                }
            }
        }

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
    }
}
