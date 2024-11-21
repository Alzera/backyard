pub struct TypeToken;

impl TypeToken {
  const KEYS: [&str; 9] = [
    // "array",
    "bool",
    "boolean",
    "real",
    "double",
    "float",
    "int",
    "integer",
    "object",
    "String",
    // "null",
  ];

  pub fn is_type(input: &String) -> bool {
    Self::KEYS.contains(&input.as_str())
  }
}
