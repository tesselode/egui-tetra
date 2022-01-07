use std::{collections::VecDeque, error::Error};

use egui::plot::{Line, Plot, Value, Values};
use egui_tetra::{egui, State, StateWrapper};
use tetra::Context;

const MAX_FPS_MEASUREMENTS: usize = 100;

struct MainState {
	fps_measurements: VecDeque<f64>,
}

impl MainState {
	fn new() -> Self {
		Self {
			fps_measurements: VecDeque::new(),
		}
	}
}

impl State<Box<dyn Error>> for MainState {
	fn update(
		&mut self,
		ctx: &mut Context,
		_egui_ctx: &egui::CtxRef,
	) -> Result<(), Box<dyn Error>> {
		self.fps_measurements.push_back(tetra::time::get_fps(ctx));
		if self.fps_measurements.len() > MAX_FPS_MEASUREMENTS {
			self.fps_measurements.pop_front();
		}
		Ok(())
	}

	fn ui(&mut self, _ctx: &mut Context, egui_ctx: &egui::CtxRef) -> Result<(), Box<dyn Error>> {
		egui::CentralPanel::default().show(egui_ctx, |ui| {
			Plot::new("fps").include_y(0.0).show(ui, |ui| {
				ui.line(Line::new(Values::from_values_iter(
					self.fps_measurements
						.iter()
						.enumerate()
						.map(|(i, fps)| Value::new(i as f64, *fps)),
				)))
			});
		});
		Ok(())
	}
}

fn main() -> Result<(), Box<dyn Error>> {
	tetra::ContextBuilder::new("Plot demo", 1280, 720)
		.show_mouse(true)
		.resizable(true)
		.vsync(false)
		.build()?
		.run(|_| Ok(StateWrapper::new(MainState::new())))
}
