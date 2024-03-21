use std::collections::HashMap;

use termion::event::Key;

use crate::editor::editor_functions::{EditorCommand as EdCmd, EditorFunction as EdFn};

use super::keymaps::{InputReaction, Keymaps};

pub(crate) struct Settings {
    pub(crate) keymaps: Keymaps,
}

impl Settings {
    pub fn colemak() -> Self {
        Self {
            keymaps: vec![HashMap::from([
                (Key::Char('q'), InputReaction::Command(EdCmd::Exit)),
                (Key::Char('a'), InputReaction::Function(EdFn::GoLnBegin)),
                (Key::Char('o'), InputReaction::Function(EdFn::GoLnEnd)),
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
