#![warn(
    clippy::all,
    clippy::pedantic,
    clippy::print_stdout,
    clippy::arithmetic_side_effects,
    clippy::as_conversions,
    clippy::integer_division
)]
mod editor;

fn main() {
    let args = std::env::args().collect::<Vec<_>>();
    let mut editor = editor::Editor::default();

    if let Some(path) = args.get(1) {
        editor.load(path);
    }

    editor.run();
}
