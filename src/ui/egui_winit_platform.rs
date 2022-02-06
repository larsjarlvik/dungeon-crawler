//! A platform integration to use [egui](https://github.com/emilk/egui) with [winit](https://github.com/rust-windowing/winit).
//!
//! You need to create a [`Platform`] and feed it with `winit::event::Event` events.
//! Use `begin_frame()` and `end_frame()` to start drawing the egui UI.
//! A basic usage example can be found [here](https://github.com/hasenbanck/egui_example).
#![warn(missing_docs)]

#[cfg(feature = "clipboard")]
use copypasta::{ClipboardContext, ClipboardProvider};
use egui::{
    emath::{pos2, vec2},
    epaint::ClippedShape,
    CtxRef, Key, Pos2,
};
use winit::{
    dpi::PhysicalSize,
    event::{Event, ModifiersState, TouchPhase, VirtualKeyCode, VirtualKeyCode::*, WindowEvent::*},
    window::CursorIcon,
};

/// Configures the creation of the `Platform`.
#[derive(Debug, Default)]
pub struct PlatformDescriptor {
    /// Width of the window in physical pixel.
    pub physical_width: u32,
    /// Height of the window in physical pixel.
    pub physical_height: u32,
    /// HiDPI scale factor.
    pub scale_factor: f64,
    /// Egui font configuration.
    pub font_definitions: egui::FontDefinitions,
    /// Egui style configuration.
    pub style: egui::Style,
}

#[cfg(feature = "webbrowser")]
fn handle_links(output: &egui::Output) {
    if let Some(open_url) = &output.open_url {
        // This does not handle open_url.new_tab
        // webbrowser does not support web anyway
        if let Err(err) = webbrowser::open(&open_url.url) {
            eprintln!("Failed to open url: {}", err);
        }
    }
}

#[cfg(feature = "clipboard")]
fn handle_clipboard(output: &egui::Output, clipboard: Option<&mut ClipboardContext>) {
    if !output.copied_text.is_empty() {
        if let Some(clipboard) = clipboard {
            if let Err(err) = clipboard.set_contents(output.copied_text.clone()) {
                eprintln!("Copy/Cut error: {}", err);
            }
        }
    }
}

/// Provides the integration between egui and winit.
pub struct Platform {
    scale_factor: f64,
    context: CtxRef,
    pub raw_input: egui::RawInput,
    modifier_state: ModifiersState,
    pointer_pos: Option<egui::Pos2>,

    #[cfg(feature = "clipboard")]
    clipboard: Option<ClipboardContext>,
}

impl Platform {
    /// Creates a new `Platform`.
    pub fn new(descriptor: PlatformDescriptor) -> Self {
        let context = CtxRef::default();

        context.set_fonts(descriptor.font_definitions.clone());
        context.set_style(descriptor.style);
        let raw_input = egui::RawInput {
            pixels_per_point: Some(descriptor.scale_factor as f32),
            screen_rect: Some(egui::Rect::from_min_size(
                Pos2::default(),
                vec2(descriptor.physical_width as f32, descriptor.physical_height as f32) / descriptor.scale_factor as f32,
            )),
            ..Default::default()
        };

        Self {
            scale_factor: descriptor.scale_factor,
            context,
            raw_input,
            modifier_state: winit::event::ModifiersState::empty(),
            pointer_pos: Some(Pos2::default()),
            #[cfg(feature = "clipboard")]
            clipboard: ClipboardContext::new().ok(),
        }
    }

