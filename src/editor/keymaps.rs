use std::collections::HashMap;

use termion::event::Key;

use super::editor_functions::{EditorCommand as EdCmd, EditorFunction as EdFn};

pub(crate) enum InputReaction {
    Command(EdCmd),
    Function(EdFn),
}

pub(crate) type Keymaps = Vec<HashMap<Key, InputReaction>>;
