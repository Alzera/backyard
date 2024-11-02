use crate::lexer::lexer::Lexer;
use crate::lexer::utils::get_tokens_until_right_bracket;
use crate::lexer::token::{ Token, TokenType };

pub struct ObjectAccessToken {}

impl ObjectAccessToken {
  pub fn lex(lexer: &mut Lexer) -> Option<Vec<Token>> {
    let mut tokens = vec![Token::new(TokenType::ObjectAccessBracketOpen, "{")];
    tokens.extend(get_tokens_until_right_bracket(lexer));
    tokens.push(Token::new(TokenType::ObjectAccessBracketClose, "}"));
    Some(tokens)
  }
}
