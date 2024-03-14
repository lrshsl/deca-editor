

pub(crate) enum InputReaction {
    Command(EdCmd),
    Function(EdFn),
}

pub(crate) type Keymaps = Vec<HashMap<Key, InputReaction>>;
