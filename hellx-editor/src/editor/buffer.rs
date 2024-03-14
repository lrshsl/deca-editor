use crate::msg;

pub(super) type Buffer = Vec<String>;

pub trait Bufferable {
    fn insert_char(&mut self, x: usize, y: usize, c: char);
    fn delete_char(&mut self, x: usize, y: usize);
}

impl Bufferable for Buffer {
    fn insert_char(&mut self, x: usize, y: usize, c: char) {
        msg!("{x} {y}: {c}");
        if y - 1 < self.len() {
            self.insert(y - 1, String::from(c))
        } else {
            self.push(String::from(c));
        }
        if x < self[y - 1].len() {
            self[y - 1].insert(x, c)
        } else {
            self[y - 1].push(c)
        }
    }

    fn delete_char(&mut self, x: usize, y: usize) {
        if y - 1 < self.len() && x - 1 < self[y - 1].len() {
            self[y - 1].remove(x - 1);
        }
    }
}
