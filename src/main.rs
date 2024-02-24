use std::fs;

use picow::{editor::{content::EditorContent, Editor}, terminal};

fn main() -> std::io::Result<()> {
    let file_content = fs::read_to_string("src/editor/mod.rs")?;
    let content = EditorContent::parse(&file_content);

    let console_mode = terminal::init()?;
    Editor::new(content).run()?;
    terminal::restore_console_mode(console_mode)
}
