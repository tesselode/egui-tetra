use std::{fmt::Display, process::ExitStatus, sync::Arc, time::Instant};

use copypasta::{ClipboardContext, ClipboardProvider};
use egui::{ClippedMesh, CtxRef, RawInput};
use tetra::{
	graphics::{self, BlendAlphaMode, BlendMode},
	Context, Event, TetraError,
};

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
fn egui_texture_to_tetra_texture(
	ctx: &mut tetra::Context,
	egui_texture: Arc<egui::Texture>,
) -> tetra::Result<tetra::graphics::Texture> {
	let mut pixels = vec![];
	// each u8 of the egui texture is the alpha channel.
	// the other components are always white. since egui
	// uses premultiplied alpha, we set every component in the
	// tetra texture to the alpha.
	for alpha in &egui_texture.pixels {
		pixels.push(*alpha);
		pixels.push(*alpha);
		pixels.push(*alpha);
		pixels.push(*alpha);
	}
	tetra::graphics::Texture::from_rgba(
		ctx,
		egui_texture.width as i32,
		egui_texture.height as i32,
		&pixels,
	)
}

#[derive(Debug)]
pub enum OpenUrlError {
	IoError(std::io::Error),
	ProcessError(ExitStatus),
}

impl Display for OpenUrlError {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			OpenUrlError::IoError(error) => error.fmt(f),
			OpenUrlError::ProcessError(status) => status.fmt(f),
		}
	}
}

impl std::error::Error for OpenUrlError {
	fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
		match self {
			OpenUrlError::IoError(error) => Some(error),
			OpenUrlError::ProcessError(_) => None,
		}
	}
}

impl From<std::io::Error> for OpenUrlError {
	fn from(error: std::io::Error) -> Self {
		Self::IoError(error)
	}
}

impl From<ExitStatus> for OpenUrlError {
	fn from(status: ExitStatus) -> Self {
		Self::ProcessError(status)
	}
}

#[derive(Debug)]
pub enum Error {
	TetraError(TetraError),
	OpenUrlError(OpenUrlError),
	ClipboardError(Box<dyn std::error::Error>),
}

impl Display for Error {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Error::TetraError(error) => error.fmt(f),
			Error::OpenUrlError(error) => error.fmt(f),
			Error::ClipboardError(error) => error.fmt(f),
		}
	}
}

impl std::error::Error for Error {
	fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
		match self {
			Error::TetraError(error) => Some(error),
			Error::OpenUrlError(error) => Some(error),
			Error::ClipboardError(error) => Some(error.as_ref()),
		}
	}
}

impl From<TetraError> for Error {
	fn from(error: TetraError) -> Self {
		Self::TetraError(error)
	}
}

impl From<OpenUrlError> for Error {
	fn from(error: OpenUrlError) -> Self {
		Self::OpenUrlError(error)
	}
}

impl From<Box<dyn std::error::Error + Send + Sync>> for Error {
	fn from(error: Box<dyn std::error::Error + Send + Sync>) -> Self {
		Self::ClipboardError(error)
	}
}

/// Wraps an egui context with features that are useful
/// for integrating egui with tetra.
pub struct EguiWrapper {
	raw_input: RawInput,
	ctx: CtxRef,
	texture: Option<tetra::graphics::Texture>,
	last_frame_time: Instant,
}

impl EguiWrapper {
	/// Creates a new [`EguiWrapper`] and underlying egui context.
	pub fn new() -> Self {
		Self {
			raw_input: RawInput::default(),
			ctx: CtxRef::default(),
			texture: None,
			last_frame_time: Instant::now(),
		}
	}

	/// Returns a reference to the underlying egui context.
	pub fn ctx(&self) -> &egui::CtxRef {
		&self.ctx
	}

