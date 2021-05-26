use std::error::Error;

use egui_tetra::EguiWrapper;
use tetra::{
	graphics::{self, Color},
	Context, ContextBuilder, Event, State,
};

struct MainState {
	egui_wrapper: EguiWrapper,
}

impl MainState {
	pub fn new() -> Self {
		Self {
			egui_wrapper: EguiWrapper::new(),
		}
	}
}

impl State<Box<dyn Error>> for MainState {
	fn update(&mut self, ctx: &mut Context) -> Result<(), Box<dyn Error>> {
		Ok(())
	}

	fn draw(&mut self, ctx: &mut Context) -> Result<(), Box<dyn Error>> {
		graphics::clear(ctx, Color::BLACK);
		self.egui_wrapper.begin_frame(ctx)?;
		egui::CentralPanel::default().show(self.egui_wrapper.ctx(), |ui| {
			ui.label("This is a label");
			ui.hyperlink("https://github.com/emilk/egui");
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
		self.egui_wrapper.end_frame(ctx)?;
		Ok(())
	}

	fn event(&mut self, ctx: &mut Context, event: Event) -> Result<(), Box<dyn Error>> {
		self.egui_wrapper.event(ctx, &event);
		Ok(())
	}
}

fn main() -> Result<(), Box<dyn Error>> {
	ContextBuilder::new("egui-tetra-test", 800, 600)
		.show_mouse(true)
		.multisampling(8)
		.build()?
		.run(|_| Ok(MainState::new()))
}
