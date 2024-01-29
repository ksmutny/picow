mod file;
mod editor;

fn main() -> std::io::Result<()> {
    let (lines, _) = file::read_file("src/main.rs")?;
    editor::Editor::new(lines).run()
}