	/// Takes a tetra event and updates the egui context as needed.
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
				self.raw_input.scroll_delta = egui::vec2(amount.x as f32, amount.y as f32);
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
		self.raw_input.predicted_dt = (now - self.last_frame_time).as_secs_f32();
		self.last_frame_time = now;
		self.ctx.begin_frame(self.raw_input.take());
		if self.texture.is_none() {
			self.texture = Some(egui_texture_to_tetra_texture(ctx, self.ctx.texture())?);
		}
		Ok(())
	}

	/// Ends a GUI frame, drawing the result to the screen.
	///
	/// Note that this function changes the tetra blend mode and
	/// scissor state.
	pub fn end_frame(&self, ctx: &mut tetra::Context) -> Result<(), Error> {
		let (output, shapes) = self.ctx.end_frame();

		// draw meshes
		if let Some(texture) = &self.texture {
			graphics::set_blend_mode(ctx, BlendMode::Alpha(BlendAlphaMode::Premultiplied));
			let clipped_meshes = self.ctx.tessellate(shapes);
			for ClippedMesh(rect, mesh) in clipped_meshes {
				graphics::set_scissor(ctx, egui_rect_to_tetra_rectangle(rect));
				let mesh = egui_mesh_to_tetra_mesh(ctx, mesh, texture.clone())?;
				mesh.draw(ctx, tetra::math::Vec2::zero());
			}
			graphics::reset_scissor(ctx);
			graphics::reset_blend_mode(ctx);
		}

		// open URLs that were clicked
		if let Some(open_url) = &output.open_url {
			let status = open::that(&open_url.url).map_err(|error| OpenUrlError::IoError(error))?;
			if !status.success() {
				return Err(Error::OpenUrlError(OpenUrlError::ProcessError(status)));
			}
		}

		// copy text to clipboard
		if !output.copied_text.is_empty() {
			ClipboardContext::new()?.set_contents(output.copied_text)?;
		}

		Ok(())
	}
}

/// A trait analogous to [`tetra::State`], but with the addition of an
/// `egui_ctx` argument in the [`draw`](State::draw) function.
///
/// You can use a type implementing this trait as your main game
/// state by wrapping it with a [`StateWrapper`] and passing the wrapper
/// to [`tetra::Context::run`].
#[allow(unused_variables)]
pub trait State<E: From<Error> = Error> {
	fn ui(&mut self, ctx: &mut tetra::Context, egui_ctx: &egui::CtxRef) -> Result<(), E> {
		Ok(())
	}

	/// Called when it is time for the game to update.
	fn update(&mut self, ctx: &mut tetra::Context) -> Result<(), E> {
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
	fn event(&mut self, ctx: &mut tetra::Context, event: Event) -> Result<(), E> {
		Ok(())
	}
}

/// An adaptor that implements [`tetra::State`] for implementors of
/// [`State`].
pub struct StateWrapper<E: From<Error>> {
	state: Box<dyn State<E>>,
	egui: EguiWrapper,
}

impl<E: From<Error>> StateWrapper<E> {
	/// Wraps an implementor of [`State`] so it implements [`tetra::State`].
	pub fn new(ctx: &mut Context, mut state: impl State<E> + 'static) -> Result<Self, E> {
		let mut egui = EguiWrapper::new();
		egui.begin_frame(ctx)?;
		state.ui(ctx, egui.ctx())?;
		Ok(Self {
			state: Box::new(state),
			egui,
		})
	}

	/// Returns a reference to this wrapper's egui context.
	pub fn ctx(&self) -> &egui::CtxRef {
		self.egui.ctx()
	}
}

impl<E: From<Error>> tetra::State<E> for StateWrapper<E> {
	fn update(&mut self, ctx: &mut tetra::Context) -> Result<(), E> {
		self.state.update(ctx)
	}

	fn draw(&mut self, ctx: &mut tetra::Context) -> Result<(), E> {
		self.state.draw(ctx, self.egui.ctx())?;
		self.egui.end_frame(ctx)?;
		self.egui.begin_frame(ctx)?;
		self.state.ui(ctx, self.egui.ctx())?;
		Ok(())
	}

	fn event(&mut self, ctx: &mut tetra::Context, event: Event) -> Result<(), E> {
		self.egui.event(ctx, &event)?;
		match &event {
			Event::KeyPressed { .. } | Event::KeyReleased { .. } => {
				if self.egui.ctx().wants_keyboard_input() {
					return Ok(());
				}
			}
			Event::MouseButtonPressed { .. } | Event::MouseButtonReleased { .. } => {
				if self.egui.ctx().is_using_pointer() {
					return Ok(());
				}
			}
			Event::MouseMoved { .. } => {
				if self.egui.ctx().is_using_pointer() {
					return Ok(());
				}
			}
			Event::MouseWheelMoved { .. } => {
				if self.egui.ctx().is_using_pointer() {
					return Ok(());
				}
			}
			_ => {}
		}
		self.state.event(ctx, event)?;
		Ok(())
	}
}
