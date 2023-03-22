use crate::{hotreaload::AsyncEvent, parse::math_into_glsl};
use egui::Color32;
use std::sync::mpsc::Sender;

pub struct GuiControl {
    edit: String,
    sender: Sender<AsyncEvent>,
    error: Option<String>,
    history: Vec<String>,
    history_index: usize,
}

impl GuiControl {
    pub fn new(sender: Sender<AsyncEvent>, initial_history: &[&str]) -> Self {
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
        }
    }
    pub fn ui(&mut self, ctx: &egui::Context) {
        egui::Window::new("")
            .id(egui::Id::new("Control"))
            .fixed_pos((8.0, 8.0))
            .fixed_size((180.0, 0.0))
            .title_bar(false)
            .show(ctx, |ui| {
                let mut new_index = None;
                ui.horizontal(|ui| {
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
                    if ui.button("Prev").clicked() {
                        new_index = self.history_index.checked_sub(1);
                    }
                    if ui.button("Next").clicked() {
                        new_index = self.history_index.checked_add(1);
                    }
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
    }
}
