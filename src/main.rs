use picow::editor;
use picow::file;

fn main() -> std::io::Result<()> {
    let (lines, delimiter) = file::read_file("src/editor.rs")?;
    editor::Editor::new(lines, delimiter).run()
}
