use std::io;

use super::editor::Editor;

pub(crate) trait Executable {
    fn execute(&self, editor: &mut Editor) -> io::Result<()>;
}

pub(crate) enum EditorCommand {
    Exit,
    Write,
    OpenFile,
    OpenCommandLine,
}

pub(crate) enum EditorFunction {
    GoLnBegin,
    GoLnEnd,
    GoLnUp,
    GoLnDown,
    GoWordLeft,
    GoWordRight,
    GoCharLeft,
    GoCharRight,
}
