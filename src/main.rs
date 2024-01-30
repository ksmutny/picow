mod ansi;
mod file;
mod editor;
mod commands;
mod terminal;
mod winapi;

fn main() -> std::io::Result<()> {
    let (lines, delimiter) = file::read_file("src/main.rs")?;
    editor::Editor::new(lines, delimiter).run()
}
