mod file;
mod editor;
mod terminal;

fn main() -> std::io::Result<()> {
    let (lines, delimiter) = file::read_file("src/editor.rs")?;
    editor::Editor::new(lines, delimiter).run()
}
