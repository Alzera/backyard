use crate::error::LexResult;
use crate::lexer::Lexer;
use crate::token::{ Token, TokenType };

pub struct ObjectAccessToken;

impl ObjectAccessToken {
  pub fn lex(lexer: &mut Lexer) -> LexResult {
    let mut tokens = vec![Token::new(TokenType::ObjectAccessBracketOpen, "{")];
    tokens.extend(lexer.next_tokens_until_right_bracket());
    tokens.push(Token::new(TokenType::ObjectAccessBracketClose, "}"));
    Ok(tokens)
  }
}
