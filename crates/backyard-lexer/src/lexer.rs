use crate::error::{ LexError, LexResult };
use crate::token::{ Token, TokenType };
use crate::internal::{
  comment::CommentToken,
  keywords::KeywordToken,
  magics::MagicToken,
  number::NumberToken,
  types::TypeToken,
  variable::VariableToken,
};
use crate::utils::get_char_until;

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

  fn unable_to_handle(d: &str) -> LexResult {
    println!("Unknown character: {}", d);
    Err(LexError::Unrecognized(d.to_string()))
  }

  fn until<F>(&mut self, mut callback: F) -> String where F: FnMut(&char) -> bool {
    self.position -= 1;
    get_char_until(&self.chars, &mut self.position, |ch, _| callback(ch))
  }

  pub fn next_tokens(&mut self, skip_whitespace: bool) -> LexResult {
    if skip_whitespace {
      get_char_until(&self.chars, &mut self.position, |ch, _| !ch.is_whitespace());
    }

    let cc = self.chars.get(self.position);
    if cc.is_none() {
      return Err(LexError::Eof);
    }

    let current_char = cc.unwrap();
    self.position += 1;

    // println!("current_char: {:?}", current_char);

    match current_char {
      '$' => VariableToken::lex(self),
      c if c.is_whitespace() =>
        Ok(vec![Token::new(TokenType::Whitespace, current_char.to_string())]),
      c if c.is_digit(10) => NumberToken::lex(&self.chars, &mut self.position),
      c if c.is_alphabetic() || *c == '_' => {
        let t = self.until(|ch| !(ch.is_alphanumeric() || *ch == '_'));
        if t.starts_with("__") && t.ends_with("__") && MagicToken::is_magic(&t) {
          return Ok(vec![Token::new(TokenType::Magic, t)]);
        }
        if TypeToken::is_type(&t) {
          return Ok(vec![Token::new(TokenType::Type, t)]);
        }
        if KeywordToken::is_keyword(&t) {
          return KeywordToken::lex(&t, self);
        }
        return Ok(vec![Token::new(TokenType::Identifier, t)]);
      }
      '=' => {
        let t = self.until(|ch| !['=', '>'].contains(ch));
        match t.as_str() {
          "===" => Ok(vec![Token::new(TokenType::IsIdentical, "===")]),
          "==" => Ok(vec![Token::new(TokenType::IsEqual, "==")]),
          "=" => Ok(vec![Token::new(TokenType::Assignment, "=")]),
          "=>" => Ok(vec![Token::new(TokenType::Arrow, "=>")]),
          _ => Lexer::unable_to_handle(&t),
        }
      }
      '&' => {
        let t = self.until(|ch| !['&', '=', '$'].contains(ch));
        match t.as_str() {
          "&=" => Ok(vec![Token::new(TokenType::BitwiseAndAssignment, "&=")]),
          "&&" => Ok(vec![Token::new(TokenType::BooleanAnd, "&&")]),
          "&$" => {
            let last_position = self.position.clone();
            if let Ok(tokens) = VariableToken::lex(self) {
              let mut tokens = tokens.clone();
              tokens.insert(0, Token::new(TokenType::Reference, "&"));
              return Ok(tokens);
            } else {
              self.position = last_position;
              return Lexer::unable_to_handle(&t);
            }
          }
          "&" => Ok(vec![Token::new(TokenType::BitwiseAnd, "&")]),
          _ => Lexer::unable_to_handle(&t),
        }
      }
      '#' => {
        let t = self.until(|ch| !['#', '['].contains(ch));
        match t.as_str() {
          "#[" => Ok(vec![Token::new(TokenType::Attribute, "#[")]),
          "#" => CommentToken::lex_line(&self.chars, &mut self.position),
          _ => Lexer::unable_to_handle(&t),
        }
      }
      '?' => {
        let t = self.until(|ch| !['?', '>', '=', '-', '{', ':'].contains(ch));
        match t.as_str() {
          "?:" => Ok(vec![Token::new(TokenType::Elvis, "?:")]),
          "?>" => Ok(vec![Token::new(TokenType::CloseTag, "?>")]),
          "?->" => Ok(vec![Token::new(TokenType::NullsafeObjectAccess, "?->")]),
          "?->{" => Ok(vec![Token::new(TokenType::NullsafeObjectAccessBracketOpen, "?->{")]),
          "??=" => Ok(vec![Token::new(TokenType::CoalesceAssignment, "??=")]),
          "??" => Ok(vec![Token::new(TokenType::Coalesce, "??")]),
          "?" => Ok(vec![Token::new(TokenType::QuestionMark, "?")]),
          _ => Lexer::unable_to_handle(&t),
        }
      }
      '%' => {
        let t = self.until(|ch| !['%', '=', '>'].contains(ch));
        match t.as_str() {
          "%>" => Ok(vec![Token::new(TokenType::CloseTagShort, "%>")]),
          "%=" => Ok(vec![Token::new(TokenType::ModulusAssignment, "%=")]),
          "%" => Ok(vec![Token::new(TokenType::Modulus, "%")]),
          _ => Lexer::unable_to_handle(&t),
        }
      }
      '^' => {
        let t = self.until(|ch| !['^', '='].contains(ch));
        match t.as_str() {
          "^=" => Ok(vec![Token::new(TokenType::BitwiseXorAssignment, "^=")]),
          "^" => Ok(vec![Token::new(TokenType::BitwiseXor, "^")]),
          _ => Lexer::unable_to_handle(&t),
        }
      }
      '*' => {
        let t = self.until(|ch| !['*', '='].contains(ch));
        match t.as_str() {
          "**=" => Ok(vec![Token::new(TokenType::ExponentiationAssignment, "**=")]),
          "*=" => Ok(vec![Token::new(TokenType::MultiplicationAssignment, "*=")]),
          "**" => Ok(vec![Token::new(TokenType::Exponentiation, "**")]),
          "*" => Ok(vec![Token::new(TokenType::Multiplication, "*")]),
          _ => Lexer::unable_to_handle(&t),
        }
      }
      '/' => {
        let last_position = self.position.clone();
        let t = self.until(|ch| !['/', '*', '='].contains(ch));
        if t.starts_with("//") {
          self.position = last_position + 1;
          return CommentToken::lex_line(&self.chars, &mut self.position);
        }
        match t.as_str() {
          "/=" => Ok(vec![Token::new(TokenType::DivisionAssignment, "/=")]),
          "/**" => CommentToken::lex_doc(&self.chars, &mut self.position),
          "/*" => CommentToken::lex_block(&self.chars, &mut self.position),
          "/" => Ok(vec![Token::new(TokenType::Division, "/")]),
          _ => Lexer::unable_to_handle(&t),
        }
      }
      '.' => {
        let t = self.until(|ch| !['.', '='].contains(ch));
        match t.as_str() {
          ".=" => Ok(vec![Token::new(TokenType::ConcatenationAssignment, ".=")]),
          "..." => Ok(vec![Token::new(TokenType::Ellipsis, "...")]),
          "." => {
            let mut t = get_char_until(&self.chars, &mut self.position, |ch, _| !ch.is_digit(10));
            if t.len() == 0 {
              Ok(vec![Token::new(TokenType::Concatenation, ".")])
            } else {
              t.insert(0, '.');
              Ok(vec![Token::new(TokenType::Number, t.to_string())])
            }
          }
          _ => Lexer::unable_to_handle(&t),
        }
      }
      '|' => {
        let t = self.until(|ch| !['|', '='].contains(ch));
        match t.as_str() {
          "|=" => Ok(vec![Token::new(TokenType::BitwiseOrAssignment, "|=")]),
          "||" => Ok(vec![Token::new(TokenType::BooleanOr, "||")]),
          "|" => Ok(vec![Token::new(TokenType::BitwiseOr, "|")]),
          _ => Lexer::unable_to_handle(&t),
        }
      }
      '-' => {
        let t = self.until(|ch| !['-', '=', '>', '{'].contains(ch));
        match t.as_str() {
          "-=" => Ok(vec![Token::new(TokenType::SubtractionAssignment, "-=")]),
          "->{" => ObjectAccessToken::lex(self),
          "->" => Ok(vec![Token::new(TokenType::ObjectAccess, "->")]),
          "--" => {
            let is_post = match self.chars.get(self.position) {
              Some(t) => t.is_whitespace() || [';', ',', ')', ']', '}', '?'].contains(&t),
              None => true,
            };
            if is_post {
              return Ok(vec![Token::new(TokenType::PostDecrement, "--")]);
            }
            return Ok(vec![Token::new(TokenType::PreDecrement, "--")]);
          }
          "-" => Ok(vec![Token::new(TokenType::Subtraction, "-")]),
          _ => Lexer::unable_to_handle(&t),
        }
      }
      '>' => {
        let t = self.until(|ch| !['>', '='].contains(ch));
        match t.as_str() {
          ">>=" => Ok(vec![Token::new(TokenType::BitwiseShiftRightAssignment, ">>=")]),
          ">=" => Ok(vec![Token::new(TokenType::IsGreaterOrEqual, ">=")]),
          ">>" => Ok(vec![Token::new(TokenType::BitwiseShiftRight, ">>")]),
          ">" => Ok(vec![Token::new(TokenType::IsGreater, ">")]),
          _ => Lexer::unable_to_handle(&t),
        }
      }
      '<' => {
        let t = self.until(|ch| !['<', '?', '=', '>', 'p', 'h', '%'].contains(ch));
        match t.as_str() {
          "<%" => Ok(vec![Token::new(TokenType::OpenTagShort, "<%")]),
          "<?php" => Ok(vec![Token::new(TokenType::OpenTag, "<?php")]),
          "<?=" => Ok(vec![Token::new(TokenType::OpenTagEcho, "<?=")]),
          "<=>" => Ok(vec![Token::new(TokenType::Spaceship, "<=>")]),
          "<>" => Ok(vec![Token::new(TokenType::IsNotEqual, "<>")]),
          "<=" => Ok(vec![Token::new(TokenType::IsLesserOrEqual, "<=")]),
          "<<=" => Ok(vec![Token::new(TokenType::BitwiseShiftLeftAssignment, "<<=")]),
          "<<<" => Ok(vec![Token::new(TokenType::HeredocStart, "<<<")]),
          "<<" => Ok(vec![Token::new(TokenType::BitwiseShiftLeft, "<<")]),
          "<" => Ok(vec![Token::new(TokenType::IsLesser, "<")]),
          _ => Lexer::unable_to_handle(&t),
        }
      }
      ':' => {
        let t = self.until(|ch| ![':'].contains(ch));
        match t.as_str() {
          "::" => Ok(vec![Token::new(TokenType::DoubleColon, "::")]),
          ":" => Ok(vec![Token::new(TokenType::Colon, ":")]),
          _ => Lexer::unable_to_handle(&t),
        }
      }
      '!' => {
        let t = self.until(|ch| !['!', '='].contains(ch));
        match t.as_str() {
          "!==" => Ok(vec![Token::new(TokenType::IsNotIdentical, "!==")]),
          "!=" => Ok(vec![Token::new(TokenType::IsNotEqual, "!=")]),
          "!" => Ok(vec![Token::new(TokenType::BooleanNegate, "!")]),
          _ => Lexer::unable_to_handle(&t),
        }
      }
      '+' => {
        let t = self.until(|ch| !['+', '='].contains(ch));
        match t.as_str() {
          "+=" => Ok(vec![Token::new(TokenType::AdditionAssignment, "+=")]),
          "++" => {
            let is_post = match self.chars.get(self.position) {
              Some(t) => t.is_whitespace() || [';', ',', ')', ']', '}', '?'].contains(&t),
              None => true,
            };
            if is_post {
              return Ok(vec![Token::new(TokenType::PostIncrement, "++")]);
            }
            return Ok(vec![Token::new(TokenType::PreIncrement, "++")]);
          }
          "+" => Ok(vec![Token::new(TokenType::Addition, "+")]),
          _ => Lexer::unable_to_handle(&t),
        }
      }
      '(' => Ok(vec![Token::new(TokenType::LeftParenthesis, "(")]),
      ')' => Ok(vec![Token::new(TokenType::RightParenthesis, ")")]),
      '{' => Ok(vec![Token::new(TokenType::LeftCurlyBracket, "{")]),
      '}' => Ok(vec![Token::new(TokenType::RightCurlyBracket, "}")]),
      '[' => Ok(vec![Token::new(TokenType::LeftSquareBracket, "[")]),
      ']' => Ok(vec![Token::new(TokenType::RightSquareBracket, "]")]),
      '`' => StringToken::lex(self, '`'),
      '"' => StringToken::lex(self, '"'),
      '\'' => StringToken::lex(self, '\''),
      '\\' => Ok(vec![Token::new(TokenType::BackSlash, "\\")]),
      ',' => Ok(vec![Token::new(TokenType::Comma, ",")]),
      ';' => Ok(vec![Token::new(TokenType::Semicolon, ";")]),
      '~' => Ok(vec![Token::new(TokenType::BooleanNegate, "~")]),
      '@' => {
        if let Some(next) = self.chars.get(self.position) {
          if !next.is_whitespace() {
            return Ok(vec![Token::new(TokenType::AtSign, "@")]);
          }
        }
        Lexer::unable_to_handle("@")
      }
      // '\n' => Ok(vec![Token::new(TokenType::LineBreak, "\n")]),
      _ => Lexer::unable_to_handle(&current_char.to_string()),
    }
  }
}
