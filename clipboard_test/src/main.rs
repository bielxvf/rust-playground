extern crate clipboard;

use clipboard::ClipboardProvider;
use clipboard::ClipboardContext;

use std::{thread, time};

fn main() {
    let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
    ctx.set_contents("Clipboard in 2 lines of code".to_owned()).unwrap();
    let sleep_timer = time::Duration::from_secs(10);
    thread::sleep(sleep_timer);
}
