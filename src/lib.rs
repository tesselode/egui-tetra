//! # egui-tetra
//!
//! egui-tetra is a library that helps integrate [egui](https://crates.io/crates/egui),
//! an immediate mode GUI library, with [Tetra](https://crates.io/crates/tetra),
//! a 2D game framework.
//!
//! ## Usage
//!
//! The easiest way to use egui-tetra is to make your main state struct implement
//! egui-tetra's [`State`] trait instead of [Tetra's](tetra::State). This will
//! give you access to a [`ui`](State::ui) callback where you can do your GUI
//! rendering.
//!
//! ```
//! use std::error::Error;
//! use egui_tetra::egui;
//!
//! struct MainState;
//!
//! impl egui_tetra::State<Box<dyn Error>> for MainState {
//! 	fn ui(
//! 		&mut self,
//! 		ctx: &mut tetra::Context,
//! 		egui_ctx: &egui::CtxRef,
//! 	) -> Result<(), Box<dyn Error>> {
//! 		egui::Window::new("hi!").show(egui_ctx, |ui| {
//! 			ui.label("Hello world!");
//! 		});
//! 		Ok(())
//! 	}
//!
//! 	fn update(
//! 		&mut self,
//! 		ctx: &mut tetra::Context,
//! 		egui_ctx: &egui::CtxRef,
//! 	) -> Result<(), Box<dyn Error>> {
//!         /// Your update code here
//! 		Ok(())
//! 	}
//!
//! 	fn draw(
//! 		&mut self,
//! 		ctx: &mut tetra::Context,
//! 		egui_ctx: &egui::CtxRef,
//! 	) -> Result<(), Box<dyn Error>> {
//!         /// Your drawing code here
//! 		Ok(())
//! 	}
//!
//! 	fn event(
//! 		&mut self,
//! 		ctx: &mut tetra::Context,
//! 		egui_ctx: &egui::CtxRef,
//! 		event: tetra::Event,
//! 	) -> Result<(), Box<dyn Error>> {
//!         /// Your event handling code here
//! 		Ok(())
//! 	}
//! }
//! ```
//!
//! When running the Tetra [`Context`](tetra::Context::run), wrap your state
//! struct in a [`StateWrapper`] to make it compatible with Tetra's
//! [`State` trait](tetra::State).
//!
//! ```no_run
//! # use std::error::Error;
//! # use egui_tetra::egui;
//! #
//! # struct MainState;
//! #
//! # impl egui_tetra::State<Box<dyn Error>> for MainState {
//! # 	fn ui(
//! # 		&mut self,
//! # 		ctx: &mut tetra::Context,
//! # 		egui_ctx: &egui::CtxRef,
//! # 	) -> Result<(), Box<dyn Error>> {
//! # 		egui::Window::new("hi!").show(egui_ctx, |ui| {
//! # 			ui.label("Hello world!");
//! # 		});
//! # 		Ok(())
//! # 	}
//! #
//! # 	fn update(
//! # 		&mut self,
//! # 		ctx: &mut tetra::Context,
//! # 		egui_ctx: &egui::CtxRef,
//! # 	) -> Result<(), Box<dyn Error>> {
//! # 		Ok(())
//! # 	}
//! #
//! # 	fn draw(
//! # 		&mut self,
//! # 		ctx: &mut tetra::Context,
//! # 		egui_ctx: &egui::CtxRef,
//! # 	) -> Result<(), Box<dyn Error>> {
//! # 		Ok(())
//! # 	}
//! #
//! # 	fn event(
//! # 		&mut self,
//! # 		ctx: &mut tetra::Context,
//! # 		egui_ctx: &egui::CtxRef,
//! # 		event: tetra::Event,
//! # 	) -> Result<(), Box<dyn Error>> {
//! # 		Ok(())
//! # 	}
//! # }
//! #
//! fn main() -> Result<(), Box<dyn Error>> {
//! 	tetra::ContextBuilder::new("example", 800, 600)
//! 		.build()?
//! 		.run(|_| Ok(egui_tetra::StateWrapper::new(MainState)))
//! }
//! ```
//!
//! If you need more control, you can use [`EguiWrapper`] and manually
//! hook up egui to Tetra's callbacks.

#![warn(missing_docs)]
#![allow(clippy::tabs_in_doc_comments)]

pub use egui;

use std::{fmt::Display, sync::Arc, time::Instant};

