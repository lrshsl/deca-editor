
#[derive(Debug, Clone)]
pub enum Mode {
    Write,
    Overwrite,
    Replace { start: u16, end: u16 },
    Move,
    Select,
}

#[derive(Debug, Clone)]
pub(crate) enum EditorCommand {
    Exit,
    Write,
    OpenFile,
    OpenCommandLine,
    EnterMode(Mode),
}

#[derive(Debug, Clone)]
pub(crate) enum EditorFunction {
    InsertLeft,
    InsertRight,
    GoLnBegin,
    GoLnEnd,
    GoLnUp,
    GoLnDown,
    GoWordLeft,
    GoWordRight,
    GoCharLeft,
    GoCharRight,
}
