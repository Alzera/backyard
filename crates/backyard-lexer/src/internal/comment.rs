use crate::utils::get_char_until;
use crate::token::{ Token, TokenType };

pub struct CommentToken;

impl CommentToken {
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

  pub fn lex_doc(chars: &Vec<char>, position: &mut usize) -> Option<Vec<Token>> {
    let comment: String = CommentToken::parse(&chars, position);
    Some(vec![Token::new(TokenType::CommentDoc, comment)])
  }

  pub fn lex_block(chars: &Vec<char>, position: &mut usize) -> Option<Vec<Token>> {
    let comment: String = CommentToken::parse(&chars, position);
    Some(vec![Token::new(TokenType::CommentBlock, comment)])
  }

  pub fn lex_line(chars: &Vec<char>, position: &mut usize) -> Option<Vec<Token>> {
    let comment: String = get_char_until(&chars, position, |ch, _| ['\n'].contains(ch));
    Some(vec![Token::new(TokenType::CommentLine, comment)])
  }
}
