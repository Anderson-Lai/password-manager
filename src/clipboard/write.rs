use copypasta::{ClipboardContext, ClipboardProvider};
use crate::platform::linux::IS_LINUX;

pub fn write_to_clipboard(value: &str) -> Result<(), ()> {

    if IS_LINUX {
        eprintln!("Clipboard writing for linux is not supported!");
        return Err(());
    }

    let context = ClipboardContext::new();
    let mut context = match context {
        Ok(c) => c,
        Err(_) => {
            eprintln!("Error while creating context for clipboard writing!");
            return Err(());
        }
    };

    match context.set_contents(value.to_string()) {
        Ok(_) => Ok(()),
        Err(_) => {
            eprintln!("Error while writing value to clipboard!");
            Err(())
        }
    }
}
