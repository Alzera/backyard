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
pub struct Control {
  chars: Vec<char>,
  position: usize,
  line: usize,
  column: usize,
}

impl Control {
  pub fn new(input: &str) -> Self {
    Control {
      chars: input.chars().collect(),
      position: 0,
      line: 1,
      column: 1,
    }
  }

  pub fn get_len(&self) -> usize {
    self.chars.len()
  }

  pub fn get_position(&self) -> usize {
    self.position
  }

  pub fn peek_char(&mut self, pos: Option<usize>) -> Option<char> {
    let p = if pos.is_some() { pos.unwrap() } else { self.position };
    if let Some(c) = self.chars.get(p) {
      Some(*c)
    } else {
      None
    }
  }

  pub fn prev_char(&mut self) -> Option<char> {
    if let Some(c) = self.chars.get(self.position - 1) {
      self.position -= 1;
      if *c == '\n' {
        self.line -= 1;

        let mut column_length = 0;
        let mut pos = self.position;
        while pos > 0 {
          pos -= 1;
          if let Some(ch) = self.chars.get(pos) {
            if *ch == '\n' {
              break;
            }
            column_length += 1;
          } else {
            break;
          }
        }
        self.column = column_length;
      } else {
        self.column -= 1;
      }
      Some(*c)
    } else {
      None
    }
  }

  pub fn next_char(&mut self) -> Option<char> {
    if let Some(c) = self.chars.get(self.position) {
      self.position += 1;
      if *c == '\n' {
        self.line += 1;
        self.column = 1;
      } else {
        self.column += 1;
      }
      Some(*c)
    } else {
      None
    }
  }

