use std::collections::HashMap;

use termion::event::Key;

use crate::{
    editor::editor_functions::{EditorCommand as EdCmd, EditorFunction as EdFn},
    InputReaction, Keymaps,
};

pub(crate) struct Settings {
    pub(crate) keymaps: Keymaps,
}

impl Settings {
    fn colemak() -> Self {
        Self {
            keymaps: vec![HashMap::from([
                (Key::Char('q'), InputReaction::Command(EdCmd::Exit)),
                (Key::Char('a'), InputReaction::Function(EdFn::MvLnBegin)),
            ])],
        }
    }
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            keymaps: vec![HashMap::new()],
        }
    }
}
