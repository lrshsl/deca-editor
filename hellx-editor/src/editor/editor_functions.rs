use std::io;

use termion::{self, cursor::DetectCursorPos};

pub(crate) trait Executable {
    fn execute(&self) -> io::Result<()>;
}

pub(crate) enum EditorCommand {
    Exit,
    Write,
}

pub(crate) enum EditorFunction {
    MvLnBegin,
    MvLnEnd,
    MvLnUp,
    MvLnDown,
    // MvWordBack,
    // MvWordFront,
    // MvCharBack,
    // MvCharFront,
}

impl Executable for EditorFunction {
    fn execute(&self) -> io::Result<()> {
        match self {
            EditorFunction::MvLnBegin => mv_ln_begin()?,
            EditorFunction::MvLnEnd => mv_ln_end()?,
            EditorFunction::MvLnUp => mv_ln_up(),
            EditorFunction::MvLnDown => mv_ln_down(),
            // EditorFunction::MvWordBack => mv_word_back(),
            // EditorFunction::MvWordFront => mv_word_front(),
            // EditorFunction::MvCharBack => mv_char_back(),
            // EditorFunction::MvCharFront => mv_char_front(),
        }
        Ok(())
    }
}

pub(super) fn mv_ln_begin() -> io::Result<()> {
    let (_, y) = io::stdout().cursor_pos()?;
    print!("{}", termion::cursor::Goto(1, y));
    Ok(())
}

pub(super) fn mv_ln_end() -> io::Result<()> {
    let (x, y) = io::stdout().cursor_pos()?;
    print!("{}", termion::cursor::Goto(x, y));
    Ok(())
}

pub(super) fn mv_ln_up() {
    print!("{}", termion::cursor::Up(1));
}

pub(super) fn mv_ln_down() {
    print!("{}", termion::cursor::Down(1));
}
