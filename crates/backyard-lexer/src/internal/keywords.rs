use bstr::{ BString, ByteVec };

use crate::{ error::LexResult, lexer::{ ControlSnapshot, Lexer }, token::{ Token, TokenType } };

pub struct KeywordToken;

impl KeywordToken {
  const KEYS: &[&'static [u8]; 75] = &[
    b"abstract",
    b"array",
    b"as",
    b"break",
    b"callable",
    b"case",
    b"catch",
    b"class",
    b"clone",
    b"const",
    b"continue",
    b"declare",
    b"default",
    b"do",
    b"echo",
    b"else",
    b"elseif",
    b"enddeclare",
    b"endfor",
    b"endforeach",
    b"endif",
    b"endswitch",
    b"endwhile",
    b"enum",
    b"exit",
    b"eval",
    b"die",
    b"extends",
    b"false",
    b"final",
    b"finally",
    b"fn",
    b"for",
    b"foreach",
    b"from",
    b"function",
    b"get",
    b"global",
    b"goto",
    b"if",
    b"implements",
    b"include",
    b"include_once",
    b"instanceof",
    b"insteadof",
    b"interface",
    b"list",
    b"and",
    b"or",
    b"match",
    b"namespace",
    b"new",
    b"null",
    b"print",
    b"private",
    b"protected",
    b"public",
    b"readonly",
    b"require",
    b"require_once",
    b"return",
    b"static",
    b"true",
    b"parent",
    b"self",
    b"set",
    b"switch",
    b"throw",
    b"trait",
    b"try",
    b"use",
    b"var",
    b"while",
    b"yield",
    b"xor",
  ];

  pub fn is_keyword(input: &[u8]) -> bool {
    Self::KEYS.contains(&input)
  }