use copypasta::{ClipboardContext, ClipboardProvider};
use egui::{ClippedMesh, CtxRef, RawInput};
use tetra::{
	graphics::{self, BlendState},
	Event, TetraError,
};

const SCROLL_SENSITIVITY: f32 = 48.0;
const ZOOM_SENSITIVITY: f32 = 1.25;

fn tetra_vec2_to_egui_pos2(tetra_vec2: tetra::math::Vec2<f32>) -> egui::Pos2 {
	egui::pos2(tetra_vec2.x, tetra_vec2.y)
}

fn egui_pos2_to_tetra_vec2(egui_pos2: egui::Pos2) -> tetra::math::Vec2<f32> {
	tetra::math::Vec2::new(egui_pos2.x, egui_pos2.y)
}

fn egui_rect_to_tetra_rectangle(egui_rect: egui::Rect) -> tetra::graphics::Rectangle<i32> {
	tetra::graphics::Rectangle::new(
		egui_rect.left() as i32,
		egui_rect.top() as i32,
		egui_rect.width() as i32,
		egui_rect.height() as i32,
	)
}

fn egui_color32_to_tetra_color(egui_color: egui::Color32) -> tetra::graphics::Color {
	tetra::graphics::Color::rgba8(
		egui_color.r(),
		egui_color.g(),
		egui_color.b(),
		egui_color.a(),
	)
}

/// Converts a [tetra key](tetra::input::Key) to an
/// [egui key](egui::Key) if there's an egui equivalent
/// to the tetra key, otherwise returns `None`.
///
/// egui doesn't care about every keyboard key, so its listing of keys
/// is less comprehensive than tetra's.
fn tetra_key_to_egui_key(key: tetra::input::Key) -> Option<egui::Key> {
	match key {
		tetra::input::Key::A => Some(egui::Key::A),
		tetra::input::Key::B => Some(egui::Key::B),
		tetra::input::Key::C => Some(egui::Key::C),
		tetra::input::Key::D => Some(egui::Key::D),
		tetra::input::Key::E => Some(egui::Key::E),
		tetra::input::Key::F => Some(egui::Key::F),
		tetra::input::Key::G => Some(egui::Key::G),
		tetra::input::Key::H => Some(egui::Key::H),
		tetra::input::Key::I => Some(egui::Key::I),
		tetra::input::Key::J => Some(egui::Key::J),
		tetra::input::Key::K => Some(egui::Key::K),
		tetra::input::Key::L => Some(egui::Key::L),
		tetra::input::Key::M => Some(egui::Key::M),
		tetra::input::Key::N => Some(egui::Key::N),
		tetra::input::Key::O => Some(egui::Key::O),
		tetra::input::Key::P => Some(egui::Key::P),
		tetra::input::Key::Q => Some(egui::Key::Q),
		tetra::input::Key::R => Some(egui::Key::R),
		tetra::input::Key::S => Some(egui::Key::S),
		tetra::input::Key::T => Some(egui::Key::T),
		tetra::input::Key::U => Some(egui::Key::U),
		tetra::input::Key::V => Some(egui::Key::V),
		tetra::input::Key::W => Some(egui::Key::W),
		tetra::input::Key::X => Some(egui::Key::X),
		tetra::input::Key::Y => Some(egui::Key::Y),
		tetra::input::Key::Z => Some(egui::Key::Z),
		tetra::input::Key::Num0 => Some(egui::Key::Num0),
		tetra::input::Key::Num1 => Some(egui::Key::Num1),
		tetra::input::Key::Num2 => Some(egui::Key::Num2),
		tetra::input::Key::Num3 => Some(egui::Key::Num3),
		tetra::input::Key::Num4 => Some(egui::Key::Num4),
		tetra::input::Key::Num5 => Some(egui::Key::Num5),
		tetra::input::Key::Num6 => Some(egui::Key::Num6),
		tetra::input::Key::Num7 => Some(egui::Key::Num7),
		tetra::input::Key::Num8 => Some(egui::Key::Num8),
		tetra::input::Key::Num9 => Some(egui::Key::Num9),
		tetra::input::Key::NumPad0 => Some(egui::Key::Num0),
		tetra::input::Key::NumPad1 => Some(egui::Key::Num1),
		tetra::input::Key::NumPad2 => Some(egui::Key::Num2),
		tetra::input::Key::NumPad3 => Some(egui::Key::Num3),
		tetra::input::Key::NumPad4 => Some(egui::Key::Num4),
		tetra::input::Key::NumPad5 => Some(egui::Key::Num5),
		tetra::input::Key::NumPad6 => Some(egui::Key::Num6),
		tetra::input::Key::NumPad7 => Some(egui::Key::Num7),
		tetra::input::Key::NumPad8 => Some(egui::Key::Num8),
		tetra::input::Key::NumPad9 => Some(egui::Key::Num9),
		tetra::input::Key::NumPadEnter => Some(egui::Key::Enter),
		tetra::input::Key::Up => Some(egui::Key::ArrowUp),
		tetra::input::Key::Down => Some(egui::Key::ArrowDown),
		tetra::input::Key::Left => Some(egui::Key::ArrowLeft),
		tetra::input::Key::Right => Some(egui::Key::ArrowRight),
		tetra::input::Key::Backspace => Some(egui::Key::Backspace),
		tetra::input::Key::Delete => Some(egui::Key::Delete),
		tetra::input::Key::End => Some(egui::Key::End),
		tetra::input::Key::Enter => Some(egui::Key::Enter),
		tetra::input::Key::Escape => Some(egui::Key::Escape),
		tetra::input::Key::Home => Some(egui::Key::Home),
		tetra::input::Key::Insert => Some(egui::Key::Insert),
		tetra::input::Key::PageDown => Some(egui::Key::PageDown),
		tetra::input::Key::PageUp => Some(egui::Key::PageUp),
		tetra::input::Key::Space => Some(egui::Key::Space),
		tetra::input::Key::Tab => Some(egui::Key::Tab),
		_ => None,
	}
}

