use std::io;

use termion::event::Key;

use crate::{cursor_pos, editor::buffer::Bufferable, emsg, flush, read_key, wait_for_input};

use super::{
    buffer::Buffer,
    editor_functions::{EditorCommand as EdCmd, Executable as _},
    keymaps::InputReaction,
    settings::Settings,
    Mode,
};

pub(crate) struct Editor {
    mode: Mode,
    settings: Settings,
    should_quit: bool,
    buf: Buffer,
}

impl Editor {
    /// Better use `nice_new!` for keyword-arguments
    pub(crate) fn new(mode: Mode, settings: Settings, should_quit: bool, buf: Buffer) -> Self {
        Self {
            mode,
            settings,
            should_quit,
            buf,
        }
    }

    /// Main loop for the editor.
    pub fn run(&mut self) -> io::Result<()> {
        loop {
            if self.should_quit {
                break Ok(());
            }

            match read_key()? {
                Key::Esc => self.mode = Mode::Move,
                key => match self.mode {
                    Mode::Write => self.write(key)?,
                    Mode::Move => self.mv(key)?,
                    Mode::Select => {}
                },
            }
        }
    }

    fn write(&mut self, c: Key) -> io::Result<()> {
        let (x, y) = cursor_pos()?;
        match c {
            Key::Char('\n') => {
                self.buf.insert(y as usize, String::new());
                print!("\n{}", termion::cursor::Goto(1, y + 1));
            }
            Key::Backspace | Key::Ctrl('h') => {
                self.buf.delete_char(x as usize - 1, y as usize);
                print!("{} {}", termion::cursor::Left(1), termion::cursor::Left(1))
            }
            Key::Char(c) => {
                self.buf.insert_char(x as usize, y as usize, c);
                print!("{c}");
            }
            key => emsg!("<<Error:Write>> Key {key:?} not implemented")?,
        }
        flush()
    }

    fn mv(&mut self, c: Key) -> io::Result<()> {
        match self.settings.keymaps[0].get(&c) {
            Some(InputReaction::Command(cmd)) => match cmd {
                EdCmd::Write => self.mode = Mode::Write,
                EdCmd::Exit => self.should_quit = true,
            },
            Some(InputReaction::Function(fnc)) => fnc.execute()?,
            None => emsg!("<<Error:Move>> Key {c:?} not implemented")?,
        }
        flush()
    }
}
