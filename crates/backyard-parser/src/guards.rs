#[macro_export]
macro_rules! guard {
  ($option:expr) => {
      match $option {
          Some(value) => value,
          None => { return; }
      }
  };
  ($option:expr, $default:expr) => {
      match $option {
          Some(value) => value,
          None => $default,
      }
  };
  ($option:expr, $failure:block) => {
      match $option {
          Some(value) => value,
          None => {
              $failure;
          }
      }
  };
}
