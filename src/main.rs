use std::io::{self, stdout, Read, Write};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use termion::terminal_size;

fn main() -> io::Result<()> {
    let _stdout = stdout().into_raw_mode()?;

    // Greet user
    msg_fullscreen("Wellcome to HellX")?;
    clear_screen();
    flush()?;

    // Start editor
    let mut editor = Editor { mode: Mode::Write };
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
}

impl Editor {
    pub fn run(&mut self) -> io::Result<()> {
        loop {
            match self.read_key()? {
                Key::Char('q') => break Ok(()),
                Key::Esc => self.mode = Mode::Move,
                key => match &self.mode {
                    &Mode::Write => Editor::write(key)?,
                    &Mode::Move => Editor::mv(key)?,
                    mode => eprint!("<<Error>> Mode {mode:?} not implemented"),
                },
            }
        }
    }

    fn read_key(&self) -> io::Result<Key> {
        loop {
            if let Some(c) = io::stdin().keys().next() {
                return c;
            }
        }
    }

    fn write(c: Key) -> io::Result<()> {
        match c {
            Key::Char('\n') => println!(),
            Key::Ctrl('h') => print!("\r"),
            Key::Char(c) => print!("{c}"),
            key => eprint!("<<Error:Write>> Key {key:?} not implemented"),
        }
        flush()
    }

    fn mv(c: Key) -> io::Result<()> {
        match c {
            Key::Up => print!("{}", termion::cursor::Up(1)),
            Key::Left => print!("{}", termion::cursor::Left(1)),
            c => eprint!("<<Error:Move>> Key {c:?} not implemented"),
        }
        flush()
    }
}

#[inline]
fn flush() -> io::Result<()> {
    io::stdout().flush()
}

#[inline]
fn msg_fullscreen(msg: &str) -> io::Result<()> {
    clear_screen();
    msg_centered(msg)?;
    flush()?;
    wait_for_input()
}

#[inline]
fn clear_screen() {
    print!("{}", termion::clear::All);
}

#[inline]
fn msg_centered(msg: &str) -> io::Result<()> {
    let (x, y) = terminal_size()?;
    print!(
        "{}{}{}",
        termion::clear::All,
        termion::cursor::Goto((x - msg.len() as u16) / 2, y / 2),
        msg
    );
    Ok(())
}

#[inline]
fn wait_for_input() -> io::Result<()> {
    io::stdin().read_exact(&mut [0u8])
}
