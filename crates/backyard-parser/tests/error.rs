use backyard_lexer::error::LexError;
use backyard_parser::error::ParserError;

#[test]
fn basic() {
  assert_eq!(
    "Failed to parse: \"test\", \"Testing\"",
    format!("{}", ParserError::Internal { parser: "test".to_owned(), args: "Testing".to_owned() })
  );
  assert_eq!("Failed to parse: Error", format!("{}", ParserError::Failed("Error".to_owned())));
  assert_eq!("End of file", format!("{}", ParserError::LexError(LexError::Eof)));
}
