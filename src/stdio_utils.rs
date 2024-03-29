use log::info;
use std::io::{self, Read as _, Write as _};
use termion::{
    self, cursor::DetectCursorPos as _, event::Key, input::TermRead as _, terminal_size,
};

use crate::utils::Rect;

const MSG_BOX_WIDTH: u16 = 30;
const MSG_BOX_HEIGHT: u16 = 6;

pub enum ScrOp {
    WaitForInput,
    WriteMsg,
    WriteMsgCentered,
    WriteError,
    WriteFullscreenMsg,
}

#[macro_export]
macro_rules! msg {
    ($($arg:tt)*) => {
        $crate::stdio_utils::msg(&format!($($arg)*))
    };
}

#[macro_export]
macro_rules! warn {
    ($($arg:tt)*) => {
        $crate::stdio_utils::warn(&format!($($arg)*))
    };
}

#[macro_export]
macro_rules! emsg {
    ($($arg:tt)*) => {
        $crate::stdio_utils::emsg(&format!($($arg)*))
    };
}

#[macro_export]
macro_rules! msg_fullscreen {
    ($($arg:tt)*) => {
        $crate::stdio_utils::msg_fullscreen(&format!($($arg)*))
    };
}

#[macro_export]
macro_rules! msg_centered {
    ($($arg:tt)*) => {
        $crate::stdio_utils::msg_centered(&format!($($arg)*))
    };
}

#[inline]
pub fn read_key() -> io::Result<Key> {
    loop {
        if let Some(c) = io::stdin().keys().next() {
            return c;
        }
    }
}

#[inline]
pub fn flush() -> io::Result<()> {
    io::stdout().flush()
}

#[inline]
pub fn cursor_pos() -> io::Result<(u16, u16)> {
    io::stdout().cursor_pos()
}

/// Wraps the message by putting `Goto`s in the right places. Cuts
/// off everything that doesn't fit into `rect.x * rect.y`.
/// Note: Cursor position is set to bottom rigth corner of `rect`.
fn fit_msg_to_box(msg: &str, rect: Rect) -> String {
    let (w, h) = (rect.width as usize, rect.height as usize);

    if msg.len() <= w {
        return msg.to_string() + " ".repeat(w - msg.len()).as_str();
    }
    if msg.len() > w * h {
        log::warn!(
            "Message too big, overflow is cut off: {} > buffer size ({})",
            msg.len(),
            rect.width as usize * rect.height as usize
        );
    }

    let mut output = String::new();

    // Go to beginning
    for i in 0..rect.height {
        // Go to the correct position
        output += &termion::cursor::Goto(rect.x, rect.y + i).to_string();

        let start_of_line = i * w as u16;

        // No characters left of `msg` -> just fill with spaces in order to
        // overwrite possible previous message
        if start_of_line >= msg.len() as u16 {
            output += " ".repeat(w).as_str();
            continue;
        }

        let start_of_next_line = start_of_line + w as u16;

        // Print remainder of message if message ends in this line
        if start_of_next_line >= msg.len() as u16 {
            output += &msg[start_of_line as usize..];
            output += " ".repeat(w - msg.len() % w).as_str();
            continue;
        }

        // Else fill the line with the next `w` characters of the message
        output += &msg[start_of_line as usize..start_of_next_line as usize - 1];
    }

    // Overflow was never appended to `output`
    output
}

#[inline]
pub fn msg(msg: &str) -> io::Result<()> {
    let (old_x, old_y) = cursor_pos()?;
    let screen = terminal_size()?;
    let msg = fit_msg_to_box(
        msg,
        Rect::from_points(
            (screen.0 - MSG_BOX_WIDTH, 1),
            (screen.0, MSG_BOX_HEIGHT + 1),
        ),
    );
    print!(
        "{}{}{}{}{}",
        termion::cursor::Goto(screen.0 - MSG_BOX_WIDTH, 1),
        termion::style::Faint,
        msg,
        termion::style::Reset,
        termion::cursor::Goto(old_x, old_y)
    );
    flush()
}

#[inline]
pub fn warn(msg: &str) -> io::Result<()> {
    let (old_x, old_y) = cursor_pos()?;
    let screen = terminal_size()?;
    print!(
        "{}{} Warning: {}{}{}{}",
        termion::cursor::Goto(1, screen.1 - 1),
        termion::style::Italic,
        msg,
        " ".repeat(screen.0 as usize - msg.len() - " Warning: ".len() - 1),
        termion::style::Reset,
        termion::cursor::Goto(old_x, old_y)
    );
    flush()
}

#[inline]
pub fn emsg(msg: &str) -> io::Result<()> {
    let (old_x, old_y) = cursor_pos()?;
    let screen = terminal_size()?;
    print!(
        "{}{} Error: {}{}{}{}",
        termion::cursor::Goto(1, screen.1 - 1),
        termion::style::Bold,
        msg,
        " ".repeat(screen.0 as usize - msg.len() - " Error: ".len() - 1),
        termion::style::Reset,
        termion::cursor::Goto(old_x, old_y)
    );
    flush()
}

#[inline]
pub fn msg_fullscreen(msg: &str) -> io::Result<()> {
    clear_screen();
    msg_centered(msg)?;
    flush()?;
    wait_for_input()
}

#[inline]
pub fn set_pos(x: u16, y: u16) {
    print!("{}", termion::cursor::Goto(x, y));
}

#[inline]
pub fn clear_screen() {
    print!("{}", termion::clear::All);
}

#[inline]
pub fn msg_centered(msg: &str) -> io::Result<()> {
    let (old_x, old_y) = cursor_pos()?;
    let (x, y) = terminal_size()?;
    print!(
        "{}{}{}{}",
        termion::clear::All,
        termion::cursor::Goto((x - msg.len() as u16) / 2, y / 2),
        msg,
        termion::cursor::Goto(old_x, old_y),
    );
    Ok(())
}

#[inline]
pub fn wait_for_input() -> io::Result<()> {
    io::stdin().read_exact(&mut [0u8])
}
