pub(crate) mod editor;
pub(crate) mod settings;
pub(crate) mod keymaps;

mod buffer;
mod editor_functions;

#[derive(Debug)]
pub enum Mode {
    Write,
    Move,
    Select,
}
