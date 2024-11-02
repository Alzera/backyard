pub struct MagicToken {}

impl MagicToken {
  const KEYS: [&str; 8] = [
    "__CLASS__",
    "__DIR__",
    "__FILE__",
    "__FUNCTION__",
    "__LINE__",
    "__METHOD__",
    "__NAMESPACE__",
    "__TRAIT__",
  ];

  pub fn is_magic(input: &String) -> bool {
    Self::KEYS.contains(&input.as_str())
  }
}
