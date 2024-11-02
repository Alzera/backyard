use crate::lexer::utils::get_char_until;
use crate::lexer::token::{ Token, TokenType };

pub struct CommentBlockToken {}

impl CommentBlockToken {
  pub fn parse(chars: &Vec<char>, position: &mut usize) -> String {
    let mut close: Vec<char> = Vec::new();
    let comment: String = get_char_until(&chars, position, |ch, endpos| {
      close.push(*ch);
      if close.len() > 2 {
        close.remove(0);
      }
      let is_close = close.iter().collect::<String>() == "*/";
      if is_close {
        *endpos -= 1;
      }
      is_close
    });
    *position += 2;
    comment
  }

  pub fn lex(chars: &Vec<char>, position: &mut usize) -> Option<Vec<Token>> {
    let comment: String = CommentBlockToken::parse(&chars, position);
    Some(vec![Token::new(TokenType::CommentBlock, comment)])
  }
}
