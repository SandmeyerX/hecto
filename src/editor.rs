mod command;
mod terminal;
mod view;

use crate::editor::command::EditorCommand;
use crate::editor::terminal::Terminal;
use crate::editor::view::View;
use crossterm::event::{Event, read};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub struct Editor {
    should_quit: bool,
    view: View,
}

impl Drop for Editor {
    fn drop(&mut self) {
        let _ = Terminal::terminate();

        if self.should_quit {
            let _ = Terminal::print("byebye~\n");
        }
    }
}

impl Editor {
    pub fn build() -> Result<Self> {
        Self::setup_panic_hook();
        Terminal::init()?;

        let mut view = View::default();
        let args = std::env::args().collect::<Vec<_>>();
        if let Some(path) = args.get(1) {
            view.load(path)?;
        }

        Ok(Self {
            should_quit: false,
            view,
        })
    }

    pub fn run(&mut self) {
        loop {
            if self.should_quit {
                break;
            }
            self.refresh_screen();

            match read() {
                Ok(event) => self.eval_event(event),
                Err(err) => {
                    #[cfg(debug_assertions)]
                    panic!("failed to read event: {err}");
                }
            }
        }
    }

    fn setup_panic_hook() {
        let current_hook = std::panic::take_hook();
        std::panic::set_hook(Box::new(move |info| {
            let _ = Terminal::terminate();
            current_hook(info);
        }));
    }

    fn refresh_screen(&mut self) {
        let _ = Terminal::hide_caret();
        self.view.render();
        let _ = Terminal::move_caret_to(self.view.get_position());
        let _ = Terminal::show_caret();
        let _ = Terminal::execute();
    }

    fn eval_event(&mut self, event: Event) {
        let should_process = match event {
            Event::Key(key_event) if key_event.is_press() => true,
            Event::Resize(_, _) => true,
            _ => false,
        };

        if !should_process {
            return;
        }

        match EditorCommand::try_from(event) {
            Ok(cmd) => {
                if cmd == EditorCommand::Quit {
                    self.should_quit = true;
                    return;
                }
                self.view.handle_cmd(cmd);
            }
            Err(err) => {
                #[cfg(debug_assertions)]
                panic!("failed to transform event to `EditorCommand`: {err}");
            }
        }
    }
}
