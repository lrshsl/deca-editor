use std::collections::HashMap;

use termion::event::Key;

use crate::editor::editor_functions::{EditorCommand as EdCmd, EditorFunction as EdFn};

use super::{editor_functions::Mode, keymaps::{InputReaction, Keymaps}};

pub struct Settings {
    pub keymaps: Keymaps,
    pub word_chars: String,
}

impl Settings {
    pub fn colemak_vim() -> Self {
        Self {
            keymaps: vec![HashMap::from([
                (Key::Esc, InputReaction::Command(EdCmd::EnterMode(Mode::Move))),
                (Key::Char('l'), InputReaction::Function(EdFn::InsertLeft)),
                (Key::Char('a'), InputReaction::Function(EdFn::InsertRight)),
                (Key::Char('q'), InputReaction::Command(EdCmd::Exit)),
                (Key::Char('H'), InputReaction::Function(EdFn::GoLnBegin)),
                (Key::Char('I'), InputReaction::Function(EdFn::GoLnEnd)),
                (Key::Char('e'), InputReaction::Function(EdFn::GoLnUp)),
                (Key::Char('n'), InputReaction::Function(EdFn::GoLnDown)),
                (Key::Char('h'), InputReaction::Function(EdFn::GoCharLeft)),
                (Key::Char('i'), InputReaction::Function(EdFn::GoCharRight)),
                (Key::Char('w'), InputReaction::Function(EdFn::GoWordRight)),
                (Key::Char('b'), InputReaction::Function(EdFn::GoWordLeft)),
                (Key::Char(':'), InputReaction::Command(EdCmd::OpenCommandLine)),
            ])],
            ..Default::default()
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
            ..Default::default()
        }
    }
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            keymaps: vec![HashMap::new()],
            word_chars: "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789_".to_string(),
        }
    }
}
