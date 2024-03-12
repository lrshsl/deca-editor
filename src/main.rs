#![feature(exclusive_range_pattern)]

use editor_functions::EditorFunction as EdFn;
use std::collections::HashMap;
use std::io::{self, stdout};
use termion::cursor::DetectCursorPos;
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use stdio_utils::*;

mod editor_functions;
mod stdio_utils;

pub(crate) type Keymaps = Vec<HashMap<Key, EdFn>>;

pub(crate) struct Settings {
    keymaps: Keymaps,
}

impl Settings {
    fn colemak() -> Self {
        Self {
            keymaps: vec![HashMap::from([(Key::Char('a'), EdFn::MvLnBegin)])],
        }
    }
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            keymaps: vec![HashMap::new()],
        }
    }
}

fn main() -> io::Result<()> {
    let _stdout = stdout().into_raw_mode()?;

    let buf: Vec<String> = Vec::new();

    // Greet user
    msg_fullscreen("Wellcome to HellX")?;
    clear_screen();
    set_pos(1, 1);
    flush()?;

    // Start editor
    let mut editor = Editor {
        mode: Mode::Write,
        settings: Settings::default(),
        buf,
    };
    editor.run()?;

    // Bye
    msg_fullscreen("Byeye")?;

    // Return to initial state
    print!("{}{}", termion::clear::All, termion::cursor::Goto(1, 1));
    flush()
}

#[derive(Debug)]
enum Mode {
    Write,
    Move,
    Select,
}

struct Editor {
    mode: Mode,
    settings: Settings,
    buf: Vec<String>,
}

impl Editor {
    pub fn run(&mut self) -> io::Result<()> {
        loop {
            match Editor::read_key()? {
                Key::Char('q') => break Ok(()),
                Key::Esc => self.mode = Mode::Move,
                key => match self.mode {
                    Mode::Write => self.write(key)?,
                    Mode::Move => self.mv(key)?,
                    Mode::Select => {}
                },
            }
        }
    }

    fn read_key() -> io::Result<Key> {
        loop {
            if let Some(c) = io::stdin().keys().next() {
                return c;
            }
        }
    }

    fn write(&mut self, c: Key) -> io::Result<()> {
        let (x, y) = io::stdout().cursor_pos()?;
        match c {
            Key::Char('\n') => {
                self.buf.insert(y as usize, String::new());
                print!("\n{}", termion::cursor::Goto(1, y + 1));
            }
            Key::Backspace => print!("\x08"),
            Key::Char(c) => {
                buf_insert(&mut self.buf, x as usize, y as usize, c);
                print!("{c}");
            }
            key => eprint!("<<Error:Write>> Key {key:?} not implemented"),
        }
        flush()
    }

    fn mv(&self, c: Key) -> io::Result<()> {
        if let Some(fn_ptr) = self.settings.keymaps[0].get(&c) {
            fn_ptr.execute()?;
        }
        flush()
    }
}

fn buf_insert(buf: &mut Vec<String>, x: usize, y: usize, c: char) {
    if y - 1 < buf.len() {
        buf.insert(y - 1, String::from(c))
    } else {
        buf.push(String::from(c));
    }
    if x < buf[y - 1].len() {
        buf[y - 1].insert(x, c)
    } else {
        buf[y - 1].push(c)
    }
}