/// Converts a [tetra mouse button](tetra::input::MouseButton) to an
/// [egui mouse button](egui::PointerButton) if there's an egui equivalent
/// to the tetra mouse button, otherwise returns `None`.
///
/// egui only supports left, middle, and right buttons.
fn tetra_mouse_button_to_egui_pointer_button(
	tetra_mouse_button: tetra::input::MouseButton,
) -> Option<egui::PointerButton> {
	match tetra_mouse_button {
		tetra::input::MouseButton::Left => Some(egui::PointerButton::Primary),
		tetra::input::MouseButton::Middle => Some(egui::PointerButton::Middle),
		tetra::input::MouseButton::Right => Some(egui::PointerButton::Secondary),
		_ => None,
	}
}

fn egui_mesh_to_tetra_mesh(
	ctx: &mut tetra::Context,
	egui_mesh: egui::epaint::Mesh,
	texture: tetra::graphics::Texture,
) -> tetra::Result<tetra::graphics::mesh::Mesh> {
	let index_buffer = tetra::graphics::mesh::IndexBuffer::new(ctx, &egui_mesh.indices)?;
	let vertices: Vec<tetra::graphics::mesh::Vertex> = egui_mesh
		.vertices
		.iter()
		.map(|vertex| {
			tetra::graphics::mesh::Vertex::new(
				egui_pos2_to_tetra_vec2(vertex.pos),
				egui_pos2_to_tetra_vec2(vertex.uv),
				egui_color32_to_tetra_color(vertex.color),
			)
		})
		.collect();
	let vertex_buffer = tetra::graphics::mesh::VertexBuffer::new(ctx, &vertices)?;
	let mut mesh = tetra::graphics::mesh::Mesh::indexed(vertex_buffer, index_buffer);
	mesh.set_texture(texture);
	mesh.set_backface_culling(false);
	Ok(mesh)
}

/// Converts an [egui font texture](egui::Texture) to a
/// [tetra texture](tetra::graphics::Texture).
fn egui_font_image_to_tetra_texture(
	ctx: &mut tetra::Context,
	egui_font_image: Arc<egui::FontImage>,
) -> tetra::Result<tetra::graphics::Texture> {
	let mut pixels = vec![];
	// each u8 of the egui texture is the alpha channel.
	// the other components are always white. since egui
	// uses premultiplied alpha, we set every component in the
	// tetra texture to the alpha.
	for alpha in &egui_font_image.pixels {
		pixels.push(*alpha);
		pixels.push(*alpha);
		pixels.push(*alpha);
		pixels.push(*alpha);
	}
	tetra::graphics::Texture::from_data(
		ctx,
		egui_font_image.width as i32,
		egui_font_image.height as i32,
		graphics::TextureFormat::Rgba8,
		&pixels,
	)
}