    /// Handles the given winit event and updates the egui context. Should be called before starting a new frame with `start_frame()`.
    pub fn handle_event<T>(&mut self, winit_event: &Event<T>) {
        match winit_event {
            Event::WindowEvent {
                window_id: _window_id,
                event,
            } => match event {
                // Resize with 0 width and height is used by winit to signal a minimize event on Windows.
                // See: https://github.com/rust-windowing/winit/issues/208
                // There is nothing to do for minimize events, so it is ignored here. This solves an issue where
                // egui window positions would be changed when minimizing on Windows.
                Resized(PhysicalSize { width: 0, height: 0 }) => {}
                Resized(physical_size) => {
                    self.raw_input.screen_rect = Some(egui::Rect::from_min_size(
                        Default::default(),
                        vec2(physical_size.width as f32, physical_size.height as f32) / self.scale_factor as f32,
                    ));
                }
                ScaleFactorChanged {
                    scale_factor,
                    new_inner_size,
                } => {
                    self.scale_factor = *scale_factor;
                    self.raw_input.pixels_per_point = Some(*scale_factor as f32);
                    self.raw_input.screen_rect = Some(egui::Rect::from_min_size(
                        Default::default(),
                        vec2(new_inner_size.width as f32, new_inner_size.height as f32) / self.scale_factor as f32,
                    ));
                }
                MouseInput { state, button, .. } => {
                    if let winit::event::MouseButton::Other(..) = button {
                    } else {
                        // push event only if the cursor is inside the window
                        if let Some(pointer_pos) = self.pointer_pos {
                            self.raw_input.events.push(egui::Event::PointerButton {
                                pos: pointer_pos,
                                button: match button {
                                    winit::event::MouseButton::Left => egui::PointerButton::Primary,
                                    winit::event::MouseButton::Right => egui::PointerButton::Secondary,
                                    winit::event::MouseButton::Middle => egui::PointerButton::Middle,
                                    winit::event::MouseButton::Other(_) => unreachable!(),
                                },
                                pressed: *state == winit::event::ElementState::Pressed,
                                modifiers: Default::default(),
                            });
                        }
                    }
                }
                Touch(touch) => {
                    let pressed = match touch.phase {
                        TouchPhase::Started => true,
                        _ => false,
                    };

                    let pointer_pos = pos2(
                        touch.location.x as f32 / self.scale_factor as f32,
                        touch.location.y as f32 / self.scale_factor as f32,
                    );

                    self.raw_input.events.push(egui::Event::PointerButton {
                        pos: pointer_pos,
                        button: egui::PointerButton::Primary,
                        pressed,
                        modifiers: Default::default(),
                    });
                }
                MouseWheel { delta, .. } => {
                    let mut delta = match delta {
                        winit::event::MouseScrollDelta::LineDelta(x, y) => {
                            let line_height = 8.0; // TODO as in egui_glium
                            vec2(*x, *y) * line_height
                        }
                        winit::event::MouseScrollDelta::PixelDelta(delta) => vec2(delta.x as f32, delta.y as f32),
                    };
                    if cfg!(target_os = "macos") {
                        // See https://github.com/rust-windowing/winit/issues/1695 for more info.
                        delta.x *= -1.0;
                    }

                    // The ctrl (cmd on macos) key indicates a zoom is desired.
                    if self.raw_input.modifiers.ctrl || self.raw_input.modifiers.command {
                        self.raw_input.events.push(egui::Event::Zoom((delta.y / 200.0).exp()));
                    } else {
                        self.raw_input.events.push(egui::Event::Scroll(delta));
                    }
                }
                CursorMoved { position, .. } => {
                    let pointer_pos = pos2(
                        position.x as f32 / self.scale_factor as f32,
                        position.y as f32 / self.scale_factor as f32,
                    );
                    self.pointer_pos = Some(pointer_pos);
                    self.raw_input.events.push(egui::Event::PointerMoved(pointer_pos));
                }
                CursorLeft { .. } => {
                    self.pointer_pos = None;
                    self.raw_input.events.push(egui::Event::PointerGone);
                }
                ModifiersChanged(input) => {
                    self.modifier_state = *input;
                    self.raw_input.modifiers = winit_to_egui_modifiers(*input);
                }
                KeyboardInput { input, .. } => {
                    if let Some(virtual_keycode) = input.virtual_keycode {
                        let pressed = input.state == winit::event::ElementState::Pressed;
                        let ctrl = self.modifier_state.ctrl();

                        match (pressed, ctrl, virtual_keycode) {
                            (true, true, VirtualKeyCode::C) => self.raw_input.events.push(egui::Event::Copy),
                            (true, true, VirtualKeyCode::X) => self.raw_input.events.push(egui::Event::Cut),
                            (true, true, VirtualKeyCode::V) =>
                            {
                                #[cfg(feature = "clipboard")]
                                if let Some(ref mut clipboard) = self.clipboard {
                                    if let Ok(contents) = clipboard.get_contents() {
                                        self.raw_input.events.push(egui::Event::Text(contents))
                                    }
                                }
                            }
                            _ => {
                                if let Some(key) = winit_to_egui_key_code(virtual_keycode) {
                                    self.raw_input.events.push(egui::Event::Key {
                                        key,
                                        pressed,
                                        modifiers: winit_to_egui_modifiers(self.modifier_state),
                                    });
                                }
                            }
                        }
                    }
                }
                ReceivedCharacter(ch) => {
                    if is_printable(*ch) && !self.modifier_state.ctrl() && !self.modifier_state.logo() {
                        self.raw_input.events.push(egui::Event::Text(ch.to_string()));
                    }
                }
                _ => {}
            },
            Event::DeviceEvent { .. } => {}
            _ => {}
        }
    }

    /// Updates the internal time for egui used for animations. `elapsed_seconds` should be the seconds since some point in time (for example application start).
    pub fn update_time(&mut self, elapsed_seconds: f64) {
        self.raw_input.time = Some(elapsed_seconds);
    }

    /// Starts a new frame by providing a new `Ui` instance to write into.
    pub fn begin_frame(&mut self) {
        self.context.begin_frame(self.raw_input.take());
    }

