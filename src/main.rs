#![warn(clippy::all, clippy::pedantic, clippy::print_stdout)]
mod editor;
mod terminal;

fn main() {
    let mut editor = editor::Editor::new();

    editor.run();
}
