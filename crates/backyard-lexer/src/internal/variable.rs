use crate::error::LexResult;
use crate::lexer::{ ControlSnapshot, Lexer };
use crate::token::{ Token, TokenType };

pub struct VariableToken;

impl VariableToken {
  pub fn lex(lexer: &mut Lexer, snapshot: &ControlSnapshot) -> LexResult {
    if let Some(next_char) = lexer.control.peek_char(None) {
      if next_char == '{' {
        lexer.control.next_char();
        lexer.tokens.push(Token::new(TokenType::VariableBracketOpen, "${".into(), snapshot));
        lexer.next_tokens_until_right_bracket()?;
        lexer.control.next_char();
        lexer.tokens.push(
          Token::new(TokenType::VariableBracketClose, "}".into(), lexer.control.get_last_snapshot())
        );
      } else {
        let t = lexer.control.next_char_until(|_, ch, _| !(ch.is_alphanumeric() || *ch == '_'));
        if t == "this" {
          lexer.tokens.push(Token::new(TokenType::This, t, snapshot));
        } else {
          lexer.tokens.push(Token::new(TokenType::Variable, t, snapshot));
        }
      }
    }
    Ok(())
  }
}
