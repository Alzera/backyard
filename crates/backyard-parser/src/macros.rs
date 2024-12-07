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

// #[macro_export]
// macro_rules! cast_lookup_result {
//   ($enum_variant:ident, $value:expr) => {
//     if let $crate::utils::LookupResultWrapper::$enum_variant(n) = $value {
//       n
//     } else {
//       return Err(ParserError::Internal);
//     }
//   };
// }

// #[macro_export]
// macro_rules! cast_lookup_result_option {
//   ($enum_variant:ident, $value:expr) => {
//     if let $crate::utils::LookupResultWrapper::$enum_variant(n) = $value {
//       Some(n)
//     } else {
//       None
//     }
//   };
// }
