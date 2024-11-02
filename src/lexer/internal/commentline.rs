use crate::lexer::utils::get_char_until;
use crate::lexer::token::{ Token, TokenType };

pub struct CommentLineToken {}

impl CommentLineToken {
  pub fn lex(chars: &Vec<char>, position: &mut usize) -> Option<Vec<Token>> {
    let comment: String = get_char_until(&chars, position, |ch, _| ['\n'].contains(ch));
    Some(vec![Token::new(TokenType::CommentLine, comment)])
  }
}
