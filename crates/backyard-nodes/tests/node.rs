use backyard_nodes::node::{ Inheritance, Modifier, Quote, UseItemModifier, Visibility };

#[test]
fn use_item_modifier() {
  assert_eq!(UseItemModifier::try_from("const"), Ok(UseItemModifier::Const));
  assert_eq!(UseItemModifier::try_from("function"), Ok(UseItemModifier::Function));
  assert!(UseItemModifier::try_from("none").is_err());

  assert_eq!("const", format!("{}", UseItemModifier::Const));
  assert_eq!("function", format!("{}", UseItemModifier::Function));
}

#[test]
fn modifier() {
  assert_eq!(Modifier::try_from("static"), Ok(Modifier::Static));
  assert_eq!(Modifier::try_from("readonly"), Ok(Modifier::Readonly));
  assert!(Modifier::try_from("none").is_err());

  assert_eq!("static", format!("{}", Modifier::Static));
  assert_eq!("readonly", format!("{}", Modifier::Readonly));
}

#[test]
fn quote() {
  assert_eq!(Quote::try_from("'"), Ok(Quote::Single));
  assert_eq!(Quote::try_from("\""), Ok(Quote::Double));
  assert_eq!(Quote::try_from("`"), Ok(Quote::Backtick));
  assert!(Quote::try_from("none").is_err());

  assert_eq!("'", format!("{}", Quote::Single));
  assert_eq!("\"", format!("{}", Quote::Double));
  assert_eq!("`", format!("{}", Quote::Backtick));
}

#[test]
fn inheritance() {
  assert_eq!(Inheritance::try_from("abstract"), Ok(Inheritance::Abstract));
  assert_eq!(Inheritance::try_from("final"), Ok(Inheritance::Final));
  assert!(Inheritance::try_from("none").is_err());

  assert_eq!("abstract", format!("{}", Inheritance::Abstract));
  assert_eq!("final", format!("{}", Inheritance::Final));
}

#[test]
fn visibility() {
  assert_eq!(Visibility::try_from("private"), Ok(Visibility::Private));
  assert_eq!(Visibility::try_from("private(get)"), Ok(Visibility::PrivateGet));
  assert_eq!(Visibility::try_from("private(set)"), Ok(Visibility::PrivateSet));
  assert_eq!(Visibility::try_from("protected"), Ok(Visibility::Protected));
  assert_eq!(Visibility::try_from("protected(get)"), Ok(Visibility::ProtectedGet));
  assert_eq!(Visibility::try_from("protected(set)"), Ok(Visibility::ProtectedSet));
  assert_eq!(Visibility::try_from("public"), Ok(Visibility::Public));
  assert_eq!(Visibility::try_from("public(get)"), Ok(Visibility::PublicGet));
  assert_eq!(Visibility::try_from("public(set)"), Ok(Visibility::PublicSet));
  assert!(Visibility::try_from("none").is_err());

  assert_eq!("private", format!("{}", Visibility::Private));
  assert_eq!("private(get)", format!("{}", Visibility::PrivateGet));
  assert_eq!("private(set)", format!("{}", Visibility::PrivateSet));
  assert_eq!("protected", format!("{}", Visibility::Protected));
  assert_eq!("protected(get)", format!("{}", Visibility::ProtectedGet));
  assert_eq!("protected(set)", format!("{}", Visibility::ProtectedSet));
  assert_eq!("public", format!("{}", Visibility::Public));
  assert_eq!("public(get)", format!("{}", Visibility::PublicGet));
  assert_eq!("public(set)", format!("{}", Visibility::PublicSet));
}
