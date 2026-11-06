#![warn(
    clippy::all,
    clippy::pedantic,
    clippy::print_stdout,
    clippy::arithmetic_side_effects,
    clippy::as_conversions,
    clippy::integer_division
)]
mod editor;
mod terminal;

fn main() {
    let mut editor = editor::Editor::new();

    editor.run();
}
