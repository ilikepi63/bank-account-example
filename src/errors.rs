use anyhow::anyhow;

pub type GenericResult<T> = anyhow::Result<T>;

pub type GenericTextError = anyhow::Error;

pub fn error<T>(text: &str) -> Result<T, anyhow::Error>{
    Err(anyhow!(text.to_string()))
}
