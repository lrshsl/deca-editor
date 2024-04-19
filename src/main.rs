#![feature(exclusive_range_pattern)]

use std::io::{self, stdout};
use stdio_utils::*;
use termion::raw::IntoRawMode;

use crate::editor::{editor::Editor, settings::Settings, editor_functions::Mode};

mod editor;
mod stdio_utils;
mod utils;

fn main() -> io::Result<()> {
    env_logger::init();

    let _stdout = stdout().into_raw_mode()?;

    let buf: Vec<String> = Vec::from([String::new()]);

    // Greet user
    msg_fullscreen("HellX Editor v0.1")?;
    clear_screen();
    set_pos(1, 1);
    flush()?;

    // Start editor
    let mut editor = Editor::new(
        Mode::Write,
        Settings::colemak_vim(),
        false,
        buf,
    );
    editor.run()?;

    // Bye
    msg_centered("Byeye")?;

    // Return to initial state
    print!("{}{}", termion::clear::All, termion::cursor::Goto(1, 1));
    flush()
}