    /// Ends the frame. Returns what has happened as `Output` and gives you the draw instructions
    /// as `PaintJobs`. If the optional `window` is set, it will set the cursor key based on
    /// egui's instructions.
    pub fn end_frame(&mut self, window: Option<&winit::window::Window>) -> (egui::Output, Vec<ClippedShape>) {
        // otherwise the below line gets flagged by clippy if both clipboard and webbrowser features are disabled
        #[allow(clippy::let_and_return)]
        let parts = self.context.end_frame();

        if let Some(window) = window {
            if let Some(cursor_icon) = egui_to_winit_cursor_icon(parts.0.cursor_icon) {
                window.set_cursor_visible(true);
                // if the pointer is located inside the window, set cursor icon
                if self.pointer_pos.is_some() {
                    window.set_cursor_icon(cursor_icon);
                }
            } else {
                window.set_cursor_visible(false);
            }
        }

        #[cfg(feature = "clipboard")]
        handle_clipboard(&parts.0, self.clipboard.as_mut());

        #[cfg(feature = "webbrowser")]
        handle_links(&parts.0);

        parts
    }

    /// Returns the internal egui context.
    pub fn context(&self) -> CtxRef {
        self.context.clone()
    }
}

/// Translates winit to egui keycodes.
#[inline]
fn winit_to_egui_key_code(key: VirtualKeyCode) -> Option<egui::Key> {
    Some(match key {
        Escape => Key::Escape,
        Insert => Key::Insert,
        Home => Key::Home,
        Delete => Key::Delete,
        End => Key::End,
        PageDown => Key::PageDown,
        PageUp => Key::PageUp,
        Left => Key::ArrowLeft,
        Up => Key::ArrowUp,
        Right => Key::ArrowRight,
        Down => Key::ArrowDown,
        Back => Key::Backspace,
        Return => Key::Enter,
        Tab => Key::Tab,
        Space => Key::Space,

        A => Key::A,
        K => Key::K,
        U => Key::U,
        W => Key::W,
        Z => Key::Z,

        _ => {
            return None;
        }
    })
}

/// Translates winit to egui modifier keys.
#[inline]
fn winit_to_egui_modifiers(modifiers: ModifiersState) -> egui::Modifiers {
    egui::Modifiers {
        alt: modifiers.alt(),
        ctrl: modifiers.ctrl(),
        shift: modifiers.shift(),
        #[cfg(target_os = "macos")]
        mac_cmd: modifiers.logo(),
        #[cfg(target_os = "macos")]
        command: modifiers.logo(),
        #[cfg(not(target_os = "macos"))]
        mac_cmd: false,
        #[cfg(not(target_os = "macos"))]
        command: modifiers.ctrl(),
    }
}

#[inline]
fn egui_to_winit_cursor_icon(icon: egui::CursorIcon) -> Option<winit::window::CursorIcon> {
    use egui::CursorIcon::*;

    match icon {
        Default => Some(CursorIcon::Default),
        ContextMenu => Some(CursorIcon::ContextMenu),
        Help => Some(CursorIcon::Help),
        PointingHand => Some(CursorIcon::Hand),
        Progress => Some(CursorIcon::Progress),
        Wait => Some(CursorIcon::Wait),
        Cell => Some(CursorIcon::Cell),
        Crosshair => Some(CursorIcon::Crosshair),
        Text => Some(CursorIcon::Text),
        VerticalText => Some(CursorIcon::VerticalText),
        Alias => Some(CursorIcon::Alias),
        Copy => Some(CursorIcon::Copy),
        Move => Some(CursorIcon::Move),
        NoDrop => Some(CursorIcon::NoDrop),
        NotAllowed => Some(CursorIcon::NotAllowed),
        Grab => Some(CursorIcon::Grab),
        Grabbing => Some(CursorIcon::Grabbing),
        AllScroll => Some(CursorIcon::AllScroll),
        ResizeHorizontal => Some(CursorIcon::EwResize),
        ResizeNeSw => Some(CursorIcon::NeswResize),
        ResizeNwSe => Some(CursorIcon::NwseResize),
        ResizeVertical => Some(CursorIcon::NsResize),
        ZoomIn => Some(CursorIcon::ZoomIn),
        ZoomOut => Some(CursorIcon::ZoomOut),
        None => Option::None,
    }
}

/// We only want printable characters and ignore all special keys.
#[inline]
fn is_printable(chr: char) -> bool {
    let is_in_private_use_area =
        '\u{e000}' <= chr && chr <= '\u{f8ff}' || '\u{f0000}' <= chr && chr <= '\u{ffffd}' || '\u{100000}' <= chr && chr <= '\u{10fffd}';

    !is_in_private_use_area && !chr.is_ascii_control()
}
