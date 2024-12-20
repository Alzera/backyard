use bstr::{ BString, ByteSlice };

use crate::error::{ LexError, LexResult };
use crate::lexer::{ ControlSnapshot, Lexer, SeriesChecker, SeriesCheckerMode };
use crate::token::{ Token, TokenType };

use super::number::NumberToken;
use super::string::StringToken;

pub struct CommentToken;

impl CommentToken {
  pub fn lex_block(
    lexer: &mut Lexer,
    take_prev_len: usize,
    snapshot: &ControlSnapshot
  ) -> LexResult {
    let againsts = [b"*/".into()];
    let mut checker = SeriesChecker::new(&againsts, SeriesCheckerMode::Comment);
    let mut comment = lexer.control.next_char_until(take_prev_len, |_, ch, _| {
      checker.push(ch);
      checker.check().is_some()
    });
    lexer.control.next_char();
    comment.pop();
    lexer.tokens.push(Token::new(TokenType::CommentBlock, comment, snapshot));
    Ok(())
  }

  pub fn lex_line(
    lexer: &mut Lexer,
    take_prev_len: usize,
    snapshot: &ControlSnapshot
  ) -> LexResult {
    let againsts = [b"\n".into(), b"?>".into()];
    let mut checker = SeriesChecker::new(&againsts, SeriesCheckerMode::Comment);
    let mut comment = lexer.control.next_char_until(take_prev_len, |_, ch, _| {
      checker.push(ch);
      checker.check().is_some()
    });
    if let Some(breaker) = checker.check() {
      if breaker == "?>" {
        lexer.control.next_char();
        comment.pop();
      }
    }
    lexer.tokens.push(Token::new(TokenType::CommentLine, comment, snapshot));
    Ok(())
  }

  pub fn lex_doc(
    lexer: &mut Lexer,
    mut take_prev_len: usize,
    snapshot: &ControlSnapshot
  ) -> LexResult {
    lexer.tokens.push(Token::new(TokenType::CommentDocOpen, b"/**".into(), snapshot));
    lexer.control.next_char_until(0, |_, ch, _| !matches!(ch, b'\t' | b'\x0C' | b' '));
    while let Ok(_) = Self::get_next_doc_token(lexer, &mut take_prev_len) {
      if take_prev_len != 0 {
        take_prev_len -= 1;
      }
      if let Some(last_token) = lexer.tokens.last() {
        if last_token.token_type == TokenType::CommentDocClose {
          break;
        }
      }
    }
    Ok(())
  }

