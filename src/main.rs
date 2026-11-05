#![warn(clippy::all, clippy::pedantic)]
mod editor;
mod terminal;

fn main() {
    let mut editor = editor::Editor::new();

    editor.run();
}
