mod decorations;
mod icons;
mod renderer;
mod state;
mod util;

use std::{
    cell::{Cell, RefCell, RefMut},
    sync::Arc,
};

use decorations::*;
use egui::{ClippedPrimitive, TexturesDelta};
use winit::{
    event::{DeviceId, ElementState, ModifiersState, MouseButton},
    window::{ResizeDirection, Window, WindowId},
};
pub use {icons::*, renderer::*, state::*, util::*};

pub type WinitEvent<'a> = winit::event::Event<'a, ()>;

#[derive(Default, Clone, Copy, PartialEq)]
pub struct MouseState {
    pub left: bool,
    pub right: bool,
    pub middle: bool,
}
impl MouseState {
    pub fn update(&mut self, button: MouseButton, state: ElementState) {
        let pressed = state == ElementState::Pressed;
        match button {
            MouseButton::Left => self.left = pressed,
            MouseButton::Right => self.right = pressed,
            MouseButton::Middle => self.middle = pressed,
            MouseButton::Other(_) => {}
        }
    }
}

pub struct GuiFrameOutput<R> {
    pub exit_requested: bool,
    pub primitives: Vec<ClippedPrimitive>,
    pub textures_delta: TexturesDelta,
    pub inner: R,
}

struct InnerWindowState {
    collected_events: Vec<WinitEvent<'static>>,
    last_mouse_device: Option<DeviceId>,
    mouse_buttons: MouseState,
    egui_winit: egui_winit::State,
}

pub struct WindowState {
    window: winit::window::Window,
    should_exit: Cell<bool>,
    egui: Arc<egui::Context>,
    inner: RefCell<InnerWindowState>,
}

impl WindowState {
    pub fn new(window: Window, event_loop: &winit::event_loop::EventLoopWindowTarget<()>) -> Self {
        let egui = egui::Context::default();
        set_fonts(&egui);
        let mut egui_winit = egui_winit::State::new(event_loop);
        egui_winit.set_max_texture_side(8192);

        Self {
            window,
            should_exit: Default::default(),
            egui: Arc::new(egui),
            inner: RefCell::new(InnerWindowState {
                collected_events: Vec::new(),
                last_mouse_device: None,
                mouse_buttons: MouseState::default(),
                egui_winit,
            }),
        }
    }

    pub fn window(&self) -> &Window {
        &self.window
    }
    pub fn ctx(&self) -> &Arc<egui::Context> {
        &self.egui
    }
    fn inner_mut(&self) -> RefMut<InnerWindowState> {
        self.inner.borrow_mut()
    }

    pub fn pixels_per_point(&self) -> f32 {
        self.egui.pixels_per_point()
    }
    pub fn mouse_state(&self) -> MouseState {
        self.inner_mut().mouse_buttons
    }

    pub fn request_exit(&self) {
        self.should_exit.set(true);
    }
    pub fn is_maximized(&self) -> bool {
        self.window.is_maximized()
    }
    pub fn is_minimized(&self) -> Option<bool> {
        self.window.is_minimized()
    }
    pub fn set_maximized(&self, maximized: bool) {
        self.window.set_maximized(maximized);
    }
    pub fn set_minimized(&self, minimized: bool) {
        self.window.set_minimized(minimized);
    }
    pub fn start_window_drag(&self) {
        self.window.drag_window();
        self.inject_pointer_release();
    }
    pub fn start_window_drag_resize(&self, direction: ResizeDirection) {
        self.window.drag_resize_window(direction);
        self.inject_pointer_release();
    }
    fn inject_pointer_release(&self) {
        // we immediatelly inject a mouse release event since at least on wayland we
        // never receive it after beginning a drag?
        let device_id = self.inner_mut().last_mouse_device.unwrap();
        self.process_event(winit::event::Event::WindowEvent {
            window_id: self.window.id(),
            event: winit::event::WindowEvent::MouseInput {
                device_id,
                state: winit::event::ElementState::Released,
                button: winit::event::MouseButton::Left,
                modifiers: ModifiersState::empty(),
            },
        });
    }
    pub fn process_event(&self, event: WinitEvent<'_>) {
        let mut inner = self.inner_mut();
        match &event {
            WinitEvent::WindowEvent { window_id, event } => {
                let mut allow_consumed = true;
                match event {
                    winit::event::WindowEvent::MouseWheel { .. } => {
                        allow_consumed = false;
                    }
                    &winit::event::WindowEvent::MouseInput {
                        device_id,
                        state,
                        button,
                        ..
                    } => {
                        inner.last_mouse_device = Some(device_id);
                        inner.mouse_buttons.update(button, state);
                    }
                    _ => {}
                }

                let response = inner.egui_winit.on_event(&self.egui, &event);
                if response.consumed && allow_consumed {
                    return;
                }
            }
            _ => (),
        }
        if let Some(event) = event.to_static() {
            inner.collected_events.push(event);
        }
    }
    pub fn gui_frame<R, F: FnOnce(&WindowState) -> R>(&self, fun: F) -> GuiFrameOutput<R> {
        let mut inner = self.inner_mut();

        let new_input = inner.egui_winit.take_egui_input(&self.window);
        let pixels_per_point = new_input
            .pixels_per_point
            .unwrap_or(inner.egui_winit.pixels_per_point());

        drop(inner);
        self.egui.begin_frame(new_input);
        let fun_output = fun(self);
        let output = self.egui.end_frame();

        let mut inner = self.inner_mut();
        inner
            .egui_winit
            .handle_platform_output(&self.window, &self.egui, output.platform_output);

        let primitives = self.egui.tessellate(output.shapes);

        GuiFrameOutput {
            exit_requested: self.should_exit.get(),
            primitives,
            textures_delta: output.textures_delta,
            inner: fun_output,
        }
    }
    pub fn drain_events(&self) -> std::vec::IntoIter<WinitEvent> {
        std::mem::take(&mut self.inner_mut().collected_events).into_iter()
    }
}

fn set_fonts(ctx: &egui::Context) {
    let mut fonts = egui::FontDefinitions::default();

    fonts.font_data.insert(
        "MFG Labs icons".to_owned(),
        egui::FontData::from_static(FONT_BYTES),
    );

    fonts
        .families
        .entry(egui_icon_font_family())
        .or_default()
        .push("MFG Labs icons".to_owned());

    ctx.set_fonts(fonts);
}
