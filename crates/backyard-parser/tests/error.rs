use backyard_lexer::{ error::LexError, token::{ Token, TokenType } };
use backyard_parser::error::ParserError;

#[test]
fn basic() {
  assert_eq!("End of file", format!("{}", ParserError::LexError(LexError::Eof)));
  assert_eq!("Internal parser error", format!("{}", ParserError::Internal));
  assert_eq!("End of file", format!("{}", ParserError::Eof));
  assert_eq!(
    "Unexpected character 'and' at line 0, column 0",
    format!(
      "{}",
      ParserError::UnexpectedToken(Token {
        token_type: TokenType::And,
        value: "and".into(),
        line: 0,
        column: 0,
        offset: 0,
      })
    )
  );
}
