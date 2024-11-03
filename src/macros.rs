#[macro_export]
macro_rules! guard {
  ($option:expr) => {
      match $option {
          Some(value) => value,
          None => {
              return None;
          }
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

#[macro_export]
macro_rules! guard_ok {
  ($option:expr) => {
      match $option {
          Ok(value) => value,
          Err(_) => {
              return None;
          }
      }
  };
  ($option:expr, $failure:block) => {
      match $option {
        Ok(value) => value,
        Err(_) => {
              $failure;
          }
      }
  };
}