  fn get_next_doc_token(lexer: &mut Lexer, take_prev_len: &mut usize) -> LexResult {
    let snapshot = &lexer.control.get_snapshot();
    let current_char = if *take_prev_len != 0 {
      if let Some(char) = lexer.control.peek_char(Some(*take_prev_len)) {
        char.to_owned()
      } else {
        return Err(LexError::Eof);
      }
    } else if let Some(current_char) = lexer.control.next_char() {
      current_char.to_owned()
    } else {
      return Err(LexError::Eof);
    };

    match current_char {
      b'$' => {
        let t: BString = lexer.until(|ch| !(ch.is_ascii_alphanumeric() || ch == b'_'));
        if t == b"$this" {
          lexer.tokens.push(Token::new(TokenType::This, t, snapshot));
          return Ok(());
        } else if let Some(f) = t.get(1) {
          if f.is_ascii_alphabetic() || *f == b'_' {
            lexer.tokens.push(Token::new(TokenType::Variable, t, snapshot));
            return Ok(());
          }
        }
        lexer.tokens.push(Token::new(TokenType::DocUnknown, t, snapshot));
      }
      b'@' => {
        let t: BString = lexer
          .until(|ch| !(ch.is_ascii_alphanumeric() || ch == b'-'))
          .to_lowercase()
          .into();
        match t.as_slice() {
          b"@param" => {
            lexer.tokens.push(Token::new(TokenType::DocTagParam, t, snapshot));
            Self::try_parse_type(lexer)?;
          }
          b"@phpstan-param" => {
            lexer.tokens.push(Token::new(TokenType::DocTagPhpstanParam, t, snapshot));
            Self::try_parse_type(lexer)?;
          }
          b"@psalm-param" => {
            lexer.tokens.push(Token::new(TokenType::DocTagPsalmParam, t, snapshot));
            Self::try_parse_type(lexer)?;
          }
          b"@phan-param" => {
            lexer.tokens.push(Token::new(TokenType::DocTagPhanParam, t, snapshot));
            Self::try_parse_type(lexer)?;
          }
          b"@param-immediately-invoked-callable" =>
            lexer.tokens.push(
              Token::new(TokenType::DocTagParamImmediatelyInvokedCallable, t, snapshot)
            ),
          b"@phpstan-param-immediately-invoked-callable" =>
            lexer.tokens.push(
              Token::new(TokenType::DocTagPhpstanParamImmediatelyInvokedCallable, t, snapshot)
            ),
          b"@param-later-invoked-callable" =>
            lexer.tokens.push(Token::new(TokenType::DocTagParamLaterInvokedCallable, t, snapshot)),
          b"@phpstan-param-later-invoked-callable" =>
            lexer.tokens.push(
              Token::new(TokenType::DocTagPhpstanParamLaterInvokedCallable, t, snapshot)
            ),
          b"@param-closure-this" => {
            lexer.tokens.push(Token::new(TokenType::DocTagParamClosureThis, t, snapshot));
            Self::try_parse_type(lexer)?;
          }
          b"@phpstan-param-closure-this" => {
            lexer.tokens.push(Token::new(TokenType::DocTagPhpstanParamClosureThis, t, snapshot));
            Self::try_parse_type(lexer)?;
          }
          b"@pure-unless-callable-is-impure" =>
            lexer.tokens.push(Token::new(TokenType::DocTagPureUnlessCallableIsImpure, t, snapshot)),
          b"@phpstan-pure-unless-callable-is-impure" =>
            lexer.tokens.push(
              Token::new(TokenType::DocTagPhpstanPureUnlessCallableIsImpure, t, snapshot)
            ),
          b"@var" => {
            lexer.tokens.push(Token::new(TokenType::DocTagVar, t, snapshot));
            Self::try_parse_type(lexer)?;
          }
          b"@phpstan-var" => {
            lexer.tokens.push(Token::new(TokenType::DocTagPhpstanVar, t, snapshot));
            Self::try_parse_type(lexer)?;
          }
          b"@psalm-var" => {
            lexer.tokens.push(Token::new(TokenType::DocTagPsalmVar, t, snapshot));
            Self::try_parse_type(lexer)?;
          }
          b"@phan-var" => {
            lexer.tokens.push(Token::new(TokenType::DocTagPhanVar, t, snapshot));
            Self::try_parse_type(lexer)?;
          }
          b"@return" => {
            lexer.tokens.push(Token::new(TokenType::DocTagReturn, t, snapshot));
            Self::try_parse_type(lexer)?;
          }
          b"@phpstan-return" => {
            lexer.tokens.push(Token::new(TokenType::DocTagPhpstanReturn, t, snapshot));
            Self::try_parse_type(lexer)?;
          }
          b"@psalm-return" => {
            lexer.tokens.push(Token::new(TokenType::DocTagPsalmReturn, t, snapshot));
            Self::try_parse_type(lexer)?;
          }
          b"@phan-return" => {
            lexer.tokens.push(Token::new(TokenType::DocTagPhanReturn, t, snapshot));
            Self::try_parse_type(lexer)?;
          }
          b"@phan-real-return" => {
            lexer.tokens.push(Token::new(TokenType::DocTagPhanRealReturn, t, snapshot));
            Self::try_parse_type(lexer)?;
          }
          b"@throws" => {
            lexer.tokens.push(Token::new(TokenType::DocTagThrows, t, snapshot));
            Self::try_parse_type(lexer)?;
          }
          b"@phpstan-throws" => {
            lexer.tokens.push(Token::new(TokenType::DocTagPhpstanThrows, t, snapshot));
            Self::try_parse_type(lexer)?;
          }
          b"@mixin" => {
            lexer.tokens.push(Token::new(TokenType::DocTagMixin, t, snapshot));
            Self::try_parse_type(lexer)?;
          }
          b"@phan-mixin" => {
            lexer.tokens.push(Token::new(TokenType::DocTagPhanMixin, t, snapshot));
            Self::try_parse_type(lexer)?;
          }
          b"@psalm-require-extends" => {
            lexer.tokens.push(Token::new(TokenType::DocTagPsalmRequireExtends, t, snapshot));
            Self::try_parse_type(lexer)?;
          }
          b"@phpstan-require-extends" => {
            lexer.tokens.push(Token::new(TokenType::DocTagPhpstanRequireExtends, t, snapshot));
            Self::try_parse_type(lexer)?;
          }
          b"@psalm-require-implements" => {
            lexer.tokens.push(Token::new(TokenType::DocTagPsalmRequireImplements, t, snapshot));
            Self::try_parse_type(lexer)?;
          }
          b"@phpstan-require-implements" => {
            lexer.tokens.push(Token::new(TokenType::DocTagPhpstanRequireImplements, t, snapshot));
            Self::try_parse_type(lexer)?;
          }
          b"@deprecated" => lexer.tokens.push(Token::new(TokenType::DocTagDeprecated, t, snapshot)),
          b"@property" => {
            lexer.tokens.push(Token::new(TokenType::DocTagProperty, t, snapshot));
            Self::try_parse_type(lexer)?;
          }
          b"@property-read" => {
            lexer.tokens.push(Token::new(TokenType::DocTagPropertyRead, t, snapshot));
            Self::try_parse_type(lexer)?;
          }
          b"@property-write" => {
            lexer.tokens.push(Token::new(TokenType::DocTagPropertyWrite, t, snapshot));
            Self::try_parse_type(lexer)?;
          }
          b"@phpstan-property" => {
            lexer.tokens.push(Token::new(TokenType::DocTagPhpstanProperty, t, snapshot));
            Self::try_parse_type(lexer)?;
          }
          b"@phpstan-property-read" => {
            lexer.tokens.push(Token::new(TokenType::DocTagPhpstanPropertyRead, t, snapshot));
            Self::try_parse_type(lexer)?;
          }
          b"@phpstan-property-write" => {
            lexer.tokens.push(Token::new(TokenType::DocTagPhpstanPropertyWrite, t, snapshot));
            Self::try_parse_type(lexer)?;
          }
          b"@psalm-property" => {
            lexer.tokens.push(Token::new(TokenType::DocTagPsalmProperty, t, snapshot));
            Self::try_parse_type(lexer)?;
          }
          b"@psalm-property-read" => {
            lexer.tokens.push(Token::new(TokenType::DocTagPsalmPropertyRead, t, snapshot));
            Self::try_parse_type(lexer)?;
          }
          b"@psalm-property-write" => {
            lexer.tokens.push(Token::new(TokenType::DocTagPsalmPropertyWrite, t, snapshot));
            Self::try_parse_type(lexer)?;
          }
          b"@phan-property" => {
            lexer.tokens.push(Token::new(TokenType::DocTagPhanProperty, t, snapshot));
            Self::try_parse_type(lexer)?;
          }
          b"@phan-property-read" => {
            lexer.tokens.push(Token::new(TokenType::DocTagPhanPropertyRead, t, snapshot));
            Self::try_parse_type(lexer)?;
          }
          b"@phan-property-write" => {
            lexer.tokens.push(Token::new(TokenType::DocTagPhanPropertyWrite, t, snapshot));
            Self::try_parse_type(lexer)?;
          }
          b"@method" => lexer.tokens.push(Token::new(TokenType::DocTagMethod, t, snapshot)),
          b"@phpstan-method" =>
            lexer.tokens.push(Token::new(TokenType::DocTagPhpstanMethod, t, snapshot)),
          b"@psalm-method" =>
            lexer.tokens.push(Token::new(TokenType::DocTagPsalmMethod, t, snapshot)),
          b"@phan-method" =>
            lexer.tokens.push(Token::new(TokenType::DocTagPhanMethod, t, snapshot)),
          b"@template" => lexer.tokens.push(Token::new(TokenType::DocTagTemplate, t, snapshot)),
          b"@phpstan-template" =>
            lexer.tokens.push(Token::new(TokenType::DocTagPhpstanTemplate, t, snapshot)),
          b"@psalm-template" =>
            lexer.tokens.push(Token::new(TokenType::DocTagPsalmTemplate, t, snapshot)),
          b"@phan-template" =>
            lexer.tokens.push(Token::new(TokenType::DocTagPhanTemplate, t, snapshot)),
          b"@template-covariant" =>
            lexer.tokens.push(Token::new(TokenType::DocTagTemplateCovariant, t, snapshot)),
          b"@phpstan-template-covariant" =>
            lexer.tokens.push(Token::new(TokenType::DocTagPhpstanTemplateCovariant, t, snapshot)),
          b"@psalm-template-covariant" =>
            lexer.tokens.push(Token::new(TokenType::DocTagPsalmTemplateCovariant, t, snapshot)),
          b"@template-contravariant" =>
            lexer.tokens.push(Token::new(TokenType::DocTagTemplateContravariant, t, snapshot)),
          b"@phpstan-template-contravariant" =>
            lexer.tokens.push(
              Token::new(TokenType::DocTagPhpstanTemplateContravariant, t, snapshot)
            ),
          b"@psalm-template-contravariant" =>
            lexer.tokens.push(Token::new(TokenType::DocTagPsalmTemplateContravariant, t, snapshot)),
          b"@extends" => lexer.tokens.push(Token::new(TokenType::DocTagExtends, t, snapshot)),
          b"@phpstan-extends" =>
            lexer.tokens.push(Token::new(TokenType::DocTagPhpstanExtends, t, snapshot)),
          b"@phan-extends" =>
            lexer.tokens.push(Token::new(TokenType::DocTagPhanExtends, t, snapshot)),
          b"@phan-inherits" =>
            lexer.tokens.push(Token::new(TokenType::DocTagPhanInherits, t, snapshot)),
          b"@template-extends" =>
            lexer.tokens.push(Token::new(TokenType::DocTagTemplateExtends, t, snapshot)),
          b"@implements" => lexer.tokens.push(Token::new(TokenType::DocTagImplements, t, snapshot)),
          b"@phpstan-implements" =>
            lexer.tokens.push(Token::new(TokenType::DocTagPhpstanImplements, t, snapshot)),
          b"@template-implements" =>
            lexer.tokens.push(Token::new(TokenType::DocTagTemplateImplements, t, snapshot)),
          b"@use" => lexer.tokens.push(Token::new(TokenType::DocTagUse, t, snapshot)),
          b"@phpstan-use" =>
            lexer.tokens.push(Token::new(TokenType::DocTagPhpstanUse, t, snapshot)),
          b"@template-use" =>
            lexer.tokens.push(Token::new(TokenType::DocTagTemplateUse, t, snapshot)),
          b"@phpstan-type" =>
            lexer.tokens.push(Token::new(TokenType::DocTagPhpstanType, t, snapshot)),
          b"@psalm-type" => lexer.tokens.push(Token::new(TokenType::DocTagPsalmType, t, snapshot)),
          b"@phan-type" => lexer.tokens.push(Token::new(TokenType::DocTagPhanType, t, snapshot)),
          b"@phpstan-import-type" =>
            lexer.tokens.push(Token::new(TokenType::DocTagPhpstanImportType, t, snapshot)),
          b"@psalm-import-type" =>
            lexer.tokens.push(Token::new(TokenType::DocTagPsalmImportType, t, snapshot)),
          b"@phpstan-assert" =>
            lexer.tokens.push(Token::new(TokenType::DocTagPhpstanAssert, t, snapshot)),
          b"@phpstan-assert-if-true" =>
            lexer.tokens.push(Token::new(TokenType::DocTagPhpstanAssertIfTrue, t, snapshot)),
          b"@phpstan-assert-if-false" =>
            lexer.tokens.push(Token::new(TokenType::DocTagPhpstanAssertIfFalse, t, snapshot)),
          b"@psalm-assert" =>
            lexer.tokens.push(Token::new(TokenType::DocTagPsalmAssert, t, snapshot)),
          b"@psalm-assert-if-true" =>
            lexer.tokens.push(Token::new(TokenType::DocTagPsalmAssertIfTrue, t, snapshot)),
          b"@psalm-assert-if-false" =>
            lexer.tokens.push(Token::new(TokenType::DocTagPsalmAssertIfFalse, t, snapshot)),
          b"@phan-assert" =>
            lexer.tokens.push(Token::new(TokenType::DocTagPhanAssert, t, snapshot)),
          b"@phan-assert-if-true" =>
            lexer.tokens.push(Token::new(TokenType::DocTagPhanAssertIfTrue, t, snapshot)),
          b"@phan-assert-if-false" =>
            lexer.tokens.push(Token::new(TokenType::DocTagPhanAssertIfFalse, t, snapshot)),
          b"@phpstan-this-out" =>
            lexer.tokens.push(Token::new(TokenType::DocTagPhpstanThisOut, t, snapshot)),
          b"@phpstan-self-out" =>
            lexer.tokens.push(Token::new(TokenType::DocTagPhpstanSelfOut, t, snapshot)),
          b"@psalm-this-out" =>
            lexer.tokens.push(Token::new(TokenType::DocTagPsalmThisOut, t, snapshot)),
          b"@psalm-self-out" =>
            lexer.tokens.push(Token::new(TokenType::DocTagPsalmSelfOut, t, snapshot)),
          b"@param-out" => lexer.tokens.push(Token::new(TokenType::DocTagParamOut, t, snapshot)),
          b"@phpstan-param-out" =>
            lexer.tokens.push(Token::new(TokenType::DocTagPhpstanParamOut, t, snapshot)),
          b"@psalm-param-out" =>
            lexer.tokens.push(Token::new(TokenType::DocTagPsalmParamOut, t, snapshot)),
          _ => lexer.tokens.push(Token::new(TokenType::DocUnknown, t, snapshot)),
        }
      }
      b'*' => {
        if lexer.control.peek_char(None) == Some(&b'/') {
          lexer.control.next_char();
          lexer.tokens.push(Token::new(TokenType::CommentDocClose, "*/".into(), snapshot));
        } else {
          lexer.tokens.push(Token::new(TokenType::DocUnknown, "*".into(), snapshot));
        }
      }
      b'\n' => {
        lexer.control.next_char_until(0, |_, ch, _| !matches!(ch, b'\t' | b'\x0C' | b' '));
        if lexer.control.peek_char(None) == Some(&b'*') {
          lexer.control.next_char();
        }
      }
      b'\t' | b'\x0C' | b' ' => {
        let t = lexer.until(|ch| !matches!(ch, b'\t' | b'\x0C' | b' '));
        if !(t.len() == 1 && t[0] == b' ') {
          lexer.tokens.push(Token::new(TokenType::DocHorizontalWhitespace, t, snapshot));
        }
      }
      _ => {
        let t = lexer.until(|ch| (ch.is_ascii_whitespace() || ch == b'*' || ch == b'@'));
        lexer.tokens.push(Token::new(TokenType::DocUnknown, t, snapshot));
      }
    }
    Ok(())
  }

