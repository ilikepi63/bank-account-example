//! This is where we'd likely keep our error(exception) library. There didn't seem to be a need to
//! expand on referencing different errors, so this was specifically built as generic as possible for the sake 
//! of this project.
use anyhow::anyhow;

pub type GenericResult<T> = anyhow::Result<T>;

pub fn error<T>(text: &str) -> Result<T, anyhow::Error> {
    Err(anyhow!(text.to_string()))
}
