//! Everything regarding the gui.
mod decorations;
mod icons;
mod renderer;
mod state;

use std::{
    cell::{Cell, RefCell, RefMut},
    sync::Arc,
};

use egui::{ClippedPrimitive, TexturesDelta};
use winit::{
    event::{DeviceId, ElementState, ModifiersState, MouseButton},
    window::{ResizeDirection, Window},
};
pub use {icons::*, renderer::*, state::*};

pub type WinitEvent<'a> = winit::event::Event<'a, ()>;

/// What mouse buttons are pressed
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

/// The output result of running the gui, see [crate::ApplicationState] which includes these fields.
pub struct GuiFrameOutput<R> {
    pub exit_requested: bool,
    pub primitives: Vec<ClippedPrimitive>,
    pub textures_delta: TexturesDelta,
    pub inner: R,
}

/// The inner state for interior mutability.
struct InnerWindowState {
    /// All the winit events collected so far, these may be injected by the application.
    collected_events: Vec<WinitEvent<'static>>,
    /// The id of the last device from which a mouse event came.
    last_mouse_device: Option<DeviceId>,
    /// Which mouse buttons are pressed.
    mouse_buttons: MouseState,
    /// Handles the integration between egui and winit.
    egui_winit: egui_winit::State,
    /// Overrides the OS-provided scaling
    override_scaling: Option<f32>,
}

/// The state of the winit window and egui.
pub struct WindowState {
    window: winit::window::Window,
    should_exit: Cell<bool>,
    egui: Arc<egui::Context>,
    inner: RefCell<InnerWindowState>,
}

impl WindowState {
    pub fn new(
        window: Window,
        override_scaling: Option<f32>,
        event_loop: &winit::event_loop::EventLoopWindowTarget<()>,
    ) -> Self {
        let egui = egui::Context::default();
        egui.tessellation_options_mut(|o| {
            // we're rendering things with msaa, this is not needed?
            o.feathering = false;
        });
        set_fonts(&egui);
        let mut egui_winit = egui_winit::State::new(event_loop);
        egui_winit.set_max_texture_side(8192);

        if let Some(scaling) = override_scaling {
            egui_winit.set_pixels_per_point(scaling);
            egui.set_pixels_per_point(scaling);
        }

        Self {
            window,
            should_exit: Default::default(),
            egui: Arc::new(egui),
            inner: RefCell::new(InnerWindowState {
                collected_events: Vec::new(),
                last_mouse_device: None,
                mouse_buttons: MouseState::default(),
                override_scaling,
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
        self.inner_mut()
            .override_scaling
            .unwrap_or_else(|| self.egui.pixels_per_point())
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
        _ = self.window.set_maximized(maximized);
    }
    pub fn set_minimized(&self, minimized: bool) {
        _ = self.window.set_minimized(minimized);
    }
    pub fn start_window_drag(&self) {
        _ = self.window.drag_window();
        self.inject_pointer_release();
    }
    /// Start an interactive resize, only works on X11 - [issue](https://github.com/rust-windowing/winit/issues/725#issuecomment-1379192997)
    pub fn start_window_drag_resize(&self, direction: ResizeDirection) {
        _ = self.window.drag_resize_window(direction);
        self.inject_pointer_release();
    }
    fn inject_pointer_release(&self) {
        // we immediately inject a mouse release event since at least on wayland we
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
    /// Append a winit event to be handled later.
    pub fn process_event(&self, event: WinitEvent<'_>) {
        let mut inner = self.inner_mut();
        match &event {
            WinitEvent::WindowEvent { event, .. } => {
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
                    &winit::event::WindowEvent::ScaleFactorChanged { .. } => {
                        // preserve the override
                        if inner.override_scaling.is_some() {
                            return;
                        }
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
    /// Run the per-frame gui logic resulting in a triangle list.
    pub fn gui_frame<R, F: FnOnce(&WindowState) -> R>(&self, fun: F) -> GuiFrameOutput<R> {
        let mut inner = self.inner_mut();

        if let Some(scaling) = inner.override_scaling {
            inner.egui_winit.set_pixels_per_point(scaling);
            self.egui.set_pixels_per_point(scaling);
        }
        let new_input = inner.egui_winit.take_egui_input(&self.window);

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

/// Set the custom icon font.
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