  pub fn next_char_until<F>(&mut self, mut until: F) -> String
    where F: FnMut(&mut Control, &char, &mut usize) -> bool
  {
    let start_position = self.position;
    let mut end_position = self.position;
    while let Some(ch) = self.chars.get(end_position) {
      let ch = ch.clone();
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

  pub fn error_unrecognized(&self, t: &str) -> LexError {
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
  pub control: Control,
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

    let current_char = guard!(self.control.next_char(), {
      return Err(LexError::Eof);
    });

    match current_char {
      '$' => VariableToken::lex(self),
      c if c.is_whitespace() =>
        Ok(vec![Token::new(TokenType::Whitespace, current_char.to_string())]),
      c if c.is_digit(10) => NumberToken::lex(self, current_char),
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
          return Ok(vec![Token::new(TokenType::Magic, t)]);
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
          return Ok(vec![Token::new(TokenType::Type, t)]);
        }
        if KeywordToken::is_keyword(&t) {
          return KeywordToken::lex(self, &t);
        }
        return Ok(vec![Token::new(TokenType::Identifier, t)]);
      }
      '=' => {
        let t = self.until(current_char, |ch| !['=', '>', '&'].contains(ch));
        match t.as_str() {
          "===" => Ok(vec![Token::new(TokenType::IsIdentical, "===")]),
          "==" => Ok(vec![Token::new(TokenType::IsEqual, "==")]),
          "=" => Ok(vec![Token::new(TokenType::Assignment, "=")]),
          "=>" => Ok(vec![Token::new(TokenType::Arrow, "=>")]),
          "=&" => Ok(vec![Token::new(TokenType::ReferenceAssignment, "=>")]),
          _ => Err(self.control.error_unrecognized(&t)),
        }
      }
      '&' => {
        let t = self.until(current_char, |ch| !['&', '='].contains(ch));
        match t.as_str() {
          "&=" => Ok(vec![Token::new(TokenType::BitwiseAndAssignment, "&=")]),
          "&&" => Ok(vec![Token::new(TokenType::BooleanAnd, "&&")]),
          // "&$" => {
          //   let last_position = self.control.position;
          //   if let Ok(tokens) = VariableToken::lex(self) {
          //     let mut tokens = tokens.clone();
          //     tokens.insert(0, Token::new(TokenType::Reference, "&"));
          //     return Ok(tokens);
          //   } else {
          //     self.control.position = last_position;
          //     return Err(self.control.error_unrecognized(&t));
          //   }
          // }
          "&" => Ok(vec![Token::new(TokenType::BitwiseAnd, "&")]),
          _ => Err(self.control.error_unrecognized(&t)),
        }
      }
      '#' => {
        let t = self.until(current_char, |ch| !['#', '['].contains(ch));
        match t.as_str() {
          "#[" => Ok(vec![Token::new(TokenType::Attribute, "#[")]),
          _ => CommentToken::lex_line(self, &t[1..]),
        }
      }
      '?' => {
        let t = self.until(current_char, |ch| !['?', '>', '=', '-', '{', ':'].contains(ch));
        match t.as_str() {
          "?:" => Ok(vec![Token::new(TokenType::Elvis, "?:")]),
          "?>" => InlineToken::lex(self, Some(Token::new(TokenType::CloseTag, "?>"))),
          "?->" => Ok(vec![Token::new(TokenType::NullsafeObjectAccess, "?->")]),
          "?->{" => Ok(vec![Token::new(TokenType::NullsafeObjectAccessBracketOpen, "?->{")]),
          "??=" => Ok(vec![Token::new(TokenType::CoalesceAssignment, "??=")]),
          "??" => Ok(vec![Token::new(TokenType::Coalesce, "??")]),
          "?" => Ok(vec![Token::new(TokenType::QuestionMark, "?")]),
          _ => Err(self.control.error_unrecognized(&t)),
        }
      }
      '%' => {
        let t = self.until(current_char, |ch| !['%', '=', '>'].contains(ch));
        match t.as_str() {
          "%>" => InlineToken::lex(self, Some(Token::new(TokenType::CloseTagShort, "%>"))),
          "%=" => Ok(vec![Token::new(TokenType::ModulusAssignment, "%=")]),
          "%" => Ok(vec![Token::new(TokenType::Modulus, "%")]),
          _ => Err(self.control.error_unrecognized(&t)),
        }
      }
      '^' => {
        let t = self.until(current_char, |ch| !['^', '='].contains(ch));
        match t.as_str() {
          "^=" => Ok(vec![Token::new(TokenType::BitwiseXorAssignment, "^=")]),
          "^" => Ok(vec![Token::new(TokenType::BitwiseXor, "^")]),
          _ => Err(self.control.error_unrecognized(&t)),
        }
      }
      '*' => {
        let t = self.until(current_char, |ch| !['*', '='].contains(ch));
        match t.as_str() {
          "**=" => Ok(vec![Token::new(TokenType::ExponentiationAssignment, "**=")]),
          "*=" => Ok(vec![Token::new(TokenType::MultiplicationAssignment, "*=")]),
          "**" => Ok(vec![Token::new(TokenType::Exponentiation, "**")]),
          "*" => Ok(vec![Token::new(TokenType::Multiplication, "*")]),
          _ => Err(self.control.error_unrecognized(&t)),
        }
      }
      '/' => {
        let t = self.until(current_char, |ch| !['/', '*', '='].contains(ch));
        match t.as_str() {
          "/=" => Ok(vec![Token::new(TokenType::DivisionAssignment, "/=")]),
          c if c.starts_with("/**") => CommentToken::lex_doc(self, &t[3..]),
          c if c.starts_with("/*") => CommentToken::lex_block(self, &t[2..]),
          c if c.starts_with("//") => CommentToken::lex_line(self, &t[2..]),
          "/" => Ok(vec![Token::new(TokenType::Division, "/")]),
          _ => Err(self.control.error_unrecognized(&t)),
        }
      }
      '.' => {
        let t = self.until(current_char, |ch| !['.', '='].contains(ch));
        match t.as_str() {
          ".=" => Ok(vec![Token::new(TokenType::ConcatenationAssignment, ".=")]),
          "..." => Ok(vec![Token::new(TokenType::Ellipsis, "...")]),
          "." => {
            let mut t = self.control.next_char_until(|_, ch, _| !ch.is_digit(10));
            if t.len() == 0 {
              Ok(vec![Token::new(TokenType::Concatenation, ".")])
            } else {
              t.insert(0, '.');
              Ok(vec![Token::new(TokenType::Number, t.to_string())])
            }
          }
          _ => Err(self.control.error_unrecognized(&t)),
        }
      }
      '|' => {
        let t = self.until(current_char, |ch| !['|', '='].contains(ch));
        match t.as_str() {
          "|=" => Ok(vec![Token::new(TokenType::BitwiseOrAssignment, "|=")]),
          "||" => Ok(vec![Token::new(TokenType::BooleanOr, "||")]),
          "|" => Ok(vec![Token::new(TokenType::BitwiseOr, "|")]),
          _ => Err(self.control.error_unrecognized(&t)),
        }
      }
      '-' => {
        let t = self.until(current_char, |ch| !['-', '=', '>', '{'].contains(ch));
        match t.as_str() {
          "-=" => Ok(vec![Token::new(TokenType::SubtractionAssignment, "-=")]),
          "->{" => {
            let mut tokens = vec![Token::new(TokenType::ObjectAccessBracketOpen, "{")];
            tokens.extend(self.next_tokens_until_right_bracket());
            tokens.push(Token::new(TokenType::ObjectAccessBracketClose, "}"));
            Ok(tokens)
          }
          "->" => Ok(vec![Token::new(TokenType::ObjectAccess, "->")]),
          "--" => {
            let is_post = match self.control.peek_char(None) {
              Some(t) => t.is_whitespace() || [';', ',', ')', ']', '}', '?'].contains(&t),
              None => true,
            };
            if is_post {
              return Ok(vec![Token::new(TokenType::PostDecrement, "--")]);
            }
            return Ok(vec![Token::new(TokenType::PreDecrement, "--")]);
          }
          "-" => Ok(vec![Token::new(TokenType::Subtraction, "-")]),
          _ => Err(self.control.error_unrecognized(&t)),
        }
      }
      '>' => {
        let t = self.until(current_char, |ch| !['>', '='].contains(ch));
        match t.as_str() {
          ">>=" => Ok(vec![Token::new(TokenType::BitwiseShiftRightAssignment, ">>=")]),
          ">=" => Ok(vec![Token::new(TokenType::IsGreaterOrEqual, ">=")]),
          ">>" => Ok(vec![Token::new(TokenType::BitwiseShiftRight, ">>")]),
          ">" => Ok(vec![Token::new(TokenType::IsGreater, ">")]),
          _ => Err(self.control.error_unrecognized(&t)),
        }
      }
      '<' => {
        let t = self.until(current_char, |ch| !['<', '=', '>'].contains(ch));
        match t.as_str() {
          "<=>" => Ok(vec![Token::new(TokenType::Spaceship, "<=>")]),
          "<>" => Ok(vec![Token::new(TokenType::IsNotEqual, "<>")]),
          "<=" => Ok(vec![Token::new(TokenType::IsLesserOrEqual, "<=")]),
          "<<=" => Ok(vec![Token::new(TokenType::BitwiseShiftLeftAssignment, "<<=")]),
          "<<<" => StringToken::lex_doc(self),
          "<<" => Ok(vec![Token::new(TokenType::BitwiseShiftLeft, "<<")]),
          "<" => Ok(vec![Token::new(TokenType::IsLesser, "<")]),
          _ => Err(self.control.error_unrecognized(&t)),
        }
      }
      ':' => {
        let t = self.until(current_char, |ch| ![':'].contains(ch));
        match t.as_str() {
          "::" => Ok(vec![Token::new(TokenType::DoubleColon, "::")]),
          ":" => Ok(vec![Token::new(TokenType::Colon, ":")]),
          _ => Err(self.control.error_unrecognized(&t)),
        }
      }
      '!' => {
        let t = self.until(current_char, |ch| !['!', '='].contains(ch));
        match t.as_str() {
          "!==" => Ok(vec![Token::new(TokenType::IsNotIdentical, "!==")]),
          "!=" => Ok(vec![Token::new(TokenType::IsNotEqual, "!=")]),
          "!" => Ok(vec![Token::new(TokenType::BooleanNegate, "!")]),
          _ => Err(self.control.error_unrecognized(&t)),
        }
      }
      '+' => {
        let t = self.until(current_char, |ch| !['+', '='].contains(ch));
        match t.as_str() {
          "+=" => Ok(vec![Token::new(TokenType::AdditionAssignment, "+=")]),
          "++" => {
            let is_post = match self.control.peek_char(None) {
              Some(t) => t.is_whitespace() || [';', ',', ')', ']', '}', '?'].contains(&t),
              None => true,
            };
            if is_post {
              return Ok(vec![Token::new(TokenType::PostIncrement, "++")]);
            }
            return Ok(vec![Token::new(TokenType::PreIncrement, "++")]);
          }
          "+" => Ok(vec![Token::new(TokenType::Addition, "+")]),
          _ => Err(self.control.error_unrecognized(&t)),
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
      '\\' => {
        let t = self.until(current_char, |ch| !(ch.is_alphanumeric() || *ch == '_' || *ch == '\\'));
        Ok(vec![Token::new(TokenType::Name, t)])
      }
      ',' => Ok(vec![Token::new(TokenType::Comma, ",")]),
      ';' => Ok(vec![Token::new(TokenType::Semicolon, ";")]),
      '~' => Ok(vec![Token::new(TokenType::BooleanNegate, "~")]),
      '@' => {
        if let Some(next) = self.control.peek_char(None) {
          if !next.is_whitespace() {
            return Ok(vec![Token::new(TokenType::AtSign, "@")]);
          }
        }
        Err(self.control.error_unrecognized(&"@".to_string()))
      }
      // '\n' => Ok(vec![Token::new(TokenType::LineBreak, "\n")]),
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
