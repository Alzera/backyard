use crate::error::LexResult;
use crate::lexer::Lexer;
use crate::utils::get_tokens_until_right_bracket;
use crate::token::{ Token, TokenType };

pub struct ObjectAccessToken;

impl ObjectAccessToken {
  pub fn lex(lexer: &mut Lexer) -> LexResult {
    let mut tokens = vec![Token::new(TokenType::ObjectAccessBracketOpen, "{")];
    tokens.extend(get_tokens_until_right_bracket(lexer));
    tokens.push(Token::new(TokenType::ObjectAccessBracketClose, "}"));
    Ok(tokens)
  }
}
