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
      b"abstract" => Token::new(TokenType::Abstract, input, snapshot),
      b"array" => Token::new(TokenType::Array, input, snapshot),
      b"as" => Token::new(TokenType::As, input, snapshot),
      b"break" => Token::new(TokenType::Break, input, snapshot),
      b"callable" => Token::new(TokenType::Callable, input, snapshot),
      b"case" => Token::new(TokenType::Case, input, snapshot),
      b"catch" => Token::new(TokenType::Catch, input, snapshot),
      b"class" => Token::new(TokenType::Class, input, snapshot),
      b"clone" => Token::new(TokenType::Clone, input, snapshot),
      b"const" => Token::new(TokenType::Const, input, snapshot),
      b"continue" => Token::new(TokenType::Continue, input, snapshot),
      b"declare" => Token::new(TokenType::Declare, input, snapshot),
      b"default" => Token::new(TokenType::Default, input, snapshot),
      b"do" => Token::new(TokenType::Do, input, snapshot),
      b"echo" => Token::new(TokenType::Echo, input, snapshot),
      b"else" => Token::new(TokenType::Else, input, snapshot),
      b"elseif" => Token::new(TokenType::ElseIf, input, snapshot),
      b"enddeclare" => Token::new(TokenType::EndDeclare, input, snapshot),
      b"endfor" => Token::new(TokenType::EndFor, input, snapshot),
      b"endforeach" => Token::new(TokenType::EndForeach, input, snapshot),
      b"endif" => Token::new(TokenType::EndIf, input, snapshot),
      b"endswitch" => Token::new(TokenType::EndSwitch, input, snapshot),
      b"endwhile" => Token::new(TokenType::EndWhile, input, snapshot),
      b"enum" => Token::new(TokenType::Enum, input, snapshot),
      b"exit" => Token::new(TokenType::Exit, input, snapshot),
      b"eval" => Token::new(TokenType::Eval, input, snapshot),
      b"die" => Token::new(TokenType::Die, input, snapshot),
      b"extends" => Token::new(TokenType::Extends, input, snapshot),
      b"false" => Token::new(TokenType::False, input, snapshot),
      b"final" => Token::new(TokenType::Final, input, snapshot),
      b"finally" => Token::new(TokenType::Finally, input, snapshot),
      b"fn" => Token::new(TokenType::Fn, input, snapshot),
      b"for" => Token::new(TokenType::For, input, snapshot),
      b"foreach" => Token::new(TokenType::Foreach, input, snapshot),
      b"from" => Token::new(TokenType::From, input, snapshot),
      b"function" => Token::new(TokenType::Function, input, snapshot),
      b"get" => Token::new(TokenType::Get, input, snapshot),
      b"global" => Token::new(TokenType::Global, input, snapshot),
      b"goto" => Token::new(TokenType::Goto, input, snapshot),
      b"if" => Token::new(TokenType::If, input, snapshot),
      b"implements" => Token::new(TokenType::Implements, input, snapshot),
      b"include" => Token::new(TokenType::Include, input, snapshot),
      b"include_once" => Token::new(TokenType::IncludeOnce, input, snapshot),
      b"instanceof" => Token::new(TokenType::InstanceOf, input, snapshot),
      b"insteadof" => Token::new(TokenType::InsteadOf, input, snapshot),
      b"interface" => Token::new(TokenType::Interface, input, snapshot),
      b"list" => Token::new(TokenType::List, input, snapshot),
      b"and" => Token::new(TokenType::And, input, snapshot),
      b"or" => Token::new(TokenType::Or, input, snapshot),
      b"match" => Token::new(TokenType::Match, input, snapshot),
      b"namespace" => Token::new(TokenType::Namespace, input, snapshot),
      b"new" => Token::new(TokenType::New, input, snapshot),
      b"null" => Token::new(TokenType::Null, input, snapshot),
      b"print" => Token::new(TokenType::Print, input, snapshot),
      b"readonly" => Token::new(TokenType::Readonly, input, snapshot),
      b"require" => Token::new(TokenType::Require, input, snapshot),
      b"require_once" => Token::new(TokenType::RequireOnce, input, snapshot),
      b"return" => Token::new(TokenType::Return, input, snapshot),
      b"static" => Token::new(TokenType::Static, input, snapshot),
      b"parent" => Token::new(TokenType::Parent, input, snapshot),
      b"self" => Token::new(TokenType::SelfKeyword, input, snapshot),
      b"set" => Token::new(TokenType::Set, input, snapshot),
      b"switch" => Token::new(TokenType::Switch, input, snapshot),
      b"throw" => Token::new(TokenType::Throw, input, snapshot),
      b"trait" => Token::new(TokenType::Trait, input, snapshot),
      b"true" => Token::new(TokenType::True, input, snapshot),
      b"try" => Token::new(TokenType::Try, input, snapshot),
      b"use" => Token::new(TokenType::Use, input, snapshot),
      b"var" => Token::new(TokenType::Var, input, snapshot),
      b"while" => Token::new(TokenType::While, input, snapshot),
      b"yield" => Token::new(TokenType::Yield, input, snapshot),
      b"xor" => Token::new(TokenType::Xor, input, snapshot),
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
    lexer.tokens.push(Token::new(token_type, input, snapshot));
    Ok(())
  }
}
