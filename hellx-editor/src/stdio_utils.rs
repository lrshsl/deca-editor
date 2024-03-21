use std::io::{self, Read as _, Write as _};
use termion::{self, cursor::DetectCursorPos as _, event::Key, input::TermRead as _, terminal_size};

const MSG_BOX_WIDTH: u16 = 30;

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

#[inline]
pub fn msg(msg: &str) -> io::Result<()> {
    let (old_x, old_y) = cursor_pos()?;
    let screen = terminal_size()?;
    print!(
        "{}{}{}{}{}{}",
        termion::cursor::Goto(
            screen.0 - MSG_BOX_WIDTH,
            1),
        termion::style::Faint,
        msg,
        " ".repeat(MSG_BOX_WIDTH as usize - msg.len() - 1),
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
