use crate::lexer::Lexer;
use crate::utils::{ get_char_until, get_tokens_until_right_bracket };
use crate::token::{ Token, TokenType };

pub struct VariableToken;

impl VariableToken {
  pub fn lex(lexer: &mut Lexer) -> Option<Vec<Token>> {
    let mut tokens: Vec<Token> = Vec::new();
    if let Some(next_char) = lexer.chars.get(lexer.position) {
      if *next_char == '{' {
        lexer.position += 1;
        tokens.push(Token::new(TokenType::VariableBracketOpen, "{"));
        tokens.extend(get_tokens_until_right_bracket(lexer));
        tokens.push(Token::new(TokenType::VariableBracketClose, "}"));
      } else {
        let t = get_char_until(
          &lexer.chars,
          &mut lexer.position,
          |ch, _| !(ch.is_alphanumeric() || *ch == '_')
        );
        if t == "this" {
          tokens.push(Token::new(TokenType::This, t));
        } else {
          tokens.push(Token::new(TokenType::Variable, t));
        }
      }
    }
    Some(tokens)
  }
}
