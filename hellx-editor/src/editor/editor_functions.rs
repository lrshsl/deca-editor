use std::io;

use crate::cursor_pos;

pub(crate) trait Executable {
    fn execute(&self) -> io::Result<()>;
}

pub(crate) enum EditorCommand {
    Exit,
    Write,
}

pub(crate) enum EditorFunction {
    GoLnBegin,
    GoLnEnd,
    GoLnUp,
    GoLnDown,
    // GoWordBack,
    // GoWordFront,
    // GoCharBack,
    // GoCharFront,
}

impl Executable for EditorFunction {
    fn execute(&self) -> io::Result<()> {
        match self {
            EditorFunction::GoLnBegin => mv_ln_begin()?,
            EditorFunction::GoLnEnd => mv_ln_end()?,
            EditorFunction::GoLnUp => mv_ln_up(),
            EditorFunction::GoLnDown => mv_ln_down(),
            // EditorFunction::GoWordBack => mv_word_back(),
            // EditorFunction::GoWordFront => mv_word_front(),
            // EditorFunction::GoCharBack => mv_char_back(),
            // EditorFunction::GoCharFront => mv_char_front(),
        }
        Ok(())
    }
}

pub(super) fn mv_ln_begin() -> io::Result<()> {
    let (_, y) = cursor_pos()?;
    print!("{}", termion::cursor::Goto(1, y));
    Ok(())
}

pub(super) fn mv_ln_end() -> io::Result<()> {
    let (x, y) = cursor_pos()?;
    print!("{}", termion::cursor::Goto(x, y));
    Ok(())
}

pub(super) fn mv_ln_up() {
    print!("{}", termion::cursor::Up(1));
}

pub(super) fn mv_ln_down() {
    print!("{}", termion::cursor::Down(1));
}
