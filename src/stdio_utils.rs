use std::io::{self, Read as _, Write as _};
use termion::{self, terminal_size};

#[inline]
pub fn flush() -> io::Result<()> {
    io::stdout().flush()
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
pub fn wait_for_input() -> io::Result<()> {
    io::stdin().read_exact(&mut [0u8])
}
