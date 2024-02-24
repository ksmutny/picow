use picow::app;

fn main() -> std::io::Result<()> {
    app::start("src/editor/mod.rs")
}
