#[macro_export]
macro_rules! guard {
  ($option:expr) => {
    match $option {
      Some(value) => value,
      None => {
        return Err(ParserError::Internal);
      }
    }
  };
}
