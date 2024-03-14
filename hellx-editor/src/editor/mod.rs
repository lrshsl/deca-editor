use proc_macro::TokenStream;

pub(crate) mod editor;
pub(crate) mod settings;

mod buffer;
mod editor_functions;

#[derive(Debug)]
pub enum Mode {
    Write,
    Move,
    Select,
}

// #[macro_export]
// macro_rules! new_editor {
//     ($($arg:tt)),*) => {
//         $crate::editor::Editor::new($($arg)*)
//     };
// }

#[proc_macro]
pub fn new_editor(input: TokenStream) -> TokenStream {
    print!("{input}");
    Editor {
    }
}
