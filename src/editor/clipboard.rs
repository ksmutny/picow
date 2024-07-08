use clipboard::{ClipboardContext, ClipboardProvider};

pub fn copy_to_clipboard(text: String) {
    let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
    ctx.set_contents(text).unwrap();
}
