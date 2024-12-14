use bstr::BString;
use bumpalo::Bump;

use crate::error::{ LexError, LexResult };
use crate::internal::inline::InlineToken;
use crate::token::{ Token, TokenType };
use crate::internal::{
  comment::CommentToken,
  keywords::KeywordToken,
  number::NumberToken,
  string::StringToken,
  variable::VariableToken,
};

#[derive(Debug)]
pub(crate) struct ControlSnapshot {
  pub(crate) line: usize,
  pub(crate) column: usize,
  pub(crate) offset: usize,
}

#[derive(Debug)]
pub(crate) struct Control {
  chars: BString,
  position: usize,
  pub(crate) line: usize,
  pub(crate) column: usize,
  pub(crate) last_snapshot: ControlSnapshot,
}

impl Control {
  pub(crate) fn new(chars: BString) -> Self {
    Control {
      chars,
      position: 0,
      line: 1,
      column: 0,
      last_snapshot: ControlSnapshot { line: 1, column: 0, offset: 0 },
    }
  }

  pub(crate) fn get_len(&self) -> usize {
    self.chars.len()
  }

  pub(crate) fn get_position(&self) -> usize {
    self.position
  }

  pub(crate) fn consume(&mut self, len: usize) {
    for _ in 0..len {
      self.next_char();
    }
  }

  pub(crate) fn get_snapshot(&self) -> ControlSnapshot {
    ControlSnapshot { line: self.line, column: self.column, offset: self.position }
  }

  pub(crate) fn get_last_snapshot(&self) -> &ControlSnapshot {
    &self.last_snapshot
  }

  pub(crate) fn peek_char(&mut self, pos: Option<usize>) -> Option<&u8> {
    let p = if let Some(pos) = pos { pos } else { self.position };
    self.chars.get(p)
  }

  pub(crate) fn peek_char_n(&mut self, pos: Option<usize>, n: usize) -> Option<BString> {
    let p = if let Some(pos) = pos { pos } else { self.position };
    self.chars.get(p..p + n).map(|x| BString::new(x.to_vec()))
  }

  pub(crate) fn next_char(&mut self) -> Option<&u8> {
    if let Some(c) = self.chars.get(self.position) {
      self.last_snapshot = self.get_snapshot();
      self.position += 1;
      if *c == b'\n' {
        self.line += 1;
        self.column = 0;
      } else {
        self.column += 1;
      }
      Some(c)
    } else {
      None
    }
  }

  pub(crate) fn next_char_until<F>(&mut self, take_prev_len: usize, mut until: F) -> BString
    where F: FnMut(&mut Control, u8, &mut usize) -> bool
  {
    let start_position = self.position - take_prev_len;
    let mut end_position = self.position;
    while let Some(ch) = self.chars.get(end_position) {
      if until(self, *ch, &mut end_position) {
        break;
      }
      end_position += 1;
    }

    let result = BString::new(self.chars[start_position..end_position].to_vec());
    while self.position < end_position {
      self.next_char();
    }
    result
  }

  pub(crate) fn error_unrecognized(&self, t: String) -> LexError {
    LexError::Unrecognized { token: t, line: self.line, column: self.column }
  }
}

// #[cfg(test)]
// mod tests {
//   use super::Control;

//   #[test]
//   fn control() {
//     let mut control = Control::new("<?php\necho 'hello world';\n?>");
//     assert_eq!(Some(b'<'), control.next_char());
//     assert_eq!(Some(b'?'), control.next_char());

//     let until = control.next_char_until(0, | ch, _| *ch == bb'\'');
//     assert_eq!("php\necho ", until);

//     let snapshot = control.get_snapshot();
//     assert_eq!(2, snapshot.line);
//     assert_eq!(5, snapshot.column);
//     assert_eq!(11, snapshot.offset);

//     control.consume(5);
//     let until = control.next_char_until(0, | ch, _| *ch == bb'\'');
//     assert_eq!("o world", until);
//   }
// }

#[derive(Debug, PartialEq)]
pub enum SeriesCheckerMode {
  String,
  Inline,
  Heredoc,
}

#[derive(Debug)]
pub struct SeriesChecker<'a> {
  list: BString,
  againsts: &'a [&'a str],
  mode: SeriesCheckerMode,
}

