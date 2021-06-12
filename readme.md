# egui-tetra

egui-tetra is a library that helps integrate [egui](https://crates.io/crates/egui),
an immediate mode GUI library, with [Tetra](https://crates.io/crates/tetra),
a 2D game framework.

## Basic example

```rust
use std::error::Error;

struct MainState;

impl egui_tetra::State<Box<dyn Error>> for MainState {
	fn ui(
		&mut self,
		ctx: &mut tetra::Context,
		egui_ctx: &egui::CtxRef,
	) -> Result<(), Box<dyn Error>> {
		egui::Window::new("hi!").show(egui_ctx, |ui| {
			ui.label("Hello world!");
		});
		Ok(())
	}
}

fn main() -> Result<(), Box<dyn Error>> {
	tetra::ContextBuilder::new("example", 800, 600)
		.build()?
		.run(|_| Ok(egui_tetra::StateWrapper::new(MainState)))
}
```

## License

[MIT](https://github.com/tesselode/egui-tetra/blob/main/license.md)
