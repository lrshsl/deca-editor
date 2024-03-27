use std::io;

use termion::event::Key;

use crate::{cursor_pos, editor::buffer::CharBuffer as _, emsg, flush, read_key};

use super::{
    buffer::Buffer,
    editor_functions::{EditorCommand as EdCmd, EditorFunction, Executable as _},
    keymaps::InputReaction,
    settings::Settings,
    Mode,
};

pub(crate) struct Editor {
    mode: Mode,
    settings: Settings,
    should_quit: bool,
    pub buf: Buffer,
}

impl Editor {
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
                EdCmd::Exit => self.should_quit = true,
                EdCmd::OpenFile => todo!(),
                EdCmd::Write => self.mode = Mode::Write,
                EdCmd::OpenCommandLine => todo!(),
            },
            Some(InputReaction::Function(fnc)) => self.execute(fnc)?,
            None => emsg!("<<Error:Move>> Key {c:?} not implemented")?,
        }
        flush()
    }

    fn execute(&self, fnc: &EditorFunction) -> io::Result<()> {
        match fnc {
            EditorFunction::GoLnBegin => self.go_ln_begin()?,
            EditorFunction::GoLnEnd => self.go_ln_end()?,
            EditorFunction::GoLnUp => self.go_ln_up(),
            EditorFunction::GoLnDown => self.go_ln_down(),
            EditorFunction::GoWordLeft => self.go_word_left(),
            EditorFunction::GoWordRight => self.go_word_right(),
            EditorFunction::GoCharLeft => self.go_char_left(),
            EditorFunction::GoCharRight => self.go_char_right(),
        }
        Ok(())
    }

    fn go_ln_begin(&self) -> io::Result<()> {
        let (_, y) = cursor_pos()?;
        print!("{}", termion::cursor::Goto(1, y));
        Ok(())
    }

    fn go_ln_end(&self) -> io::Result<()> {
        let (_, y) = cursor_pos()?;
        print!(
            "{}",
            termion::cursor::Goto(
                self.buf
                    .get(y as usize - 1)
                    .expect("<EdFn::mv_ln_end>, not on a valid line")
                    .len() as u16,
                y
            )
        );
        Ok(())
    }

    fn go_ln_up(&self) {
        print!("{}", termion::cursor::Up(1));
    }

    fn go_ln_down(&self) {
        print!("{}", termion::cursor::Down(1));
    }

    fn go_word_left(&self) {
        todo!()
    }

    fn go_word_right(&self) {
        todo!()
    }

    fn go_char_left(&self) {
        print!("{}", termion::cursor::Left(1));
    }

    fn go_char_right(&self) {
        print!("{}", termion::cursor::Right(1));
    }
}
