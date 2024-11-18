#[macro_export]
macro_rules! cast_node {
  ($enum_variant:path, $value:expr) => {
    if let $enum_variant(n) = $value {
      n
    } else {
      return;
    }
  };
}