  // fn is_valid_label(lexer: &mut Lexer, t: &BString, snapshot: &ControlSnapshot) -> bool {
  //   if let Some(next_char) = lexer.control.peek_char(None) {
  //     if *next_char == b':' {
  //       let snapshot = &lexer.control.get_snapshot();
  //       lexer.tokens.push(Token::new(TokenType::Label, t.to_owned(), snapshot));
  //       lexer.control.next_char();
  //       lexer.tokens.push(Token::new(TokenType::Colon, ":".into(), snapshot));
  //       return true;
  //     } else if *next_char == b'?' {
  //       if let Some(next_char) = lexer.control.peek_char(Some(lexer.control.get_position() + 1)) {
  //         if *next_char == b':' {
  //           lexer.tokens.push(Token::new(TokenType::Label, t.to_owned(), snapshot));
  //           let snapshot = &lexer.control.get_snapshot();
  //           lexer.tokens.push(Token::new(TokenType::Colon, "?".into(), snapshot));
  //           lexer.control.consume(2);
  //           let snapshot = &lexer.control.get_last_snapshot();
  //           lexer.tokens.push(Token::new(TokenType::Colon, ":".into(), snapshot));
  //           return true;
  //         }
  //       }
  //     }
  //   }
  //   false
  // }

  fn try_parse_type(lexer: &mut Lexer) -> LexResult {
    let mut expect_non_type = false;
    loop {
      lexer.control.next_char_until(0, |_, ch, _| !matches!(ch, b'\t' | b'\x0C' | b' '));

      if let Some(next_char) = lexer.control.peek_char(None) {
        if expect_non_type {
          if next_char.is_ascii_alphanumeric() || [b'\\'].contains(&next_char) {
            break;
          }
          expect_non_type = false;
        }
        if *next_char == b'$' {
          break;
        }
      }

      let snapshot = &lexer.control.get_snapshot();
      let current_char = if let Some(current_char) = lexer.control.next_char() {
        current_char.to_owned()
      } else {
        return Err(LexError::Eof);
      };

      let _ = match current_char {
        c if c.is_ascii_digit() => {
          NumberToken::lex(lexer, &current_char, snapshot)?;
          expect_non_type = true;
        }
        c if c.is_ascii_alphabetic() => {
          let t = lexer.until(|ch| !(ch.is_ascii_alphanumeric() || ch == b'_' || ch == b'\\'));
          if t.contains(&b'\\') {
            if t.starts_with(b"namespace") {
              lexer.tokens.push(Token::new(TokenType::RelativeName, t, snapshot));
            } else {
              lexer.tokens.push(Token::new(TokenType::QualifiedName, t, snapshot));
            }
          } else {
            let t_lowercase: BString = t.to_ascii_lowercase().into();
            let t_lowercase_sliced = t_lowercase.as_slice();
            if TYPE_KEYWORDS.contains(&t_lowercase_sliced) {
              lexer.tokens.push(Token::new(TokenType::Type, t_lowercase, snapshot));
            } else {
              lexer.tokens.push(Token::new(TokenType::UnqualifiedName, t, snapshot));
            }
          }
          expect_non_type = true;
        }
        b'(' => lexer.tokens.push(Token::new(TokenType::LeftParenthesis, "(".into(), snapshot)),
        b')' => lexer.tokens.push(Token::new(TokenType::RightParenthesis, ")".into(), snapshot)),
        b'<' => lexer.tokens.push(Token::new(TokenType::LeftAngleBracket, "<".into(), snapshot)),
        b'>' => lexer.tokens.push(Token::new(TokenType::RightAngleBracket, ">".into(), snapshot)),
        b'{' => lexer.tokens.push(Token::new(TokenType::LeftCurlyBracket, "{".into(), snapshot)),
        b'}' => lexer.tokens.push(Token::new(TokenType::RightCurlyBracket, "}".into(), snapshot)),
        b'[' => lexer.tokens.push(Token::new(TokenType::LeftSquareBracket, "[".into(), snapshot)),
        b']' => lexer.tokens.push(Token::new(TokenType::RightSquareBracket, "]".into(), snapshot)),
        b':' => lexer.tokens.push(Token::new(TokenType::Colon, ":".into(), snapshot)),
        b',' => lexer.tokens.push(Token::new(TokenType::Comma, ",".into(), snapshot)),
        b'|' => lexer.tokens.push(Token::new(TokenType::BitwiseOr, "|".into(), snapshot)),
        b'\'' => StringToken::lex_basic(lexer, snapshot)?,
        b'?' => lexer.tokens.push(Token::new(TokenType::QuestionMark, "?".into(), snapshot)),
        b'&' => {
          lexer.tokens.push(Token::new(TokenType::BitwiseAnd, "&".into(), snapshot));
        }
        b'.' => {
          let t = lexer.until(|ch| ![b'.', b'='].contains(&ch));
          if t == b"..." {
            lexer.tokens.push(Token::new(TokenType::Ellipsis, "...".into(), snapshot));
          } else {
            lexer.tokens.push(Token::new(TokenType::DocUnknown, t, snapshot));
            break;
          }
        }
        b'\\' => {
          let t = lexer.until(|ch| !(ch.is_ascii_alphanumeric() || ch == b'_' || ch == b'\\'));
          lexer.tokens.push(Token::new(TokenType::FullyQualifiedName, t, snapshot));
          expect_non_type = true;
        }
        b'*' => {
          if lexer.control.peek_char(None) == Some(&b'/') {
            lexer.control.next_char();
            lexer.tokens.push(Token::new(TokenType::CommentDocClose, "*/".into(), snapshot));
            break;
          } else {
            lexer.tokens.push(Token::new(TokenType::Multiplication, "*".into(), snapshot));
          }
        }
        b'\n' => {
          lexer.control.next_char_until(0, |_, ch, _| !matches!(ch, b'\t' | b'\x0C' | b' '));
          if lexer.control.peek_char(None) == Some(&b'*') {
            lexer.control.next_char();
          }
        }
        _ => {
          break;
        }
      };
    }
    Ok(())
  }
}

const TYPE_KEYWORDS: &[&[u8]] = &[
  b"array",
  b"list",
  b"non-empty-array",
  b"non-empty-list",
  b"bool",
  b"boolean",
  b"real",
  b"double",
  b"float",
  b"int",
  b"integer",
  b"object",
  b"string",
  b"mixed",
  b"void",
  b"null",
  b"callable",
];
