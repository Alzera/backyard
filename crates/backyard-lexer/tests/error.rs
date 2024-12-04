use backyard_lexer::error::LexError;

#[test]
fn unrecognized() {
  let error = LexError::Unrecognized { token: "�".to_string(), line: 1, column: 7 };
  assert_eq!("Unrecognized character '�' at line 1, column 7", format!("{}", error));
}

#[test]
fn eof() {
  assert_eq!("End of file", format!("{}", LexError::Eof));
}
