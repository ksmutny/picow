use std::fs;

use picow::editor::{content::EditorContent, Editor};

fn main() -> std::io::Result<()> {
    let file_content = fs::read_to_string("src/editor/mod.rs")?;
    let content = EditorContent::parse(&file_content);
    Editor::new(content).run()
}
