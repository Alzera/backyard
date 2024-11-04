use std::fmt::Display;

use crate::lexer::token::{ Token, TokenType };
use crate::lexer::internal::{
  commentblock::CommentBlockToken,
  commentdoc::CommentDocToken,
  commentline::CommentLineToken,
  keywords::KeywordToken,
  magics::MagicToken,
  number::NumberToken,
  types::TypeToken,
  variable::VariableToken,
};
use crate::lexer::utils::get_char_until;

use super::internal::objectaccess::ObjectAccessToken;
use super::internal::string::StringToken;

pub struct Lexer {
  pub chars: Vec<char>,
  pub position: usize,
}

impl Lexer {
  pub fn new(input: &str) -> Self {
    Lexer {
      chars: input.chars().collect(),
      position: 0,
    }
  }

  fn unable_to_handle<T: Display>(d: T) -> Option<Vec<Token>> {
    println!("Unknown character: {}", d);
    None
  }

  fn until<F>(&mut self, mut callback: F) -> String where F: FnMut(&char) -> bool {
    self.position -= 1;
    get_char_until(&self.chars, &mut self.position, |ch, _| callback(ch))
  }

  pub fn next_tokens(&mut self, skip_whitespace: bool) -> Option<Vec<Token>> {
    if skip_whitespace {
      get_char_until(&self.chars, &mut self.position, |ch, _| !ch.is_whitespace());
    }

    let cc = self.chars.get(self.position);
    if cc.is_none() {
      return None;
    }

    let current_char = cc.unwrap();
    self.position += 1;

    // println!("current_char: {:?}", current_char);

    match current_char {
      '$' => VariableToken::lex(self),
      c if c.is_whitespace() =>
        Some(vec![Token::new(TokenType::Whitespace, current_char.to_string())]),
      c if c.is_digit(10) => NumberToken::lex(&self.chars, &mut self.position),
      c if c.is_alphabetic() || *c == '_' => {
        let t = self.until(|ch| !(ch.is_alphanumeric() || *ch == '_'));
        if t.starts_with("__") && t.ends_with("__") && MagicToken::is_magic(&t) {
          return Some(vec![Token::new(TokenType::Magic, t)]);
        }
        if TypeToken::is_type(&t) {
          return Some(vec![Token::new(TokenType::Type, t)]);
        }
        if KeywordToken::is_keyword(&t) {
          return KeywordToken::lex(&t, self);
        }
        return Some(vec![Token::new(TokenType::Identifier, t)]);
      }
      '=' => {
        let t = self.until(|ch| !['=', '>'].contains(ch));
        match t.as_str() {
          "===" => Some(vec![Token::new(TokenType::IsIdentical, "===")]),
          "==" => Some(vec![Token::new(TokenType::IsEqual, "==")]),
          "=" => Some(vec![Token::new(TokenType::Assignment, "=")]),
          "=>" => Some(vec![Token::new(TokenType::Arrow, "=>")]),
          _ => Lexer::unable_to_handle(t),
        }
      }
      '&' => {
        let t = self.until(|ch| !['=', '&'].contains(ch));
        match t.as_str() {
          "&=" => Some(vec![Token::new(TokenType::BitwiseAndAssignment, "&=")]),
          "&&" => Some(vec![Token::new(TokenType::BooleanAnd, "&&")]),
          "&" => {
            let t = self.until(|ch| !['.', '&', '$'].contains(ch));
            match t.as_str() {
              "&$" => {
                if let Some(tokens) = VariableToken::lex(self) {
                  let mut tokens = tokens.clone();
                  tokens.insert(0, Token::new(TokenType::Reference, "&"));
                  return Some(tokens);
                } else {
                  None
                }
              }
              "&...$" => {
                if let Some(tokens) = VariableToken::lex(self) {
                  let mut tokens = tokens.clone();
                  tokens.insert(0, Token::new(TokenType::Ellipsis, "..."));
                  tokens.insert(0, Token::new(TokenType::Reference, "&"));
                  return Some(tokens);
                } else {
                  None
                }
              }
              "&" => Some(vec![Token::new(TokenType::BitwiseAnd, "&")]),
              _ => Lexer::unable_to_handle(t),
            }
          }
          _ => Lexer::unable_to_handle(t),
        }
      }
      '#' => {
        let t = self.until(|ch| !['['].contains(ch));
        match t.as_str() {
          "#[" => Some(vec![Token::new(TokenType::Attribute, "#[")]),
          "#" => CommentLineToken::lex(&self.chars, &mut self.position),
          _ => Lexer::unable_to_handle(t),
        }
      }
      '?' => {
        let t = self.until(|ch| !['?', '>', '=', '-', '{'].contains(ch));
        match t.as_str() {
          "?>" => Some(vec![Token::new(TokenType::CloseTag, "?>")]),
          "?->" => Some(vec![Token::new(TokenType::NullsafeObjectAccess, "?->")]),
          "?->{" => Some(vec![Token::new(TokenType::NullsafeObjectAccessBracketOpen, "?->{")]),
          "??=" => Some(vec![Token::new(TokenType::CoalesceAssignment, "??=")]),
          "??" => Some(vec![Token::new(TokenType::Coalesce, "??")]),
          "?" => Some(vec![Token::new(TokenType::QuestionMark, "?")]),
          _ => Lexer::unable_to_handle(t),
        }
      }
      '%' => {
        let t = self.until(|ch| !['=', '>'].contains(ch));
        match t.as_str() {
          "%>" => Some(vec![Token::new(TokenType::CloseTagShort, "%>")]),
          "%=" => Some(vec![Token::new(TokenType::ModulusAssignment, "%=")]),
          "%" => Some(vec![Token::new(TokenType::Modulus, "%")]),
          _ => Lexer::unable_to_handle(t),
        }
      }
      '^' => {
        let t = self.until(|ch| !['='].contains(ch));
        match t.as_str() {
          "^=" => Some(vec![Token::new(TokenType::BitwiseXorAssignment, "^=")]),
          "^" => Some(vec![Token::new(TokenType::BitwiseXor, "^")]),
          _ => Lexer::unable_to_handle(t),
        }
      }
      '*' => {
        let t = self.until(|ch| !['*', '/', '='].contains(ch));
        match t.as_str() {
          "**=" => Some(vec![Token::new(TokenType::ExponentiationAssignment, "**=")]),
          "*=" => Some(vec![Token::new(TokenType::MultiplicationAssignment, "*=")]),
          // "*/" => Some(vec![Token::new(TokenType::CommentClose, "*/")]),
          "**" => Some(vec![Token::new(TokenType::Exponentiation, "**")]),
          "*" => Some(vec![Token::new(TokenType::Multiplication, "*")]),
          _ => Lexer::unable_to_handle(t),
        }
      }
      '/' => {
        let t = self.until(|ch| !['/', '*', '='].contains(ch));
        match t.as_str() {
          "/=" => Some(vec![Token::new(TokenType::DivisionAssignment, "/=")]),
          "/**" => CommentDocToken::lex(&self.chars, &mut self.position),
          "/*" => CommentBlockToken::lex(&self.chars, &mut self.position),
          "//" => CommentLineToken::lex(&self.chars, &mut self.position),
          "/" => Some(vec![Token::new(TokenType::Division, "/")]),
          _ => Lexer::unable_to_handle(t),
        }
      }
      '.' => {
        let t = self.until(|ch| !['.', '='].contains(ch));
        match t.as_str() {
          ".=" => Some(vec![Token::new(TokenType::ConcatenationAssignment, ".=")]),
          "..." => Some(vec![Token::new(TokenType::Ellipsis, "...")]),
          "." => {
            let mut t = get_char_until(&self.chars, &mut self.position, |ch, _| !ch.is_digit(10));
            if t.len() == 0 {
              Some(vec![Token::new(TokenType::Concatenation, ".")])
            } else {
              t.insert(0, '.');
              Some(vec![Token::new(TokenType::Number, t.to_string())])
            }
          }
          _ => Lexer::unable_to_handle(t),
        }
      }
      '|' => {
        let t = self.until(|ch| !['|', '='].contains(ch));
        match t.as_str() {
          "|=" => Some(vec![Token::new(TokenType::BitwiseOrAssignment, "|=")]),
          "||" => Some(vec![Token::new(TokenType::BooleanOr, "||")]),
          "|" => Some(vec![Token::new(TokenType::BitwiseOr, "|")]),
          _ => Lexer::unable_to_handle(t),
        }
      }
      '-' => {
        let t = self.until(|ch| !['-', '=', '>', '{'].contains(ch));
        match t.as_str() {
          "-=" => Some(vec![Token::new(TokenType::SubtractionAssignment, "-=")]),
          "->{" => ObjectAccessToken::lex(self),
          "->" => Some(vec![Token::new(TokenType::ObjectAccess, "->")]),
          "--" => {
            let is_post = match self.chars.get(self.position) {
              Some(t) => t.is_whitespace() || [';', ',', ')', ']', '}', '?'].contains(&t),
              None => true,
            };
            if is_post {
              return Some(vec![Token::new(TokenType::PostDecrement, "--")]);
            }
            return Some(vec![Token::new(TokenType::PreDecrement, "--")]);
          }
          "-" => Some(vec![Token::new(TokenType::Subtraction, "-")]),
          _ => Lexer::unable_to_handle(t),
        }
      }
      '>' => {
        let t = self.until(|ch| !['>', '='].contains(ch));
        match t.as_str() {
          ">>=" => Some(vec![Token::new(TokenType::BitwiseShiftRightAssignment, ">>=")]),
          ">=" => Some(vec![Token::new(TokenType::IsGreaterOrEqual, ">=")]),
          ">>" => Some(vec![Token::new(TokenType::BitwiseShiftRight, ">>")]),
          ">" => Some(vec![Token::new(TokenType::IsGreater, ">")]),
          _ => Lexer::unable_to_handle(t),
        }
      }
      '<' => {
        let t = self.until(|ch| !['?', '=', '>', '<', 'p', 'h', '%'].contains(ch));
        match t.as_str() {
          "<%" => Some(vec![Token::new(TokenType::OpenTagShort, "<%")]),
          "<?php" => Some(vec![Token::new(TokenType::OpenTag, "<?php")]),
          "<?=" => Some(vec![Token::new(TokenType::OpenTagEcho, "<?=")]),
          "<=>" => Some(vec![Token::new(TokenType::Spaceship, "<=>")]),
          "<>" => Some(vec![Token::new(TokenType::IsNotEqual, "<>")]),
          "<=" => Some(vec![Token::new(TokenType::IsLesserOrEqual, "<=")]),
          "<<=" => Some(vec![Token::new(TokenType::BitwiseShiftLeftAssignment, "<<=")]),
          "<<<" => Some(vec![Token::new(TokenType::HeredocStart, "<<<")]),
          "<<" => Some(vec![Token::new(TokenType::BitwiseShiftLeft, "<<")]),
          "<" => Some(vec![Token::new(TokenType::IsLesser, "<")]),
          _ => Lexer::unable_to_handle(t),
        }
      }
      ':' => {
        let t = self.until(|ch| ![':'].contains(ch));
        match t.as_str() {
          "::" => Some(vec![Token::new(TokenType::DoubleColon, "::")]),
          ":" => Some(vec![Token::new(TokenType::Colon, ":")]),
          _ => Lexer::unable_to_handle(t),
        }
      }
      '!' => {
        let t = self.until(|ch| !['!', '='].contains(ch));
        match t.as_str() {
          "!==" => Some(vec![Token::new(TokenType::IsNotIdentical, "!==")]),
          "!=" => Some(vec![Token::new(TokenType::IsNotEqual, "!=")]),
          "!" => Some(vec![Token::new(TokenType::BooleanNegate, "!")]),
          _ => Lexer::unable_to_handle(t),
        }
      }
      '+' => {
        let t = self.until(|ch| !['+', '='].contains(ch));
        match t.as_str() {
          "+=" => Some(vec![Token::new(TokenType::AdditionAssignment, "+=")]),
          "++" => {
            let is_post = match self.chars.get(self.position) {
              Some(t) => t.is_whitespace() || [';', ',', ')', ']', '}', '?'].contains(&t),
              None => true,
            };
            if is_post {
              return Some(vec![Token::new(TokenType::PostIncrement, "++")]);
            }
            return Some(vec![Token::new(TokenType::PreIncrement, "++")]);
          }
          "+" => Some(vec![Token::new(TokenType::Addition, "+")]),
          _ => Lexer::unable_to_handle(t),
        }
      }
      '(' => Some(vec![Token::new(TokenType::LeftParenthesis, "(")]),
      ')' => Some(vec![Token::new(TokenType::RightParenthesis, ")")]),
      '{' => Some(vec![Token::new(TokenType::LeftCurlyBracket, "{")]),
      // '{' => {
      //   let t = self.until(|ch| !['$'].contains(ch));
      //   match t.as_str() {
      //     // "{$" => Some(vec![Token::new(TokenType::AdvanceInterpolationOpen, "{$")]),
      //     "{" => Some(vec![Token::new(TokenType::LeftCurlyBracket, "{")]),
      //     _ => Lexer::unable_to_handle(t),
      //   }
      // }
      '}' => Some(vec![Token::new(TokenType::RightCurlyBracket, "}")]),
      '[' => Some(vec![Token::new(TokenType::LeftSquareBracket, "[")]),
      ']' => Some(vec![Token::new(TokenType::RightSquareBracket, "]")]),
      '`' => StringToken::lex(self, '`'),
      '"' => StringToken::lex(self, '"'),
      '\'' => StringToken::lex(self, '\''),
      '\\' => Some(vec![Token::new(TokenType::BackSlash, "\\")]),
      ',' => Some(vec![Token::new(TokenType::Comma, ",")]),
      ';' => Some(vec![Token::new(TokenType::Semicolon, ";")]),
      '~' => Some(vec![Token::new(TokenType::BooleanNegate, "~")]),
      // '\n' => Some(vec![Token::new(TokenType::LineBreak, "\n")]),
      _ => Lexer::unable_to_handle(current_char),
    }
  }
}
