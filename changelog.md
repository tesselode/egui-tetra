## v0.2.1 - September 18, 2021

- Fix egui not receiving screen size information

## v0.2.0 - August 26, 2021

### Improvements:

- Fix UI not rendering correctly when using fixed timestep
with vsync disabled
- Implement `Default` for `EguiWrapper`
- Update `egui` to 0.14.0
- Update `open` to 2.0.1

### Breaking changes

- Remove `OpenError` and change `Error::OpenError` to contain
an `std::io::Error`
- `EguiWrapper::end_frame` now takes a `&mut tetra::Context`
argument
- `EguiWrapper::draw_frame` no longer returns a `Result`

## v0.1.0 - June 26, 2021

First official release
