use std::io;

use termion::event::Key;

use crate::{cursor_pos, editor::buffer::CharBuffer as _, emsg, flush, read_key, warn};

use super::{
    buffer::Buffer,
    editor_functions::{EditorCommand as EdCmd, EditorFunction},
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
            EditorFunction::GoLnUp => self.go_ln_up()?,
            EditorFunction::GoLnDown => self.go_ln_down()?,
            EditorFunction::GoWordLeft => self.go_word_left(),
            EditorFunction::GoWordRight => self.go_word_right(),
            EditorFunction::GoCharLeft => self.go_char_left(),
            EditorFunction::GoCharRight => self.go_char_right()?,
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
                    .expect("<EdFn::mv_ln_end> Not on a valid line")
                    .len() as u16,
                y
            )
        );
        Ok(())
    }

    fn go_ln_up(&self) -> io::Result<()> {
        let (mut x, y) = cursor_pos()?;
        let last_line_length = self.buf[y as usize - 2].len() as u16;
        if last_line_length < x {
            x = last_line_length;
        }
        print!("{}", termion::cursor::Goto(x, y - 1));
        Ok(())
    }

    fn go_ln_down(&self) -> io::Result<()> {
        let (mut x, y) = cursor_pos()?;
        let is_last_line = y as usize >= self.buf.len(); // y is 1-indexed
        if is_last_line {
            warn!("Already on last line")?;
            return Ok(());
        }
        let next_line_length = self.buf[y as usize].len() as u16;
        if next_line_length < x {
            x = next_line_length;
        }
        print!("{}", termion::cursor::Goto(x, y + 1));
        Ok(())
    }

    fn go_word_left(&self) {
        todo!()
    }

    fn go_word_right(&self) {
        todo!()
    }

    fn go_char_left(&self) {
        let (x, _) = cursor_pos().unwrap();
        if x > 1 {
            print!("{}", termion::cursor::Left(1));
        }
    }

    fn go_char_right(&self) -> io::Result<()> {
        let (x, y) = cursor_pos()?;
        if x < self
            .buf
            .get(y as usize - 1)
            .expect("<EdFn::mv_char_right> Not on a valid line for index y")
            .len() as u16
        {
            print!("{}", termion::cursor::Right(1));
        }
        Ok(())
    }
}
