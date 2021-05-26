use std::error::Error;

use egui::RawInput;
use tetra::{
	graphics::{self, Color},
	Context, ContextBuilder, Event, State,
};

struct MainState;

impl State<Box<dyn Error>> for MainState {
	fn update(&mut self, ctx: &mut Context) -> Result<(), Box<dyn Error>> {
		Ok(())
	}

	fn draw(&mut self, ctx: &mut Context) -> Result<(), Box<dyn Error>> {
		graphics::clear(ctx, Color::BLACK);
		Ok(())
	}

	fn event(&mut self, ctx: &mut Context, event: Event) -> Result<(), Box<dyn Error>> {
		Ok(())
	}
}

fn main() -> Result<(), Box<dyn Error>> {
	ContextBuilder::new("egui-tetra-test", 800, 600)
		.build()?
		.run(|_| Ok(MainState))
}