/// An error that can occur when using egui-tetra.
#[derive(Debug)]
pub enum Error {
	/// A Tetra error occurred.
	TetraError(TetraError),
	/// An error occurred when opening a URL or other path
	/// by clicking a hyperlink.
	OpenError(std::io::Error),
	/// An error occurred when accessing the system's clipboard.
	ClipboardError(Box<dyn std::error::Error + Send + Sync>),
}

impl Display for Error {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Error::TetraError(error) => error.fmt(f),
			Error::OpenError(error) => error.fmt(f),
			Error::ClipboardError(error) => error.fmt(f),
		}
	}
}

impl std::error::Error for Error {
	fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
		match self {
			Error::TetraError(error) => Some(error),
			Error::OpenError(error) => Some(error),
			Error::ClipboardError(error) => Some(error.as_ref()),
		}
	}
}

impl From<TetraError> for Error {
	fn from(error: TetraError) -> Self {
		Self::TetraError(error)
	}
}

impl From<std::io::Error> for Error {
	fn from(v: std::io::Error) -> Self {
		Self::OpenError(v)
	}
}

impl From<Box<dyn std::error::Error + Send + Sync>> for Error {
	fn from(error: Box<dyn std::error::Error + Send + Sync>) -> Self {
		Self::ClipboardError(error)
	}
}

/// Wraps an egui context with features that are useful
/// for integrating egui with Tetra.
pub struct EguiWrapper {
	raw_input: RawInput,
	ctx: CtxRef,
	texture: Option<tetra::graphics::Texture>,
	last_frame_time: Instant,
	meshes: Vec<(tetra::graphics::Rectangle<i32>, tetra::graphics::mesh::Mesh)>,
}

impl EguiWrapper {
	/// Creates a new [`EguiWrapper`] and underlying egui context.
	pub fn new() -> Self {
		Self {
			raw_input: RawInput::default(),
			ctx: CtxRef::default(),
			texture: None,
			last_frame_time: Instant::now(),
			meshes: vec![],
		}
	}

	/// Returns a reference to the underlying egui context.
	pub fn ctx(&self) -> &egui::CtxRef {
		&self.ctx
	}

	/// Dispaches a Tetra [`Event`](tetra::Event) to the egui context.
	pub fn event(&mut self, ctx: &tetra::Context, event: &tetra::Event) -> Result<(), Error> {
		match event {
			tetra::Event::KeyPressed { key } => {
				// update modifiers
				match key {
					tetra::input::Key::LeftCtrl | tetra::input::Key::RightCtrl => {
						self.raw_input.modifiers.ctrl = true;
						self.raw_input.modifiers.command = true;
					}
					tetra::input::Key::LeftShift | tetra::input::Key::RightShift => {
						self.raw_input.modifiers.shift = true;
					}
					tetra::input::Key::LeftAlt | tetra::input::Key::RightAlt => {
						self.raw_input.modifiers.alt = true;
					}
					_ => {}
				}

				// copy/cut/paste
				if tetra::input::is_key_down(ctx, tetra::input::Key::LeftCtrl)
					| tetra::input::is_key_down(ctx, tetra::input::Key::RightCtrl)
				{
					if let tetra::input::Key::C = key {
						self.raw_input.events.push(egui::Event::Copy);
					}
					if let tetra::input::Key::X = key {
						self.raw_input.events.push(egui::Event::Cut);
					}
					if let tetra::input::Key::V = key {
						self.raw_input
							.events
							.push(egui::Event::Text(ClipboardContext::new()?.get_contents()?));
					}
				}

				if let Some(key) = tetra_key_to_egui_key(*key) {
					self.raw_input.events.push(egui::Event::Key {
						key,
						pressed: true,
						modifiers: self.raw_input.modifiers,
					});
				}
			}
			tetra::Event::KeyReleased { key } => {
				match key {
					tetra::input::Key::LeftCtrl | tetra::input::Key::RightCtrl => {
						self.raw_input.modifiers.ctrl = false;
						self.raw_input.modifiers.command = false;
					}
					tetra::input::Key::LeftShift | tetra::input::Key::RightShift => {
						self.raw_input.modifiers.shift = false;
					}
					tetra::input::Key::LeftAlt | tetra::input::Key::RightAlt => {
						self.raw_input.modifiers.alt = false;
					}
					_ => {}
				}
				if let Some(key) = tetra_key_to_egui_key(*key) {
					self.raw_input.events.push(egui::Event::Key {
						key,
						pressed: false,
						modifiers: self.raw_input.modifiers,
					});
				}
			}
			tetra::Event::MouseButtonPressed { button } => {
				if let Some(button) = tetra_mouse_button_to_egui_pointer_button(*button) {
					self.raw_input.events.push(egui::Event::PointerButton {
						pos: tetra_vec2_to_egui_pos2(tetra::input::get_mouse_position(ctx)),
						button,
						pressed: true,
						modifiers: self.raw_input.modifiers,
					});
				}
			}
			tetra::Event::MouseButtonReleased { button } => {
				if let Some(button) = tetra_mouse_button_to_egui_pointer_button(*button) {
					self.raw_input.events.push(egui::Event::PointerButton {
						pos: tetra_vec2_to_egui_pos2(tetra::input::get_mouse_position(ctx)),
						button,
						pressed: false,
						modifiers: self.raw_input.modifiers,
					});
				}
			}
			tetra::Event::MouseMoved { position, .. } => {
				self.raw_input
					.events
					.push(egui::Event::PointerMoved(tetra_vec2_to_egui_pos2(
						*position,
					)));
			}
			tetra::Event::MouseWheelMoved { amount } => {
				if tetra::input::is_key_down(ctx, tetra::input::Key::LeftCtrl)
					|| tetra::input::is_key_down(ctx, tetra::input::Key::RightCtrl)
				{
					self.raw_input
						.events
						.push(egui::Event::Zoom(ZOOM_SENSITIVITY.powi(amount.y)));
				} else {
					self.raw_input.events.push(egui::Event::Scroll(
						egui::vec2(amount.x as f32, amount.y as f32) * SCROLL_SENSITIVITY,
					));
				}
			}
			tetra::Event::TextInput { text } => {
				self.raw_input.events.push(egui::Event::Text(text.clone()));
			}
			_ => {}
		}
		Ok(())
	}

