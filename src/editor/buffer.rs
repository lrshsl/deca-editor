use crate::{msg, warn};

pub(super) type Buffer = Vec<String>;

pub trait CharBuffer {
    fn insert_char(&mut self, x: usize, y: usize, c: char);
    fn delete_char(&mut self, x: usize, y: usize);
}

impl CharBuffer for Buffer {
    fn insert_char(&mut self, x: usize, y: usize, c: char) {
        if x < self[y - 1].len() {
            self[y - 1].insert(x, c)
        } else {
            self[y - 1].push(c)
        }
        msg!("inserted {x} {y}: {}", self[y - 1]);
    }

    fn delete_char(&mut self, x: usize, y: usize) {
        msg!("removed {x} {y}");
        if x == 0 {
            warn!("cannot delete newline character");
            return;
        }
        if y <= self.len() && x <= self[y - 1].len() {
            self[y - 1].remove(x - 1);
            msg!("removed {} {}: {}", x - 1, y - 1, self[y - 1]);
        } else {
            warn!("cannot delete character at {x} {y}");
        }
    }
}
