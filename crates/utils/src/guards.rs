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

// #[macro_export]
// macro_rules! guard_none {
//   ($option:expr) => {
//       match $option {
//           Some(value) => value,
//           None => {
//               return None;
//           }
//       }
//   };
// }

#[macro_export]
macro_rules! guard_ok {
  //   ($option:expr) => {
  //       match $option {
  //           Ok(value) => value,
  //           Err(err) => {
  //               return Err(err);
  //           }
  //       }
  //   };
  ($option:expr, $failure:block) => {
      match $option {
        Ok(value) => value,
        Err(_) => {
              $failure;
          }
      }
  };
}

// #[macro_export]
// macro_rules! guard_parser_statement {
//   ($name:expr, $option:expr) => {
//       match $option {
//         Ok(value) => {
//             match value {
//                 Some(v) => v,
//                 None => {
//                     return Err(ParserError::internal($name, args))
//                 }
//             }
//         },
//         Err(err) => {
//             return Err(err);
//         }
//       }
//   };
// }

// #[macro_export]
// macro_rules! guard_ok_some {
//   ($option:expr, $failure:block) => {
//       match $option {
//         Ok(value) => {
//             match value {
//                 Some(v) => v,
//                 None => {
//                     $failure
//                 }
//             }
//         },
//         Err(err) => {
//             return Err(err);
//         }
//       }
//   };
// }