	/// Begins a new GUI frame.
	pub fn begin_frame(&mut self, ctx: &mut tetra::Context) -> Result<(), Error> {
		let now = Instant::now();
		self.raw_input.screen_rect = Some(egui::Rect {
			min: egui::pos2(0.0, 0.0),
			max: egui::pos2(
				tetra::window::get_width(ctx) as f32,
				tetra::window::get_height(ctx) as f32,
			),
		});
		self.raw_input.predicted_dt = (now - self.last_frame_time).as_secs_f32();
		self.last_frame_time = now;
		self.meshes.clear();
		self.ctx.begin_frame(self.raw_input.take());
		if self.texture.is_none() {
			self.texture = Some(egui_font_image_to_tetra_texture(
				ctx,
				self.ctx.font_image(),
			)?);
		}
		Ok(())
	}

	/// Ends a GUI frame.
	pub fn end_frame(&mut self, ctx: &mut tetra::Context) -> Result<(), Error> {
		let (output, shapes) = self.ctx.end_frame();
		if let Some(texture) = &self.texture {
			let clipped_meshes = self.ctx.tessellate(shapes);
			for ClippedMesh(rect, mesh) in clipped_meshes {
				let rect = egui_rect_to_tetra_rectangle(rect);
				let mesh = egui_mesh_to_tetra_mesh(ctx, mesh, texture.clone())?;
				self.meshes.push((rect, mesh));
			}
		}

		// open URLs that were clicked
		if let Some(open_url) = &output.open_url {
			open::that(&open_url.url)?;
		}

		// copy text to clipboard
		if !output.copied_text.is_empty() {
			ClipboardContext::new()?.set_contents(output.copied_text)?;
		}

		Ok(())
	}

	/// Draws the latest finished GUI frame to the screen.
	///
	/// Note that this function changes the Tetra blend mode and
	/// scissor state.
	pub fn draw_frame(&mut self, ctx: &mut tetra::Context) {
		graphics::set_blend_state(ctx, BlendState::alpha(true));
		for (rect, mesh) in &self.meshes {
			graphics::set_scissor(ctx, *rect);
			mesh.draw(ctx, tetra::math::Vec2::zero());
		}
		graphics::reset_scissor(ctx);
		graphics::reset_blend_state(ctx);
	}
}

impl Default for EguiWrapper {
	fn default() -> Self {
		Self::new()
	}
}