impl<'a> SeriesChecker<'a> {
  pub fn new(againsts: &'a [&'a str], mode: SeriesCheckerMode) -> Self {
    Self { list: BString::new(vec![]), againsts, mode }
  }

  pub fn push(&mut self, ch: u8) {
    if self.mode == SeriesCheckerMode::Heredoc {
      if ch == b'\n' {
        self.list.clear();
      } else {
        self.list.push(ch);
      }
      return;
    }
    if ch.is_whitespace() {
      self.list.clear();
    } else {
      self.list.push(ch);
    }
  }

  pub fn check(&mut self) -> Option<&'a str> {
    let text = self.list.to_string();
    if self.mode == SeriesCheckerMode::Heredoc {
      if let Some(label) = self.againsts.first() {
        return if text.trim() == *label { Some(label.to_owned().into()) } else { None };
      }
    } else if let Some(valid) = self.againsts.iter().find(|i| text.ends_with(*i)) {
      if !self.is_escaped(self.list.len() - valid.len()) {
        return Some(valid.to_owned().into());
      }
    }
    None
  }

  pub fn is_escaped(&self, index: usize) -> bool {
    if self.mode != SeriesCheckerMode::String {
      return false;
    }
    self.list[..index]
      .iter()
      .rev()
      .take_while(|x| **x == b'\\')
      .count() % 2 == 1
  }

  pub fn is_last_escaped(&self) -> bool {
    self.is_escaped(self.list.len() - 1)
  }
}

const MAGIC_KEYWORDS: &[&'static [u8]] = &[
  b"__CLASS__",
  b"__DIR__",
  b"__FILE__",
  b"__FUNCTION__",
  b"__LINE__",
  b"__METHOD__",
  b"__NAMESPACE__",
  b"__TRAIT__",
  b"__PROPERTY__",
];

const MAGIC_METHOD_KEYWORDS: &[&'static [u8]] = &[
  b"__construct",
  b"__destruct",
  b"__call",
  b"__callStatic",
  b"__get",
  b"__set",
  b"__isset",
  b"__unset",
  b"__sleep",
  b"__wakeup",
  b"__serialize",
  b"__unserialize",
  b"__toString",
  b"__invoke",
  b"__set_state",
  b"__clone",
  b"__debugInfo",
];

const TYPE_KEYWORDS: &[&'static [u8]] = &[
  // "array",
  b"bool",
  b"boolean",
  b"real",
  b"double",
  b"float",
  b"int",
  b"integer",
  b"object",
  b"String",
  b"string",
  b"mixed",
  b"void",
  // "null",
];

#[derive(Debug)]
pub struct Lexer<'a> {
  pub(crate) tokens: bumpalo::collections::Vec<'a, Token>,
  pub(crate) control: Control,
}

impl<'a> Lexer<'a> {
  pub fn new(arena: &'a Bump, input: &str) -> Self {
    Lexer {
      tokens: bumpalo::collections::Vec::new_in(arena),
      control: Control::new(BString::new(input.as_bytes().to_vec())),
    }
  }

  pub fn next_tokens_until_right_bracket(&mut self) -> LexResult {
    let mut level = 1;
    while let Some(next_token) = self.control.peek_char(None) {
      match next_token {
        b'{' => {
          level += 1;
          self.next_tokens(true)?;
        }
        b'}' => {
          level -= 1;
          if level == 0 {
            break;
          }
          self.next_tokens(true)?;
        }
        _ => {
          self.next_tokens(true)?;
        }
      }
    }
    Ok(())
  }

  fn until<F>(&mut self, mut callback: F) -> BString where F: FnMut(u8) -> bool {
    self.control.next_char_until(1, |_, ch, _| callback(ch))
  }

  pub fn start(&mut self) -> LexResult {
    loop {
      let result = self.next_tokens(true);
      if let Err(err) = result {
        if err == LexError::Eof {
          break;
        }
        return Err(err);
      }
    }
    Ok(())
  }

