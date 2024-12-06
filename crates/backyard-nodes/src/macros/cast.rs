#[macro_export]
macro_rules! cast_node {
  ($enum_variant:ident, $value:expr) => {
    if let NodeWrapper::$enum_variant(n) = $value {
      n
    } else {
      return;
    }
  };
}
