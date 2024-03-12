use std::io;

use termion::{self, cursor::DetectCursorPos};

pub enum EditorFunction {
    MvLnBegin,
    MvLnEnd,
    MvLnUp,
    MvLnDown,
    // MvWordBack,
    // MvWordFront,
    // MvCharBack,
    // MvCharFront,
}

impl EditorFunction {
    pub fn execute(&self) -> io::Result<()> {
        match self {
            EditorFunction::MvLnBegin => mv_ln_begin(),
            EditorFunction::MvLnEnd => mv_ln_end(),
            EditorFunction::MvLnUp => {
                mv_ln_up();
                Ok(())
            }
            EditorFunction::MvLnDown => {
                mv_ln_down();
                Ok(())
            }
            // EditorFunction::MvWordBack => mv_word_back(),
            // EditorFunction::MvWordFront => mv_word_front(),
            // EditorFunction::MvCharBack => mv_char_back(),
            // EditorFunction::MvCharFront => mv_char_front(),
        }
    }
}

pub fn mv_ln_begin() -> io::Result<()> {
    let (_, y) = io::stdout().cursor_pos()?;
    print!("{}", termion::cursor::Goto(1, y));
    Ok(())
}

pub fn mv_ln_end() -> io::Result<()> {
    let (x, y) = io::stdout().cursor_pos()?;
    print!("{}", termion::cursor::Goto(x, y));
    Ok(())
}

pub fn mv_ln_up() {
    print!("{}", termion::cursor::Up(1));
}

pub fn mv_ln_down() {
    print!("{}", termion::cursor::Down(1));
}
