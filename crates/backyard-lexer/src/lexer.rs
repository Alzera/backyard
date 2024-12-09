use bumpalo::Bump;
use compact_str::CompactString;

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
  chars: Vec<char>,
  position: usize,
  pub(crate) line: usize,
  pub(crate) column: usize,
  pub(crate) last_snapshot: ControlSnapshot,
}

impl Control {
  pub(crate) fn new(input: &str) -> Self {
    Control {
      chars: input.chars().collect(),
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

  pub(crate) fn peek_char(&mut self, pos: Option<usize>) -> Option<char> {
    let p = if let Some(pos) = pos { pos } else { self.position };
    self.chars.get(p).copied()
  }

  pub(crate) fn peek_char_n(&mut self, pos: Option<usize>, n: usize) -> Option<String> {
    let p = if let Some(pos) = pos { pos } else { self.position };
    self.chars.get(p..p + n).map(|i| i.iter().collect())
  }

  pub(crate) fn next_char(&mut self) -> Option<char> {
    if let Some(c) = self.chars.get(self.position) {
      self.last_snapshot = self.get_snapshot();
      self.position += 1;
      if *c == '\n' {
        self.line += 1;
        self.column = 0;
      } else {
        self.column += 1;
      }
      Some(*c)
    } else {
      None
    }
  }

  pub(crate) fn next_char_until<F>(&mut self, mut until: F) -> CompactString
    where F: FnMut(&mut Control, &char, &mut usize) -> bool
  {
    let start_position = self.position;
    let mut end_position = self.position;
    while let Some(ch) = self.chars.get(end_position) {
      let ch = *ch;
      if until(self, &ch, &mut end_position) {
        break;
      }
      end_position += 1;
    }

    let result: CompactString = self.chars[start_position..end_position].iter().collect();
    while self.position < end_position {
      self.next_char();
    }
    result
  }

  pub(crate) fn error_unrecognized(&self, t: &str) -> LexError {
    LexError::Unrecognized { token: t.to_string(), line: self.line, column: self.column }
  }
}

#[derive(Debug, PartialEq)]
pub enum SeriesCheckerMode {
  String,
  Inline,
  Heredoc,
}

#[derive(Debug)]
pub struct SeriesChecker<'a> {
  list: Vec<char>,
  againsts: &'a [&'a str],
  mode: SeriesCheckerMode,
}

