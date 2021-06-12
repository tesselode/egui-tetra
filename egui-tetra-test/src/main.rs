use std::error::Error;

use egui_tetra::{State, StateWrapper};
use tetra::{
	graphics::{self, Color},
	ContextBuilder, Event,
};

struct MainState {
	text: String,
}

impl MainState {
	fn new() -> Self {
		Self {
			text: String::new(),
		}
	}
}

impl State<Box<dyn Error>> for MainState {
	fn draw(
		&mut self,
		ctx: &mut tetra::Context,
		egui_ctx: &egui::CtxRef,
	) -> Result<(), Box<dyn Error>> {
		graphics::clear(ctx, Color::BLACK);
		egui::CentralPanel::default().show(egui_ctx, |ui| {
			ui.label("This is a label");
			ui.hyperlink("https://github.com/emilk/egui");
			ui.text_edit_singleline(&mut self.text);
			if ui.button("Click me").clicked() {}
			ui.add(egui::Slider::new(&mut 0.0, 0.0..=100.0));
			ui.add(egui::DragValue::new(&mut 0.0));

			ui.checkbox(&mut false, "Checkbox");

			#[derive(PartialEq)]
			enum Enum {
				First,
				Second,
				Third,
			}
			let mut my_enum = Enum::First;
			ui.horizontal(|ui| {
				ui.radio_value(&mut my_enum, Enum::First, "First");
				ui.radio_value(&mut my_enum, Enum::Second, "Second");
				ui.radio_value(&mut my_enum, Enum::Third, "Third");
			});

			ui.separator();

			ui.collapsing("Click to see what is hidden!", |ui| {
				ui.label("Not much, as it turns out");
			});
		});
		Ok(())
	}

	fn event(&mut self, _ctx: &mut tetra::Context, event: Event) -> Result<(), Box<dyn Error>> {
		if let Event::KeyPressed { key } = event {
			println!("{:?}", key);
		}
		Ok(())
	}
}

fn main() -> Result<(), Box<dyn Error>> {
	ContextBuilder::new("egui-tetra-test", 800, 600)
		.show_mouse(true)
		.build()?
		.run(|ctx| StateWrapper::new(ctx, MainState::new()))
}
