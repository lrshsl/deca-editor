#![feature(exclusive_range_pattern)]

use std::io::{self, stdout};
use stdio_utils::*;
use termion::raw::IntoRawMode;

mod stdio_utils;
mod editor;

fn main() -> io::Result<()> {
    let _stdout = stdout().into_raw_mode()?;

    let buf: Vec<String> = Vec::new();

    // Greet user
    msg_fullscreen("Wellcome to HellX")?;
    clear_screen();
    set_pos(1, 1);
    flush()?;

    // Start editor
    let mut editor = new_editor!{
        mode: Mode::Write,
        settings: Settings::colemak(),
        should_quit: false,
        buf,
    };
    editor.run()?;

    // Bye
    msg_centered("Byeye")?;

    // Print buffer
    for (i, line) in editor.buf.iter().enumerate() {
        print!("{}{line}", termion::cursor::Goto(1, i as u16 + 1));
    }
    flush()?;
    wait_for_input()?;

    // Return to initial state
    print!("{}{}", termion::clear::All, termion::cursor::Goto(1, 1));
    flush()
}

