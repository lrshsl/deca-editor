use std::collections::HashMap;

use termion::event::Key;

use crate::editor::editor_functions::{EditorCommand as EdCmd, EditorFunction as EdFn};

use super::keymaps::{InputReaction, Keymaps};

pub(crate) struct Settings {
    pub(crate) keymaps: Keymaps,
}

impl Settings {
    pub fn colemak_vim() -> Self {
        Self {
            keymaps: vec![HashMap::from([
                (Key::Char('q'), InputReaction::Command(EdCmd::Exit)),
                (Key::Char('H'), InputReaction::Function(EdFn::GoLnBegin)),
                (Key::Char('I'), InputReaction::Function(EdFn::GoLnEnd)),
                (Key::Char('e'), InputReaction::Function(EdFn::GoLnUp)),
                (Key::Char('n'), InputReaction::Function(EdFn::GoLnDown)),
                (Key::Char('h'), InputReaction::Function(EdFn::GoCharLeft)),
                (Key::Char('i'), InputReaction::Function(EdFn::GoCharRight)),
                (Key::Char('w'), InputReaction::Function(EdFn::GoWordLeft)),
                (Key::Char('b'), InputReaction::Function(EdFn::GoWordRight)),
                (Key::Char(':'), InputReaction::Command(EdCmd::OpenCommandLine)),
            ])],
        }
    }
    pub fn colemak() -> Self {
        Self {
            keymaps: vec![HashMap::from([
                (Key::Char('q'), InputReaction::Command(EdCmd::Exit)),
                (Key::Char('a'), InputReaction::Function(EdFn::GoLnBegin)),
                (Key::Char('o'), InputReaction::Function(EdFn::GoLnEnd)),
                (Key::Char('e'), InputReaction::Function(EdFn::GoLnUp)),
                (Key::Char('s'), InputReaction::Function(EdFn::GoLnDown)),
                (Key::Char('t'), InputReaction::Function(EdFn::GoCharLeft)),
                (Key::Char('n'), InputReaction::Function(EdFn::GoCharRight)),
                (Key::Char(':'), InputReaction::Command(EdCmd::OpenCommandLine)),
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
