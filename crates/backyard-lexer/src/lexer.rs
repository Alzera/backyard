use utils::guard;

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

  pub(crate) fn next_char_until<F>(&mut self, mut until: F) -> String
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

    let result = self.chars[start_position..end_position].to_vec();
    while self.position < end_position {
      self.next_char();
    }
    result.iter().collect()
  }

  pub(crate) fn error_unrecognized(&self, t: &str) -> LexError {
    LexError::Unrecognized { token: t.to_string(), line: self.line, column: self.column }
  }
}

#[derive(Debug)]
pub struct SeriesChecker<'a> {
  list: Vec<char>,
  againsts: Vec<&'a str>,
  escape_check: bool,
}

impl<'a> SeriesChecker<'a> {
  pub fn new(againsts: &[&'a str]) -> Self {
    Self { list: vec![], againsts: againsts.to_vec(), escape_check: true }
  }

  pub fn safe(againsts: &[&'a str]) -> Self {
    Self { list: vec![], againsts: againsts.to_vec(), escape_check: false }
  }

  pub fn push(&mut self, ch: char) {
    if ch.is_whitespace() {
      self.list.clear();
    } else {
      self.list.push(ch);
    }
  }

  pub fn check(&mut self) -> Option<&str> {
    let text = self.list.clone().into_iter().collect::<String>();
    let valid = self.againsts
      .clone()
      .into_iter()
      .find(|i| text.ends_with(i));
    if valid.is_some() {
      let valid = valid.unwrap();
      if !self.is_escaped(self.list.len() - valid.len()) {
        return Some(valid);
      }
    }
    None
  }

  pub fn is_escaped(&self, index: usize) -> bool {
    if !self.escape_check {
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
pub struct Lexer {
  pub(crate) control: Control,
}

impl Lexer {
  pub fn new(input: &str) -> Self {
    Lexer {
      control: Control::new(input),
    }
  }

  pub fn start(&mut self) -> LexResult {
    let mut tokens = Vec::new();
    loop {
      match self.next_tokens(true) {
        Ok(token) => tokens.extend(token),
        Err(err) => {
          if err == LexError::Eof {
            break;
          }
          return Err(err);
        }
      }
    }
    Ok(tokens)
  }

  pub fn next_tokens(&mut self, skip_whitespace: bool) -> LexResult {
    if skip_whitespace {
      self.control.next_char_until(|_, ch, _| !ch.is_whitespace());
    }

    let snapshot = &self.control.get_snapshot();
    let current_char = guard!(self.control.next_char(), {
      return Err(LexError::Eof);
    });

    match current_char {
      '$' => VariableToken::lex(self, snapshot),
      c if c.is_whitespace() =>
        Ok(vec![Token::new(TokenType::Whitespace, current_char.to_string(), snapshot)]),
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
          ].contains(&t.as_str())
        {
          return Ok(vec![Token::new(TokenType::Magic, &t, snapshot)]);
        }
        if
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
            "mixed",
            // "null",
          ].contains(&t.as_str())
        {
          return Ok(vec![Token::new(TokenType::Type, &t, snapshot)]);
        }
        if KeywordToken::is_keyword(&t) {
          return KeywordToken::lex(self, &t, snapshot);
        }
        Ok(vec![Token::new(TokenType::Identifier, &t, snapshot)])
      }
      '=' => {
        let t = self.until(current_char, |ch| !['=', '>', '&'].contains(ch));
        match t.as_str() {
          "===" => Ok(vec![Token::new(TokenType::IsIdentical, "===", snapshot)]),
          "==" => Ok(vec![Token::new(TokenType::IsEqual, "==", snapshot)]),
          "=" => Ok(vec![Token::new(TokenType::Assignment, "=", snapshot)]),
          "=>" => Ok(vec![Token::new(TokenType::Arrow, "=>", snapshot)]),
          "=&" => Ok(vec![Token::new(TokenType::ReferenceAssignment, "=&", snapshot)]),
          _ => Err(self.control.error_unrecognized(&t)),
        }
      }
      '&' => {
        let t = self.until(current_char, |ch| !['&', '='].contains(ch));
        match t.as_str() {
          "&=" => Ok(vec![Token::new(TokenType::BitwiseAndAssignment, "&=", snapshot)]),
          "&&" => Ok(vec![Token::new(TokenType::BooleanAnd, "&&", snapshot)]),
          "&" => Ok(vec![Token::new(TokenType::BitwiseAnd, "&", snapshot)]),
          _ => Err(self.control.error_unrecognized(&t)),
        }
      }
      '#' => {
        let t = self.until(current_char, |ch| !['#', '['].contains(ch));
        match t.as_str() {
          "#[" => Ok(vec![Token::new(TokenType::Attribute, "#[", snapshot)]),
          _ => CommentToken::lex_line(self, &t[1..], snapshot),
        }
      }
      '?' => {
        let t = self.until(current_char, |ch| !['?', '>', '=', '-', ':'].contains(ch));
        match t.as_str() {
          "?:" => Ok(vec![Token::new(TokenType::Elvis, "?:", snapshot)]),
          "?->" => Ok(vec![Token::new(TokenType::NullsafeObjectAccess, "?->", snapshot)]),
          "??=" => Ok(vec![Token::new(TokenType::CoalesceAssignment, "??=", snapshot)]),
          "??" => Ok(vec![Token::new(TokenType::Coalesce, "??", snapshot)]),
          "?" => Ok(vec![Token::new(TokenType::QuestionMark, "?", snapshot)]),
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
          "%=" => Ok(vec![Token::new(TokenType::ModulusAssignment, "%=", snapshot)]),
          "%" => Ok(vec![Token::new(TokenType::Modulus, "%", snapshot)]),
          _ => Err(self.control.error_unrecognized(&t)),
        }
      }
      '^' => {
        let t = self.until(current_char, |ch| !['^', '='].contains(ch));
        match t.as_str() {
          "^=" => Ok(vec![Token::new(TokenType::BitwiseXorAssignment, "^=", snapshot)]),
          "^" => Ok(vec![Token::new(TokenType::BitwiseXor, "^", snapshot)]),
          _ => Err(self.control.error_unrecognized(&t)),
        }
      }
      '*' => {
        let t = self.until(current_char, |ch| !['*', '='].contains(ch));
        match t.as_str() {
          "**=" => Ok(vec![Token::new(TokenType::ExponentiationAssignment, "**=", snapshot)]),
          "*=" => Ok(vec![Token::new(TokenType::MultiplicationAssignment, "*=", snapshot)]),
          "**" => Ok(vec![Token::new(TokenType::Exponentiation, "**", snapshot)]),
          "*" => Ok(vec![Token::new(TokenType::Multiplication, "*", snapshot)]),
          _ => Err(self.control.error_unrecognized(&t)),
        }
      }
      '/' => {
        let t = self.until(current_char, |ch| !['/', '*', '='].contains(ch));
        match t.as_str() {
          "/=" => Ok(vec![Token::new(TokenType::DivisionAssignment, "/=", snapshot)]),
          c if c.starts_with("/**") => CommentToken::lex_doc(self, &t[3..], snapshot),
          c if c.starts_with("/*") => CommentToken::lex_block(self, &t[2..], snapshot),
          c if c.starts_with("//") => CommentToken::lex_line(self, &t[2..], snapshot),
          "/" => Ok(vec![Token::new(TokenType::Division, "/", snapshot)]),
          _ => Err(self.control.error_unrecognized(&t)),
        }
      }
      '.' => {
        let t = self.until(current_char, |ch| !['.', '='].contains(ch));
        match t.as_str() {
          ".=" => Ok(vec![Token::new(TokenType::ConcatenationAssignment, ".=", snapshot)]),
          "..." => Ok(vec![Token::new(TokenType::Ellipsis, "...", snapshot)]),
          "." => {
            let mut t = self.control.next_char_until(|_, ch, _| !ch.is_ascii_digit());
            if t.is_empty() {
              Ok(vec![Token::new(TokenType::Concatenation, ".", snapshot)])
            } else {
              t.insert(0, '.');
              Ok(vec![Token::new(TokenType::Number, &t, snapshot)])
            }
          }
          _ => Err(self.control.error_unrecognized(&t)),
        }
      }
      '|' => {
        let t = self.until(current_char, |ch| !['|', '='].contains(ch));
        match t.as_str() {
          "|=" => Ok(vec![Token::new(TokenType::BitwiseOrAssignment, "|=", snapshot)]),
          "||" => Ok(vec![Token::new(TokenType::BooleanOr, "||", snapshot)]),
          "|" => Ok(vec![Token::new(TokenType::BitwiseOr, "|", snapshot)]),
          _ => Err(self.control.error_unrecognized(&t)),
        }
      }
      '-' => {
        let t = self.until(current_char, |ch| !['-', '=', '>'].contains(ch));
        match t.as_str() {
          "-=" => Ok(vec![Token::new(TokenType::SubtractionAssignment, "-=", snapshot)]),
          "->" => Ok(vec![Token::new(TokenType::ObjectAccess, "->", snapshot)]),
          "--" => {
            let is_post = match self.control.peek_char(None) {
              Some(t) => t.is_whitespace() || [';', ',', ')', ']', '}', '?'].contains(&t),
              None => true,
            };
            if is_post {
              return Ok(vec![Token::new(TokenType::PostDecrement, "--", snapshot)]);
            }
            Ok(vec![Token::new(TokenType::PreDecrement, "--", snapshot)])
          }
          "-" => Ok(vec![Token::new(TokenType::Subtraction, "-", snapshot)]),
          _ => Err(self.control.error_unrecognized(&t)),
        }
      }
      '>' => {
        let t = self.until(current_char, |ch| !['>', '='].contains(ch));
        match t.as_str() {
          ">>=" => Ok(vec![Token::new(TokenType::BitwiseShiftRightAssignment, ">>=", snapshot)]),
          ">=" => Ok(vec![Token::new(TokenType::IsGreaterOrEqual, ">=", snapshot)]),
          ">>" => Ok(vec![Token::new(TokenType::BitwiseShiftRight, ">>", snapshot)]),
          ">" => Ok(vec![Token::new(TokenType::IsGreater, ">", snapshot)]),
          _ => Err(self.control.error_unrecognized(&t)),
        }
      }
      '<' => {
        let t = self.until(current_char, |ch| !['<', '=', '>'].contains(ch));
        match t.as_str() {
          "<=>" => Ok(vec![Token::new(TokenType::Spaceship, "<=>", snapshot)]),
          "<>" => Ok(vec![Token::new(TokenType::IsNotEqual, "<>", snapshot)]),
          "<=" => Ok(vec![Token::new(TokenType::IsLesserOrEqual, "<=", snapshot)]),
          "<<=" => Ok(vec![Token::new(TokenType::BitwiseShiftLeftAssignment, "<<=", snapshot)]),
          "<<<" => StringToken::lex_doc(self, snapshot),
          "<<" => Ok(vec![Token::new(TokenType::BitwiseShiftLeft, "<<", snapshot)]),
          "<" => Ok(vec![Token::new(TokenType::IsLesser, "<", snapshot)]),
          _ => Err(self.control.error_unrecognized(&t)),
        }
      }
      ':' => {
        let t = self.until(current_char, |ch| ![':'].contains(ch));
        match t.as_str() {
          "::" => Ok(vec![Token::new(TokenType::DoubleColon, "::", snapshot)]),
          ":" => Ok(vec![Token::new(TokenType::Colon, ":", snapshot)]),
          _ => Err(self.control.error_unrecognized(&t)),
        }
      }
      '!' => {
        let t = self.until(current_char, |ch| !['!', '='].contains(ch));
        match t.as_str() {
          "!==" => Ok(vec![Token::new(TokenType::IsNotIdentical, "!==", snapshot)]),
          "!=" => Ok(vec![Token::new(TokenType::IsNotEqual, "!=", snapshot)]),
          c if c.starts_with("!") => {
            if t.len() > 1 {
              self.control.position -= t.len() - 1;
            }
            Ok(vec![Token::new(TokenType::BooleanNegate, "!", snapshot)])
          }
          _ => Err(self.control.error_unrecognized(&t)),
        }
      }
      '+' => {
        let t = self.until(current_char, |ch| !['+', '='].contains(ch));
        match t.as_str() {
          "+=" => Ok(vec![Token::new(TokenType::AdditionAssignment, "+=", snapshot)]),
          "++" => {
            let is_post = match self.control.peek_char(None) {
              Some(t) => t.is_whitespace() || [';', ',', ')', ']', '}', '?'].contains(&t),
              None => true,
            };
            if is_post {
              return Ok(vec![Token::new(TokenType::PostIncrement, "++", snapshot)]);
            }
            Ok(vec![Token::new(TokenType::PreIncrement, "++", snapshot)])
          }
          "+" => Ok(vec![Token::new(TokenType::Addition, "+", snapshot)]),
          _ => Err(self.control.error_unrecognized(&t)),
        }
      }
      '(' => Ok(vec![Token::new(TokenType::LeftParenthesis, "(", snapshot)]),
      ')' => Ok(vec![Token::new(TokenType::RightParenthesis, ")", snapshot)]),
      '{' => Ok(vec![Token::new(TokenType::LeftCurlyBracket, "{", snapshot)]),
      '}' => Ok(vec![Token::new(TokenType::RightCurlyBracket, "}", snapshot)]),
      '[' => Ok(vec![Token::new(TokenType::LeftSquareBracket, "[", snapshot)]),
      ']' => Ok(vec![Token::new(TokenType::RightSquareBracket, "]", snapshot)]),
      '`' => StringToken::lex(self, '`', snapshot),
      '"' => StringToken::lex(self, '"', snapshot),
      '\'' => StringToken::lex(self, '\'', snapshot),
      '\\' => {
        let t = self.until(current_char, |ch| !(ch.is_alphanumeric() || *ch == '_' || *ch == '\\'));
        Ok(vec![Token::new(TokenType::Name, &t, snapshot)])
      }
      ',' => Ok(vec![Token::new(TokenType::Comma, ",", snapshot)]),
      ';' => Ok(vec![Token::new(TokenType::Semicolon, ";", snapshot)]),
      '~' => Ok(vec![Token::new(TokenType::BooleanNegate, "~", snapshot)]),
      '@' => {
        if let Some(next) = self.control.peek_char(None) {
          if !next.is_whitespace() {
            return Ok(vec![Token::new(TokenType::AtSign, "@", snapshot)]);
          }
        }
        Err(self.control.error_unrecognized("@"))
      }
      // '\n' => Ok(vec![Token::new(TokenType::LineBreak, "\n", snapshot)]),
      _ => Err(self.control.error_unrecognized(&current_char.to_string())),
    }
  }

  pub fn next_tokens_until_right_bracket(&mut self) -> Vec<Token> {
    self.next_tokens_level(
      1,
      [TokenType::LeftCurlyBracket].to_vec(),
      [TokenType::RightCurlyBracket].to_vec()
    )
  }

  pub fn next_tokens_level(
    &mut self,
    start_level: usize,
    level_ups: Vec<TokenType>,
    level_downs: Vec<TokenType>
  ) -> Vec<Token> {
    let mut result: Vec<Token> = Vec::new();
    let mut level = start_level;
    loop {
      if let Ok(tokens) = self.next_tokens(true) {
        if let Some(token) = tokens.first() {
          if level_ups.contains(&token.token_type) {
            level += 1;
          } else if level_downs.contains(&token.token_type) {
            level -= 1;
            if level == 0 {
              break result;
            }
          }
        }
        result.extend(tokens);
      } else {
        break result;
      }
    }
  }

  fn until<F>(&mut self, ch: char, mut callback: F) -> String where F: FnMut(&char) -> bool {
    let mut next_chars = self.control.next_char_until(|_, ch, _| callback(ch));
    next_chars.insert(0, ch);
    next_chars
  }
}
