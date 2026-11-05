#![warn(clippy::all, clippy::pedantic)]
mod editor;

fn main() {
    let editor = editor::Editor::new();

    editor.run();
}