/// A trait analogous to [`tetra::State`], but with the addition of a
/// [`ui`](State::ui) callback and an `egui_ctx` argument in the
/// other callbacks.
///
/// You can use a type implementing this trait as your main game
/// state by wrapping it with a [`StateWrapper`] and passing the wrapper
/// to [`tetra::Context::run`].
#[allow(unused_variables)]
pub trait State<E: From<Error> = Error> {
	/// Called when it is time for the game to construct a GUI.
	fn ui(&mut self, ctx: &mut tetra::Context, egui_ctx: &egui::CtxRef) -> Result<(), E> {
		Ok(())
	}

	/// Called when it is time for the game to update.
	fn update(&mut self, ctx: &mut tetra::Context, egui_ctx: &egui::CtxRef) -> Result<(), E> {
		Ok(())
	}

	/// Called when it is time for the game to be drawn.
	fn draw(&mut self, ctx: &mut tetra::Context, egui_ctx: &egui::CtxRef) -> Result<(), E> {
		Ok(())
	}

	/// Called when a window or input event occurs.
	///
	/// Mouse and keyboard input events will not be received if the GUI
	/// is using the mouse or keyboard, respectively.
	fn event(
		&mut self,
		ctx: &mut tetra::Context,
		egui_ctx: &egui::CtxRef,
		event: Event,
	) -> Result<(), E> {
		Ok(())
	}
}

/// An adaptor that implements [`tetra::State`] for implementors of
/// [`State`].
pub struct StateWrapper<E: From<Error>> {
	events: Vec<tetra::Event>,
	state: Box<dyn State<E>>,
	egui: EguiWrapper,
}

impl<E: From<Error>> StateWrapper<E> {
	/// Wraps an implementor of [`State`] so it implements [`tetra::State`].
	pub fn new(state: impl State<E> + 'static) -> Self {
		Self {
			events: vec![],
			state: Box::new(state),
			egui: EguiWrapper::new(),
		}
	}

	/// Returns a reference to this wrapper's egui context.
	pub fn ctx(&self) -> &egui::CtxRef {
		self.egui.ctx()
	}
}

/*
A note about the order of events:

Tetra's game loop is:
- Poll and dispatch events
- Run the update callback (may not happen every frame depending
on the timestamp setting)
- Run the draw callback

We don't want to dispatch mouse and keyboard events to the
gameplay code if egui wants them, but egui can't tell us if
it wants them until after we call end_frame(). So we need
to run the UI code at the beginning of the game loop (but
not draw the result until the end of the game loop, of course).

Here's the order of operations I settled on:
- Whenever event is called, send it to the egui ctx and queue
it up for later
- At the beginning of update, run the UI callback and save the
resulting meshes and scissor rectangles. Then, dispatch queued
events to the gameplay code (unless the UI wanted them).
- In the draw callback, draw gameplay first, then UI
*/

impl<E: From<Error>> tetra::State<E> for StateWrapper<E> {
	fn update(&mut self, ctx: &mut tetra::Context) -> Result<(), E> {
		self.egui.begin_frame(ctx)?;
		self.state.ui(ctx, self.egui.ctx())?;
		self.egui.end_frame(ctx)?;

		for event in self.events.drain(..) {
			match &event {
				Event::KeyPressed { .. } | Event::KeyReleased { .. } => {
					if self.egui.ctx().wants_keyboard_input() {
						continue;
					}
				}
				Event::MouseButtonPressed { .. } | Event::MouseButtonReleased { .. } => {
					if self.egui.ctx().is_using_pointer() {
						continue;
					}
				}
				Event::MouseMoved { .. } => {
					if self.egui.ctx().is_using_pointer() {
						continue;
					}
				}
				Event::MouseWheelMoved { .. } => {
					if self.egui.ctx().is_using_pointer() {
						continue;
					}
				}
				_ => {}
			}
			self.state.event(ctx, self.egui.ctx(), event)?;
		}

		self.state.update(ctx, self.egui.ctx())
	}

	fn draw(&mut self, ctx: &mut tetra::Context) -> Result<(), E> {
		self.state.draw(ctx, self.egui.ctx())?;
		self.egui.draw_frame(ctx);
		Ok(())
	}

	fn event(&mut self, ctx: &mut tetra::Context, event: Event) -> Result<(), E> {
		self.egui.event(ctx, &event)?;
		self.events.push(event);
		Ok(())
	}
}