impl<'a> SeriesChecker<'a> {
  pub fn new(againsts: &'a [&'a str], mode: SeriesCheckerMode) -> Self {
    Self { list: vec![], againsts, mode }
  }

  pub fn push(&mut self, ch: char) {
    if self.mode == SeriesCheckerMode::Heredoc {
      if ch == '\n' {
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

  pub fn check(&mut self) -> Option<CompactString> {
    let text = self.list.iter().collect::<String>();
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
      .take_while(|x| **x == '\\')
      .count() % 2 == 1
  }

  pub fn is_last_escaped(&self) -> bool {
    self.is_escaped(self.list.len() - 1)
  }
}

#[derive(Debug)]
pub struct Lexer<'a> {
  pub(crate) tokens: bumpalo::collections::Vec<'a, Token>,
  pub(crate) control: Control,
}

impl<'a> Lexer<'a> {
  pub fn new(arena: &'a Bump, input: &str) -> Self {
    Lexer {
      tokens: bumpalo::collections::Vec::new_in(arena),
      control: Control::new(input),
    }
  }

  pub fn next_tokens_until_right_bracket(&mut self) -> LexResult {
    let mut level = 1;
    while let Some(next_token) = self.control.peek_char(None) {
      match next_token {
        '{' => {
          level += 1;
          self.next_tokens(true)?;
        }
        '}' => {
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

  fn until<F>(&mut self, ch: char, mut callback: F) -> CompactString where F: FnMut(&char) -> bool {
    let mut next_chars = self.control.next_char_until(|_, ch, _| callback(ch));
    next_chars.insert(0, ch);
    next_chars
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
      self.control.next_char_until(|_, ch, _| !ch.is_whitespace());
    }

    let snapshot = &self.control.get_snapshot();
    let current_char = if let Some(current_char) = self.control.next_char() {
      current_char
    } else {
      return Err(LexError::Eof);
    };

    match current_char {
      '$' => VariableToken::lex(self, snapshot),
      c if c.is_ascii_digit() => NumberToken::lex(self, current_char, snapshot),
      c if c.is_alphabetic() || c == '_' => {
        let t = self.until(current_char, |ch| !(ch.is_alphanumeric() || *ch == '_' || *ch == '\\'));
        if
          [
            "__CLASS__",
            "__DIR__",
            "__FILE__",
            "__FUNCTION__",
            "__LINE__",
            "__METHOD__",
            "__NAMESPACE__",
            "__TRAIT__",
            "__PROPERTY__",
          ].contains(&t.as_str())
        {
          self.tokens.push(Token::new(TokenType::Magic, t, snapshot));
          Ok(())
        } else if
          [
            "__construct",
            "__destruct",
            "__call",
            "__callStatic",
            "__get",
            "__set",
            "__isset",
            "__unset",
            "__sleep",
            "__wakeup",
            "__serialize",
            "__unserialize",
            "__toString",
            "__invoke",
            "__set_state",
            "__clone",
            "__debugInfo",
          ].contains(&t.as_str())
        {
          self.tokens.push(Token::new(TokenType::MagicMethod, t, snapshot));
          Ok(())
        } else if
          [
            // "array",
            "bool",
            "boolean",
            "real",
            "double",
            "float",
            "int",
            "integer",
            "object",
            "String",
            "string",
            "mixed",
            "void",
            // "null",
          ].contains(&t.as_str())
        {
          self.tokens.push(Token::new(TokenType::Type, t, snapshot));
          Ok(())
        } else if KeywordToken::is_keyword(&t) {
          KeywordToken::lex(self, &t, snapshot)
        } else {
          self.tokens.push(Token::new(TokenType::Identifier, t, snapshot));
          Ok(())
        }
      }
      '=' => {
        let t = self.until(current_char, |ch| !['=', '>', '&'].contains(ch));
        match t.as_str() {
          "===" => {
            self.tokens.push(Token::new(TokenType::IsIdentical, "===".into(), snapshot));
            Ok(())
          }
          "==" => {
            self.tokens.push(Token::new(TokenType::IsEqual, "==".into(), snapshot));
            Ok(())
          }
          "=" => {
            self.tokens.push(Token::new(TokenType::Assignment, "=".into(), snapshot));
            Ok(())
          }
          "=>" => {
            self.tokens.push(Token::new(TokenType::Arrow, "=>".into(), snapshot));
            Ok(())
          }
          "=&" => {
            self.tokens.push(Token::new(TokenType::ReferenceAssignment, "=&".into(), snapshot));
            Ok(())
          }
          _ => Err(self.control.error_unrecognized(&t)),
        }
      }
      '&' => {
        let t = self.until(current_char, |ch| !['&', '='].contains(ch));
        match t.as_str() {
          "&=" => {
            self.tokens.push(Token::new(TokenType::BitwiseAndAssignment, "&=".into(), snapshot));
            Ok(())
          }
          "&&" => {
            self.tokens.push(Token::new(TokenType::BooleanAnd, "&&".into(), snapshot));
            Ok(())
          }
          "&" => {
            self.tokens.push(Token::new(TokenType::BitwiseAnd, "&".into(), snapshot));
            Ok(())
          }
          _ => Err(self.control.error_unrecognized(&t)),
        }
      }
      '#' => {
        let t = self.until(current_char, |ch| !['#', '['].contains(ch));
        match t.as_str() {
          "#[" => {
            self.tokens.push(Token::new(TokenType::Attribute, "#[".into(), snapshot));
            Ok(())
          }
          _ => CommentToken::lex_line(self, &t[1..], snapshot),
        }
      }
      '?' => {
        let t = self.until(current_char, |ch| !['?', '>', '=', '-', ':'].contains(ch));
        match t.as_str() {
          "?:" => {
            self.tokens.push(Token::new(TokenType::Elvis, "?:".into(), snapshot));
            Ok(())
          }
          "?->" => {
            self.tokens.push(Token::new(TokenType::NullsafeObjectAccess, "?->".into(), snapshot));
            Ok(())
          }
          "??=" => {
            self.tokens.push(Token::new(TokenType::CoalesceAssignment, "??=".into(), snapshot));
            Ok(())
          }
          "??" => {
            self.tokens.push(Token::new(TokenType::Coalesce, "??".into(), snapshot));
            Ok(())
          }
          "?" => {
            self.tokens.push(Token::new(TokenType::QuestionMark, "?".into(), snapshot));
            Ok(())
          }
          c if c.starts_with("?>") => {
            if t.len() > 2 {
              self.control.position -= t.len() - 2;
            }
            InlineToken::lex(self, snapshot)
          }
          _ => Err(self.control.error_unrecognized(&t)),
        }
      }
      '%' => {
        let t = self.until(current_char, |ch| !['%', '='].contains(ch));
        match t.as_str() {
          "%=" => {
            self.tokens.push(Token::new(TokenType::ModulusAssignment, "%=".into(), snapshot));
            Ok(())
          }
          "%" => {
            self.tokens.push(Token::new(TokenType::Modulus, "%".into(), snapshot));
            Ok(())
          }
          _ => Err(self.control.error_unrecognized(&t)),
        }
      }
      '^' => {
        let t = self.until(current_char, |ch| !['^', '='].contains(ch));
        match t.as_str() {
          "^=" => {
            self.tokens.push(Token::new(TokenType::BitwiseXorAssignment, "^=".into(), snapshot));
            Ok(())
          }
          "^" => {
            self.tokens.push(Token::new(TokenType::BitwiseXor, "^".into(), snapshot));
            Ok(())
          }
          _ => Err(self.control.error_unrecognized(&t)),
        }
      }
      '*' => {
        let t = self.until(current_char, |ch| !['*', '='].contains(ch));
        match t.as_str() {
          "**=" => {
            self.tokens.push(
              Token::new(TokenType::ExponentiationAssignment, "**=".into(), snapshot)
            );
            Ok(())
          }
          "*=" => {
            self.tokens.push(
              Token::new(TokenType::MultiplicationAssignment, "*=".into(), snapshot)
            );
            Ok(())
          }
          "**" => {
            self.tokens.push(Token::new(TokenType::Exponentiation, "**".into(), snapshot));
            Ok(())
          }
          "*" => {
            self.tokens.push(Token::new(TokenType::Multiplication, "*".into(), snapshot));
            Ok(())
          }
          _ => Err(self.control.error_unrecognized(&t)),
        }
      }
      '/' => {
        let t = self.until(current_char, |ch| !['/', '*', '='].contains(ch));
        match t.as_str() {
          "/=" => {
            self.tokens.push(Token::new(TokenType::DivisionAssignment, "/=".into(), snapshot));
            Ok(())
          }
          c if c.starts_with("/**") => CommentToken::lex_doc(self, &t[3..], snapshot),
          c if c.starts_with("/*") => CommentToken::lex_block(self, &t[2..], snapshot),
          c if c.starts_with("//") => CommentToken::lex_line(self, &t[2..], snapshot),
          "/" => {
            self.tokens.push(Token::new(TokenType::Division, "/".into(), snapshot));
            Ok(())
          }
          _ => Err(self.control.error_unrecognized(&t)),
        }
      }
      '.' => {
        let t = self.until(current_char, |ch| !['.', '='].contains(ch));
        match t.as_str() {
          ".=" => {
            self.tokens.push(Token::new(TokenType::ConcatenationAssignment, ".=".into(), snapshot));
            Ok(())
          }
          "..." => {
            self.tokens.push(Token::new(TokenType::Ellipsis, "...".into(), snapshot));
            Ok(())
          }
          "." => {
            let mut t = self.control.next_char_until(|_, ch, _| !ch.is_ascii_digit());
            if t.is_empty() {
              self.tokens.push(Token::new(TokenType::Concatenation, ".".into(), snapshot));
              Ok(())
            } else {
              t.insert(0, '.');
              self.tokens.push(Token::new(TokenType::Number, t, snapshot));
              Ok(())
            }
          }
          _ => Err(self.control.error_unrecognized(&t)),
        }
      }
      '|' => {
        let t = self.until(current_char, |ch| !['|', '='].contains(ch));
        match t.as_str() {
          "|=" => {
            self.tokens.push(Token::new(TokenType::BitwiseOrAssignment, "|=".into(), snapshot));
            Ok(())
          }
          "||" => {
            self.tokens.push(Token::new(TokenType::BooleanOr, "||".into(), snapshot));
            Ok(())
          }
          "|" => {
            self.tokens.push(Token::new(TokenType::BitwiseOr, "|".into(), snapshot));
            Ok(())
          }
          _ => Err(self.control.error_unrecognized(&t)),
        }
      }
      '-' => {
        let t = self.until(current_char, |ch| !['-', '=', '>'].contains(ch));
        match t.as_str() {
          "-=" => {
            self.tokens.push(Token::new(TokenType::SubtractionAssignment, "-=".into(), snapshot));
            Ok(())
          }
          "->" => {
            self.tokens.push(Token::new(TokenType::ObjectAccess, "->".into(), snapshot));
            Ok(())
          }
          "--" => {
            let is_post = match self.control.peek_char(None) {
              Some(t) => t.is_whitespace() || [';', ',', ')', ']', '}', '?'].contains(&t),
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
          "-" => {
            self.tokens.push(Token::new(TokenType::Subtraction, "-".into(), snapshot));
            Ok(())
          }
          _ => Err(self.control.error_unrecognized(&t)),
        }
      }
      '>' => {
        let t = self.until(current_char, |ch| !['>', '='].contains(ch));
        match t.as_str() {
          ">>=" => {
            self.tokens.push(
              Token::new(TokenType::BitwiseShiftRightAssignment, ">>=".into(), snapshot)
            );
            Ok(())
          }
          ">=" => {
            self.tokens.push(Token::new(TokenType::IsGreaterOrEqual, ">=".into(), snapshot));
            Ok(())
          }
          ">>" => {
            self.tokens.push(Token::new(TokenType::BitwiseShiftRight, ">>".into(), snapshot));
            Ok(())
          }
          ">" => {
            self.tokens.push(Token::new(TokenType::IsGreater, ">".into(), snapshot));
            Ok(())
          }
          _ => Err(self.control.error_unrecognized(&t)),
        }
      }
      '<' => {
        let t = self.until(current_char, |ch| !['<', '=', '>'].contains(ch));
        match t.as_str() {
          "<=>" => {
            self.tokens.push(Token::new(TokenType::Spaceship, "<=>".into(), snapshot));
            Ok(())
          }
          "<>" => {
            self.tokens.push(Token::new(TokenType::IsNotEqual, "<>".into(), snapshot));
            Ok(())
          }
          "<=" => {
            self.tokens.push(Token::new(TokenType::IsLesserOrEqual, "<=".into(), snapshot));
            Ok(())
          }
          "<<=" => {
            self.tokens.push(
              Token::new(TokenType::BitwiseShiftLeftAssignment, "<<=".into(), snapshot)
            );
            Ok(())
          }
          "<<<" => StringToken::lex_doc(self, snapshot),
          "<<" => {
            self.tokens.push(Token::new(TokenType::BitwiseShiftLeft, "<<".into(), snapshot));
            Ok(())
          }
          "<" => {
            self.tokens.push(Token::new(TokenType::IsLesser, "<".into(), snapshot));
            Ok(())
          }
          _ => Err(self.control.error_unrecognized(&t)),
        }
      }
      ':' => {
        let t = self.until(current_char, |ch| ![':'].contains(ch));
        match t.as_str() {
          "::" => {
            self.tokens.push(Token::new(TokenType::DoubleColon, "::".into(), snapshot));
            Ok(())
          }
          ":" => {
            self.tokens.push(Token::new(TokenType::Colon, ":".into(), snapshot));
            Ok(())
          }
          _ => Err(self.control.error_unrecognized(&t)),
        }
      }
      '!' => {
        let t = self.until(current_char, |ch| !['!', '='].contains(ch));
        match t.as_str() {
          "!==" => {
            self.tokens.push(Token::new(TokenType::IsNotIdentical, "!==".into(), snapshot));
            Ok(())
          }
          "!=" => {
            self.tokens.push(Token::new(TokenType::IsNotEqual, "!=".into(), snapshot));
            Ok(())
          }
          c if c.starts_with('!') => {
            if t.len() > 1 {
              self.control.position -= t.len() - 1;
            }
            self.tokens.push(Token::new(TokenType::BooleanNegate, "!".into(), snapshot));
            Ok(())
          }
          _ => Err(self.control.error_unrecognized(&t)),
        }
      }
      '+' => {
        let t = self.until(current_char, |ch| !['+', '='].contains(ch));
        match t.as_str() {
          "+=" => {
            self.tokens.push(Token::new(TokenType::AdditionAssignment, "+=".into(), snapshot));
            Ok(())
          }
          "++" => {
            let is_post = match self.control.peek_char(None) {
              Some(t) => t.is_whitespace() || [';', ',', ')', ']', '}', '?'].contains(&t),
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
          "+" => {
            self.tokens.push(Token::new(TokenType::Addition, "+".into(), snapshot));
            Ok(())
          }
          _ => Err(self.control.error_unrecognized(&t)),
        }
      }
      '(' => {
        self.tokens.push(Token::new(TokenType::LeftParenthesis, "(".into(), snapshot));
        Ok(())
      }
      ')' => {
        self.tokens.push(Token::new(TokenType::RightParenthesis, ")".into(), snapshot));
        Ok(())
      }
      '{' => {
        self.tokens.push(Token::new(TokenType::LeftCurlyBracket, "{".into(), snapshot));
        Ok(())
      }
      '}' => {
        self.tokens.push(Token::new(TokenType::RightCurlyBracket, "}".into(), snapshot));
        Ok(())
      }
      '[' => {
        self.tokens.push(Token::new(TokenType::LeftSquareBracket, "[".into(), snapshot));
        Ok(())
      }
      ']' => {
        self.tokens.push(Token::new(TokenType::RightSquareBracket, "]".into(), snapshot));
        Ok(())
      }
      '`' => StringToken::lex(self, "`", snapshot),
      '"' => StringToken::lex(self, "\"", snapshot),
      '\'' => StringToken::lex_basic(self, "\'", snapshot),
      '\\' => {
        let t = self.until(current_char, |ch| !(ch.is_alphanumeric() || *ch == '_' || *ch == '\\'));
        self.tokens.push(Token::new(TokenType::Name, t, snapshot));
        Ok(())
      }
      ',' => {
        self.tokens.push(Token::new(TokenType::Comma, ",".into(), snapshot));
        Ok(())
      }
      ';' => {
        self.tokens.push(Token::new(TokenType::Semicolon, ";".into(), snapshot));
        Ok(())
      }
      '~' => {
        self.tokens.push(Token::new(TokenType::BooleanNegate, "~".into(), snapshot));
        Ok(())
      }
      '@' => {
        self.tokens.push(Token::new(TokenType::AtSign, "@".into(), snapshot));
        Ok(())
      }
      _ => Err(self.control.error_unrecognized(&current_char.to_string())),
    }
  }
}
