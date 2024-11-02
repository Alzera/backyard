use crate::lexer::token::{ Token, TokenType };
use crate::lexer::internal::commentblock::CommentBlockToken;

pub struct CommentDocToken {}

impl CommentDocToken {
  pub fn lex(chars: &Vec<char>, position: &mut usize) -> Option<Vec<Token>> {
    let comment: String = CommentBlockToken::parse(&chars, position);
    Some(vec![Token::new(TokenType::CommentDoc, comment)])
  }
}
