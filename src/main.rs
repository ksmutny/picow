mod ansi;
mod file;
mod editor;
mod commands;
mod terminal;

fn main() -> std::io::Result<()> {
    let (lines, _) = file::read_file("src/main.rs")?;
    editor::Editor::new(lines).run()
}