  pub fn lex(lexer: &mut Lexer, input: BString, snapshot: &ControlSnapshot) -> LexResult {
    let token = match input.as_slice() {
      b"abstract" => Token::new(TokenType::Abstract, input.into(), snapshot),
      b"array" => Token::new(TokenType::Array, input.into(), snapshot),
      b"as" => Token::new(TokenType::As, input.into(), snapshot),
      b"break" => Token::new(TokenType::Break, input.into(), snapshot),
      b"callable" => Token::new(TokenType::Callable, input.into(), snapshot),
      b"case" => Token::new(TokenType::Case, input.into(), snapshot),
      b"catch" => Token::new(TokenType::Catch, input.into(), snapshot),
      b"class" => Token::new(TokenType::Class, input.into(), snapshot),
      b"clone" => Token::new(TokenType::Clone, input.into(), snapshot),
      b"const" => Token::new(TokenType::Const, input.into(), snapshot),
      b"continue" => Token::new(TokenType::Continue, input.into(), snapshot),
      b"declare" => Token::new(TokenType::Declare, input.into(), snapshot),
      b"default" => Token::new(TokenType::Default, input.into(), snapshot),
      b"do" => Token::new(TokenType::Do, input.into(), snapshot),
      b"echo" => Token::new(TokenType::Echo, input.into(), snapshot),
      b"else" => Token::new(TokenType::Else, input.into(), snapshot),
      b"elseif" => Token::new(TokenType::ElseIf, input.into(), snapshot),
      b"enddeclare" => Token::new(TokenType::EndDeclare, input.into(), snapshot),
      b"endfor" => Token::new(TokenType::EndFor, input.into(), snapshot),
      b"endforeach" => Token::new(TokenType::EndForeach, input.into(), snapshot),
      b"endif" => Token::new(TokenType::EndIf, input.into(), snapshot),
      b"endswitch" => Token::new(TokenType::EndSwitch, input.into(), snapshot),
      b"endwhile" => Token::new(TokenType::EndWhile, input.into(), snapshot),
      b"enum" => Token::new(TokenType::Enum, input.into(), snapshot),
      b"exit" => Token::new(TokenType::Exit, input.into(), snapshot),
      b"eval" => Token::new(TokenType::Eval, input.into(), snapshot),
      b"die" => Token::new(TokenType::Die, input.into(), snapshot),
      b"extends" => Token::new(TokenType::Extends, input.into(), snapshot),
      b"false" => Token::new(TokenType::False, input.into(), snapshot),
      b"final" => Token::new(TokenType::Final, input.into(), snapshot),
      b"finally" => Token::new(TokenType::Finally, input.into(), snapshot),
      b"fn" => Token::new(TokenType::Fn, input.into(), snapshot),
      b"for" => Token::new(TokenType::For, input.into(), snapshot),
      b"foreach" => Token::new(TokenType::Foreach, input.into(), snapshot),
      b"from" => Token::new(TokenType::From, input.into(), snapshot),
      b"function" => Token::new(TokenType::Function, input.into(), snapshot),
      b"get" => Token::new(TokenType::Get, input.into(), snapshot),
      b"global" => Token::new(TokenType::Global, input.into(), snapshot),
      b"goto" => Token::new(TokenType::Goto, input.into(), snapshot),
      b"if" => Token::new(TokenType::If, input.into(), snapshot),
      b"implements" => Token::new(TokenType::Implements, input.into(), snapshot),
      b"include" => Token::new(TokenType::Include, input.into(), snapshot),
      b"include_once" => Token::new(TokenType::IncludeOnce, input.into(), snapshot),
      b"instanceof" => Token::new(TokenType::InstanceOf, input.into(), snapshot),
      b"insteadof" => Token::new(TokenType::InsteadOf, input.into(), snapshot),
      b"interface" => Token::new(TokenType::Interface, input.into(), snapshot),
      b"list" => Token::new(TokenType::List, input.into(), snapshot),
      b"and" => Token::new(TokenType::And, input.into(), snapshot),
      b"or" => Token::new(TokenType::Or, input.into(), snapshot),
      b"match" => Token::new(TokenType::Match, input.into(), snapshot),
      b"namespace" => Token::new(TokenType::Namespace, input.into(), snapshot),
      b"new" => Token::new(TokenType::New, input.into(), snapshot),
      b"null" => Token::new(TokenType::Null, input.into(), snapshot),
      b"print" => Token::new(TokenType::Print, input.into(), snapshot),
      b"readonly" => Token::new(TokenType::Readonly, input.into(), snapshot),
      b"require" => Token::new(TokenType::Require, input.into(), snapshot),
      b"require_once" => Token::new(TokenType::RequireOnce, input.into(), snapshot),
      b"return" => Token::new(TokenType::Return, input.into(), snapshot),
      b"static" => Token::new(TokenType::Static, input.into(), snapshot),
      b"parent" => Token::new(TokenType::Parent, input.into(), snapshot),
      b"self" => Token::new(TokenType::SelfKeyword, input.into(), snapshot),
      b"set" => Token::new(TokenType::Set, input.into(), snapshot),
      b"switch" => Token::new(TokenType::Switch, input.into(), snapshot),
      b"throw" => Token::new(TokenType::Throw, input.into(), snapshot),
      b"trait" => Token::new(TokenType::Trait, input.into(), snapshot),
      b"true" => Token::new(TokenType::True, input.into(), snapshot),
      b"try" => Token::new(TokenType::Try, input.into(), snapshot),
      b"use" => Token::new(TokenType::Use, input.into(), snapshot),
      b"var" => Token::new(TokenType::Var, input.into(), snapshot),
      b"while" => Token::new(TokenType::While, input.into(), snapshot),
      b"yield" => Token::new(TokenType::Yield, input.into(), snapshot),
      b"xor" => Token::new(TokenType::Xor, input.into(), snapshot),
      _ => {
        return Self::check_visibility(lexer, input, snapshot);
      }
    };
    lexer.tokens.push(token);
    Ok(())
  }

  fn check_visibility(
    lexer: &mut Lexer,
    mut input: BString,
    snapshot: &ControlSnapshot
  ) -> LexResult {
    let token_type = match input.as_slice() {
      b"private" => TokenType::Private,
      b"protected" => TokenType::Protected,
      b"public" => TokenType::Public,
      _ => {
        return Err(lexer.control.error_unrecognized(input.to_string().split_off(1)));
      }
    };
    if let Some(pos) = lexer.control.peek_char_n(None, 5) {
      if pos == "(get)" {
        lexer.control.consume(5);
        input.push_str(pos);
        lexer.tokens.push(
          Token::new(
            match token_type {
              TokenType::Private => TokenType::PrivateGet,
              TokenType::Protected => TokenType::ProtectedGet,
              TokenType::Public => TokenType::PublicGet,
              _ => {
                return Err(lexer.control.error_unrecognized(input.to_string().split_off(1)));
              }
            },
            input,
            snapshot
          )
        );
        return Ok(());
      } else if pos == "(set)" {
        lexer.control.consume(5);
        input.push_str(pos);
        lexer.tokens.push(
          Token::new(
            match token_type {
              TokenType::Private => TokenType::PrivateSet,
              TokenType::Protected => TokenType::ProtectedSet,
              TokenType::Public => TokenType::PublicSet,
              _ => {
                return Err(lexer.control.error_unrecognized(input.to_string().split_off(1)));
              }
            },
            input,
            snapshot
          )
        );
        return Ok(());
      }
    }
    lexer.tokens.push(Token::new(token_type, input.into(), snapshot));
    Ok(())
  }
}