  pub fn next_tokens(&mut self, skip_whitespace: bool) -> LexResult {
    if skip_whitespace {
      self.control.next_char_until(0, |_, ch, _| !ch.is_whitespace());
    }

    let snapshot = &self.control.get_snapshot();
    let current_char = if let Some(current_char) = self.control.next_char() {
      current_char.to_owned()
    } else {
      return Err(LexError::Eof);
    };

    match current_char {
      b'$' => VariableToken::lex(self, snapshot),
      c if c.is_ascii_digit() => NumberToken::lex(self, &current_char, snapshot),
      c if c.is_alphabetic() || c == b'_' => {
        let t = self.until(|ch| !(ch.is_alphanumeric() || ch == b'_' || ch == b'\\'));
        let t_slice = t.as_slice();
        if MAGIC_KEYWORDS.contains(&t_slice) {
          self.tokens.push(Token::new(TokenType::Magic, t, snapshot));
          Ok(())
        } else if MAGIC_METHOD_KEYWORDS.contains(&t_slice) {
          self.tokens.push(Token::new(TokenType::MagicMethod, t, snapshot));
          Ok(())
        } else if TYPE_KEYWORDS.contains(&t_slice) {
          self.tokens.push(Token::new(TokenType::Type, t, snapshot));
          Ok(())
        } else if KeywordToken::is_keyword(t_slice) {
          KeywordToken::lex(self, t, snapshot)
        } else {
          self.tokens.push(Token::new(TokenType::Identifier, t, snapshot));
          Ok(())
        }
      }
      b'=' => {
        let t = self.until(|ch| ![b'=', b'>', b'&'].contains(&ch));
        match t.as_slice() {
          b"===" => {
            self.tokens.push(Token::new(TokenType::IsIdentical, "===".into(), snapshot));
            Ok(())
          }
          b"==" => {
            self.tokens.push(Token::new(TokenType::IsEqual, "==".into(), snapshot));
            Ok(())
          }
          b"=" => {
            self.tokens.push(Token::new(TokenType::Assignment, "=".into(), snapshot));
            Ok(())
          }
          b"=>" => {
            self.tokens.push(Token::new(TokenType::Arrow, "=>".into(), snapshot));
            Ok(())
          }
          b"=&" => {
            self.tokens.push(Token::new(TokenType::ReferenceAssignment, "=&".into(), snapshot));
            Ok(())
          }
          _ => Err(self.control.error_unrecognized(t.to_string().split_off(1))),
        }
      }
      b'&' => {
        let t = self.until(|ch| ![b'&', b'='].contains(&ch));
        match t.as_slice() {
          b"&=" => {
            self.tokens.push(Token::new(TokenType::BitwiseAndAssignment, "&=".into(), snapshot));
            Ok(())
          }
          b"&&" => {
            self.tokens.push(Token::new(TokenType::BooleanAnd, "&&".into(), snapshot));
            Ok(())
          }
          b"&" => {
            self.tokens.push(Token::new(TokenType::BitwiseAnd, "&".into(), snapshot));
            Ok(())
          }
          _ => Err(self.control.error_unrecognized(t.to_string().split_off(1))),
        }
      }
      b'#' => {
        let t = self.until(|ch| ![b'#', b'['].contains(&ch));
        match t.as_slice() {
          b"#[" => {
            self.tokens.push(Token::new(TokenType::Attribute, "#[".into(), snapshot));
            Ok(())
          }
          _ => CommentToken::lex_line(self, t.len() - 1, snapshot),
        }
      }
      b'?' => {
        let t = self.until(|ch| ![b'?', b'>', b'=', b'-', b':'].contains(&ch));
        match t.as_slice() {
          b"?:" => {
            self.tokens.push(Token::new(TokenType::Elvis, "?:".into(), snapshot));
            Ok(())
          }
          b"?->" => {
            self.tokens.push(Token::new(TokenType::NullsafeObjectAccess, "?->".into(), snapshot));
            Ok(())
          }
          b"??=" => {
            self.tokens.push(Token::new(TokenType::CoalesceAssignment, "??=".into(), snapshot));
            Ok(())
          }
          b"??" => {
            self.tokens.push(Token::new(TokenType::Coalesce, "??".into(), snapshot));
            Ok(())
          }
          b"?" => {
            self.tokens.push(Token::new(TokenType::QuestionMark, "?".into(), snapshot));
            Ok(())
          }
          c if c.starts_with(b"?>") => {
            if t.len() > 2 {
              self.control.position -= t.len() - 2;
            }
            InlineToken::lex(self, snapshot)
          }
          _ => Err(self.control.error_unrecognized(t.to_string().split_off(1))),
        }
      }
      b'%' => {
        let t = self.until(|ch| ![b'%', b'='].contains(&ch));
        match t.as_slice() {
          b"%=" => {
            self.tokens.push(Token::new(TokenType::ModulusAssignment, "%=".into(), snapshot));
            Ok(())
          }
          b"%" => {
            self.tokens.push(Token::new(TokenType::Modulus, "%".into(), snapshot));
            Ok(())
          }
          _ => Err(self.control.error_unrecognized(t.to_string().split_off(1))),
        }
      }
      b'^' => {
        let t = self.until(|ch| ![b'^', b'='].contains(&ch));
        match t.as_slice() {
          b"^=" => {
            self.tokens.push(Token::new(TokenType::BitwiseXorAssignment, "^=".into(), snapshot));
            Ok(())
          }
          b"^" => {
            self.tokens.push(Token::new(TokenType::BitwiseXor, "^".into(), snapshot));
            Ok(())
          }
          _ => Err(self.control.error_unrecognized(t.to_string().split_off(1))),
        }
      }
      b'*' => {
        let t = self.until(|ch| ![b'*', b'='].contains(&ch));
        match t.as_slice() {
          b"**=" => {
            self.tokens.push(
              Token::new(TokenType::ExponentiationAssignment, "**=".into(), snapshot)
            );
            Ok(())
          }
          b"*=" => {
            self.tokens.push(
              Token::new(TokenType::MultiplicationAssignment, "*=".into(), snapshot)
            );
            Ok(())
          }
          b"**" => {
            self.tokens.push(Token::new(TokenType::Exponentiation, "**".into(), snapshot));
            Ok(())
          }
          b"*" => {
            self.tokens.push(Token::new(TokenType::Multiplication, "*".into(), snapshot));
            Ok(())
          }
          _ => Err(self.control.error_unrecognized(t.to_string().split_off(1))),
        }
      }
      b'/' => {
        let t = self.until(|ch| ![b'/', b'*', b'='].contains(&ch));
        match t.as_slice() {
          b"/=" => {
            self.tokens.push(Token::new(TokenType::DivisionAssignment, "/=".into(), snapshot));
            Ok(())
          }
          c if c.starts_with(b"/**") => CommentToken::lex_doc(self, t.len() - 3, snapshot),
          c if c.starts_with(b"/*") => CommentToken::lex_block(self, t.len() - 2, snapshot),
          c if c.starts_with(b"//") => CommentToken::lex_line(self, t.len() - 2, snapshot),
          b"/" => {
            self.tokens.push(Token::new(TokenType::Division, "/".into(), snapshot));
            Ok(())
          }
          _ => Err(self.control.error_unrecognized(t.to_string().split_off(1))),
        }
      }
      b'.' => {
        let t = self.until(|ch| ![b'.', b'='].contains(&ch));
        match t.as_slice() {
          b".=" => {
            self.tokens.push(Token::new(TokenType::ConcatenationAssignment, ".=".into(), snapshot));
            Ok(())
          }
          b"..." => {
            self.tokens.push(Token::new(TokenType::Ellipsis, "...".into(), snapshot));
            Ok(())
          }
          b"." => {
            let t = self.control.next_char_until(1, |_, ch, _| !ch.is_ascii_digit());
            if t.len() == 1 {
              self.tokens.push(Token::new(TokenType::Concatenation, ".".into(), snapshot));
              Ok(())
            } else {
              self.tokens.push(Token::new(TokenType::Number, t, snapshot));
              Ok(())
            }
          }
          _ => Err(self.control.error_unrecognized(t.to_string().split_off(1))),
        }
      }
      b'|' => {
        let t = self.until(|ch| ![b'|', b'='].contains(&ch));
        match t.as_slice() {
          b"|=" => {
            self.tokens.push(Token::new(TokenType::BitwiseOrAssignment, "|=".into(), snapshot));
            Ok(())
          }
          b"||" => {
            self.tokens.push(Token::new(TokenType::BooleanOr, "||".into(), snapshot));
            Ok(())
          }
          b"|" => {
            self.tokens.push(Token::new(TokenType::BitwiseOr, "|".into(), snapshot));
            Ok(())
          }
          _ => Err(self.control.error_unrecognized(t.to_string().split_off(1))),
        }
      }
      b'-' => {
        let t = self.until(|ch| ![b'-', b'=', b'>'].contains(&ch));
        match t.as_slice() {
          b"-=" => {
            self.tokens.push(Token::new(TokenType::SubtractionAssignment, "-=".into(), snapshot));
            Ok(())
          }
          b"->" => {
            self.tokens.push(Token::new(TokenType::ObjectAccess, "->".into(), snapshot));
            Ok(())
          }
          b"--" => {
            let is_post = match self.control.peek_char(None) {
              Some(t) => t.is_whitespace() || [b';', b',', b')', b']', b'}', b'?'].contains(&t),
              None => true,
            };
            if is_post {
              self.tokens.push(Token::new(TokenType::PostDecrement, "--".into(), snapshot));
              Ok(())
            } else {
              self.tokens.push(Token::new(TokenType::PreDecrement, "--".into(), snapshot));
              Ok(())
            }
          }
          b"-" => {
            self.tokens.push(Token::new(TokenType::Subtraction, "-".into(), snapshot));
            Ok(())
          }
          _ => Err(self.control.error_unrecognized(t.to_string().split_off(1))),
        }
      }
      b'>' => {
        let t = self.until(|ch| ![b'>', b'='].contains(&ch));
        match t.as_slice() {
          b">>=" => {
            self.tokens.push(
              Token::new(TokenType::BitwiseShiftRightAssignment, ">>=".into(), snapshot)
            );
            Ok(())
          }
          b">=" => {
            self.tokens.push(Token::new(TokenType::IsGreaterOrEqual, ">=".into(), snapshot));
            Ok(())
          }
          b">>" => {
            self.tokens.push(Token::new(TokenType::BitwiseShiftRight, ">>".into(), snapshot));
            Ok(())
          }
          b">" => {
            self.tokens.push(Token::new(TokenType::IsGreater, ">".into(), snapshot));
            Ok(())
          }
          _ => Err(self.control.error_unrecognized(t.to_string().split_off(1))),
        }
      }
      b'<' => {
        let t = self.until(|ch| ![b'<', b'=', b'>'].contains(&ch));
        match t.as_slice() {
          b"<=>" => {
            self.tokens.push(Token::new(TokenType::Spaceship, "<=>".into(), snapshot));
            Ok(())
          }
          b"<>" => {
            self.tokens.push(Token::new(TokenType::IsNotEqual, "<>".into(), snapshot));
            Ok(())
          }
          b"<=" => {
            self.tokens.push(Token::new(TokenType::IsLesserOrEqual, "<=".into(), snapshot));
            Ok(())
          }
          b"<<=" => {
            self.tokens.push(
              Token::new(TokenType::BitwiseShiftLeftAssignment, "<<=".into(), snapshot)
            );
            Ok(())
          }
          b"<<<" => StringToken::lex_doc(self, snapshot),
          b"<<" => {
            self.tokens.push(Token::new(TokenType::BitwiseShiftLeft, "<<".into(), snapshot));
            Ok(())
          }
          b"<" => {
            self.tokens.push(Token::new(TokenType::IsLesser, "<".into(), snapshot));
            Ok(())
          }
          _ => Err(self.control.error_unrecognized(t.to_string().split_off(1))),
        }
      }
      b':' => {
        let t = self.until(|ch| ![b':'].contains(&ch));
        match t.as_slice() {
          b"::" => {
            self.tokens.push(Token::new(TokenType::DoubleColon, "::".into(), snapshot));
            Ok(())
          }
          b":" => {
            self.tokens.push(Token::new(TokenType::Colon, ":".into(), snapshot));
            Ok(())
          }
          _ => Err(self.control.error_unrecognized(t.to_string().split_off(1))),
        }
      }
      b'!' => {
        let t = self.until(|ch| ![b'!', b'='].contains(&ch));
        match t.as_slice() {
          b"!==" => {
            self.tokens.push(Token::new(TokenType::IsNotIdentical, "!==".into(), snapshot));
            Ok(())
          }
          b"!=" => {
            self.tokens.push(Token::new(TokenType::IsNotEqual, "!=".into(), snapshot));
            Ok(())
          }
          c if c.starts_with(b"!") => {
            if t.len() > 1 {
              self.control.position -= t.len() - 1;
            }
            self.tokens.push(Token::new(TokenType::BooleanNegate, "!".into(), snapshot));
            Ok(())
          }
          _ => Err(self.control.error_unrecognized(t.to_string().split_off(1))),
        }
      }
      b'+' => {
        let t = self.until(|ch| ![b'+', b'='].contains(&ch));
        match t.as_slice() {
          b"+=" => {
            self.tokens.push(Token::new(TokenType::AdditionAssignment, "+=".into(), snapshot));
            Ok(())
          }
          b"++" => {
            let is_post = match self.control.peek_char(None) {
              Some(t) => t.is_whitespace() || [b';', b',', b')', b']', b'}', b'?'].contains(&t),
              None => true,
            };
            if is_post {
              self.tokens.push(Token::new(TokenType::PostIncrement, "++".into(), snapshot));
              Ok(())
            } else {
              self.tokens.push(Token::new(TokenType::PreIncrement, "++".into(), snapshot));
              Ok(())
            }
          }
          b"+" => {
            self.tokens.push(Token::new(TokenType::Addition, "+".into(), snapshot));
            Ok(())
          }
          _ => Err(self.control.error_unrecognized(t.to_string().split_off(1))),
        }
      }
      b'(' => {
        self.tokens.push(Token::new(TokenType::LeftParenthesis, "(".into(), snapshot));
        Ok(())
      }
      b')' => {
        self.tokens.push(Token::new(TokenType::RightParenthesis, ")".into(), snapshot));
        Ok(())
      }
      b'{' => {
        self.tokens.push(Token::new(TokenType::LeftCurlyBracket, "{".into(), snapshot));
        Ok(())
      }
      b'}' => {
        self.tokens.push(Token::new(TokenType::RightCurlyBracket, "}".into(), snapshot));
        Ok(())
      }
      b'[' => {
        self.tokens.push(Token::new(TokenType::LeftSquareBracket, "[".into(), snapshot));
        Ok(())
      }
      b']' => {
        self.tokens.push(Token::new(TokenType::RightSquareBracket, "]".into(), snapshot));
        Ok(())
      }
      b'`' => StringToken::lex(self, "`", snapshot),
      b'"' => StringToken::lex(self, "\"", snapshot),
      b'\'' => StringToken::lex_basic(self, "\'", snapshot),
      b'\\' => {
        let t = self.until(|ch| !(ch.is_alphanumeric() || ch == b'_' || ch == b'\\'));
        self.tokens.push(Token::new(TokenType::Name, t, snapshot));
        Ok(())
      }
      b',' => {
        self.tokens.push(Token::new(TokenType::Comma, ",".into(), snapshot));
        Ok(())
      }
      b';' => {
        self.tokens.push(Token::new(TokenType::Semicolon, ";".into(), snapshot));
        Ok(())
      }
      b'~' => {
        self.tokens.push(Token::new(TokenType::BooleanNegate, "~".into(), snapshot));
        Ok(())
      }
      b'@' => {
        self.tokens.push(Token::new(TokenType::AtSign, "@".into(), snapshot));
        Ok(())
      }
      _ => Err(self.control.error_unrecognized(BString::new(vec![current_char]).to_string())),
    }
  }
}

pub trait U8Ext {
  fn is_whitespace(self) -> bool;
  fn is_ascii_digit(self) -> bool;
  fn is_alphabetic(self) -> bool;
  fn is_alphanumeric(self) -> bool;
}

impl U8Ext for u8 {
  fn is_whitespace(self) -> bool {
    self == b' ' || self == b'\t' || self == b'\n' || self == b'\r' || self == b'\x0C'
  }

  fn is_ascii_digit(self) -> bool {
    self >= b'0' && self <= b'9'
  }

  fn is_alphabetic(self) -> bool {
    (self >= b'a' && self <= b'z') || (self >= b'A' && self <= b'Z')
  }

  fn is_alphanumeric(self) -> bool {
    self.is_ascii_digit() || self.is_alphabetic()
  }
}
