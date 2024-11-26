use crate::error::LexResult;
use crate::lexer::{ ControlSnapshot, Lexer };
use crate::token::{ Token, TokenType };

pub struct VariableToken;

impl VariableToken {
  pub fn lex(lexer: &mut Lexer, snapshot: &ControlSnapshot) -> LexResult {
    let mut tokens: Vec<Token> = Vec::new();
    if let Some(next_char) = lexer.control.peek_char(None) {
      if next_char == '{' {
        lexer.control.next_char();
        tokens.push(Token::new(TokenType::VariableBracketOpen, "${", snapshot));
        tokens.extend(lexer.next_tokens_until_right_bracket());
        tokens.push(
          Token::new(TokenType::VariableBracketClose, "}", &lexer.control.get_snapshot())
        );
      } else {
        let t = lexer.control.next_char_until(|_, ch, _| !(ch.is_alphanumeric() || *ch == '_'));
        if t == "this" {
          tokens.push(Token::new(TokenType::This, t, snapshot));
        } else {
          // $ skipped on value but not on snapshot
          tokens.push(Token::new(TokenType::Variable, t, snapshot));
        }
      }
    }
    Ok(tokens)
  }
}
