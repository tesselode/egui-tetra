use std::{error::Error, ops::RangeInclusive};

use egui_tetra::{State, StateWrapper};
use tetra::{
	graphics::{
		mesh::{Mesh, ShapeStyle},
		Color,
	},
	input::MouseButton,
	math::Vec2,
	Context,
};

const SCREEN_WIDTH: i32 = 800;
const SCREEN_HEIGHT: i32 = 600;
const BALL_RADIUS: f32 = 64.0;
const DEFAULT_GRAVITY: f32 = 500.0;
const GRAVITY_RANGE: RangeInclusive<f32> = 0.0..=1000.0;
const DEFAULT_BOUNCINESS: f32 = 0.5;
const BOUNCINESS_RANGE: RangeInclusive<f32> = 0.0..=1.0;

struct Ball {
	gravity: f32,
	bounciness: f32,
	position: Vec2<f32>,
	velocity: Vec2<f32>,
	circle_mesh: Mesh,
}

impl Ball {
	fn new(ctx: &mut Context) -> Result<Self, Box<dyn Error>> {
		Ok(Self {
			gravity: DEFAULT_GRAVITY,
			bounciness: DEFAULT_BOUNCINESS,
			position: Vec2::new(400.0, 300.0),
			velocity: Vec2::zero(),
			circle_mesh: Mesh::circle(ctx, ShapeStyle::Fill, Vec2::zero(), BALL_RADIUS)?,
		})
	}

	fn update(&mut self, ctx: &mut Context) {
		let delta_time = tetra::time::get_delta_time(ctx).as_secs_f32();
		self.velocity.y += self.gravity * delta_time;
		self.position += self.velocity * delta_time;
		if self.position.y + BALL_RADIUS > SCREEN_HEIGHT as f32 {
			self.position.y = SCREEN_HEIGHT as f32 - BALL_RADIUS;
			self.velocity.y *= -self.bounciness;
		}
	}

	fn draw(&self, ctx: &mut Context) {
		self.circle_mesh.draw(ctx, self.position);
	}
}

struct MainState {
	ball: Ball,
	moving_ball: bool,
}

impl MainState {
	pub fn new(ctx: &mut Context) -> Result<Self, Box<dyn Error>> {
		Ok(Self {
			ball: Ball::new(ctx)?,
			moving_ball: false,
		})
	}
}

impl State<Box<dyn Error>> for MainState {
	fn update(&mut self, ctx: &mut Context) -> Result<(), Box<dyn Error>> {
		if !self.moving_ball {
			self.ball.update(ctx);
		}
		Ok(())
	}

	fn draw(&mut self, ctx: &mut Context, egui_ctx: &egui::CtxRef) -> Result<(), Box<dyn Error>> {
		egui::Window::new("Bouncy Ball").show(egui_ctx, |ui| {
			ui.label("Gravity");
			ui.add(egui::Slider::new(&mut self.ball.gravity, GRAVITY_RANGE));
			ui.label("Bounciness");
			ui.add(egui::Slider::new(
				&mut self.ball.bounciness,
				BOUNCINESS_RANGE,
			));
		});

		tetra::graphics::clear(ctx, Color::BLACK);
		self.ball.draw(ctx);

		Ok(())
	}

	fn event(&mut self, ctx: &mut Context, event: tetra::Event) -> Result<(), Box<dyn Error>> {
		if let tetra::Event::MouseButtonPressed {
			button: MouseButton::Left,
		} = &event
		{
			self.moving_ball = true;
			self.ball.position = tetra::input::get_mouse_position(ctx);
			self.ball.velocity = Vec2::zero();
		}
		if let tetra::Event::MouseButtonReleased {
			button: MouseButton::Left,
		} = &event
		{
			self.moving_ball = false;
		}
		if let tetra::Event::MouseMoved { position, .. } = &event {
			if self.moving_ball {
				self.ball.position = *position;
			}
		}
		Ok(())
	}
}

fn main() -> Result<(), Box<dyn Error>> {
	tetra::ContextBuilder::new("Bouncy ball example", SCREEN_WIDTH, SCREEN_HEIGHT)
		.show_mouse(true)
		.build()?
		.run(|ctx| Ok(StateWrapper::new(MainState::new(ctx)?)))
}
