use anyhow::anyhow;

pub type GenericResult<T> = anyhow::Result<T>;

pub fn error<T>(text: &str) -> Result<T, anyhow::Error> {
    Err(anyhow!(text.to_string()))
}
