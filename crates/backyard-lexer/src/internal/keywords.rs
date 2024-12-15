use bstr::{ BString, ByteVec };

use crate::{ lexer::{ ControlSnapshot, Lexer }, token::{ Token, TokenType } };

pub struct KeywordToken;

impl KeywordToken {
  pub fn try_lex(lexer: &mut Lexer, input: &BString, snapshot: &ControlSnapshot) -> Option<Token> {
    let input: BString = input.to_ascii_lowercase().into();
    match input.as_slice() {
      b"abstract" => Some(Token::new(TokenType::Abstract, input.to_owned(), snapshot)),
      b"array" => Some(Token::new(TokenType::Array, input.to_owned(), snapshot)),
      b"as" => Some(Token::new(TokenType::As, input.to_owned(), snapshot)),
      b"break" => Some(Token::new(TokenType::Break, input.to_owned(), snapshot)),
      b"callable" => Some(Token::new(TokenType::Callable, input.to_owned(), snapshot)),
      b"case" => Some(Token::new(TokenType::Case, input.to_owned(), snapshot)),
      b"catch" => Some(Token::new(TokenType::Catch, input.to_owned(), snapshot)),
      b"class" => Some(Token::new(TokenType::Class, input.to_owned(), snapshot)),
      b"clone" => Some(Token::new(TokenType::Clone, input.to_owned(), snapshot)),
      b"const" => Some(Token::new(TokenType::Const, input.to_owned(), snapshot)),
      b"continue" => Some(Token::new(TokenType::Continue, input.to_owned(), snapshot)),
      b"declare" => Some(Token::new(TokenType::Declare, input.to_owned(), snapshot)),
      b"default" => Some(Token::new(TokenType::Default, input.to_owned(), snapshot)),
      b"do" => Some(Token::new(TokenType::Do, input.to_owned(), snapshot)),
      b"echo" => Some(Token::new(TokenType::Echo, input.to_owned(), snapshot)),
      b"else" => Some(Token::new(TokenType::Else, input.to_owned(), snapshot)),
      b"elseif" => Some(Token::new(TokenType::ElseIf, input.to_owned(), snapshot)),
      b"enddeclare" => Some(Token::new(TokenType::EndDeclare, input.to_owned(), snapshot)),
      b"endfor" => Some(Token::new(TokenType::EndFor, input.to_owned(), snapshot)),
      b"endforeach" => Some(Token::new(TokenType::EndForeach, input.to_owned(), snapshot)),
      b"endif" => Some(Token::new(TokenType::EndIf, input.to_owned(), snapshot)),
      b"endswitch" => Some(Token::new(TokenType::EndSwitch, input.to_owned(), snapshot)),
      b"endwhile" => Some(Token::new(TokenType::EndWhile, input.to_owned(), snapshot)),
      b"enum" => Some(Token::new(TokenType::Enum, input.to_owned(), snapshot)),
      b"exit" => Some(Token::new(TokenType::Exit, input.to_owned(), snapshot)),
      b"eval" => Some(Token::new(TokenType::Eval, input.to_owned(), snapshot)),
      b"die" => Some(Token::new(TokenType::Die, input.to_owned(), snapshot)),
      b"extends" => Some(Token::new(TokenType::Extends, input.to_owned(), snapshot)),
      b"false" => Some(Token::new(TokenType::False, input.to_owned(), snapshot)),
      b"final" => Some(Token::new(TokenType::Final, input.to_owned(), snapshot)),
      b"finally" => Some(Token::new(TokenType::Finally, input.to_owned(), snapshot)),
      b"fn" => Some(Token::new(TokenType::Fn, input.to_owned(), snapshot)),
      b"for" => Some(Token::new(TokenType::For, input.to_owned(), snapshot)),
      b"foreach" => Some(Token::new(TokenType::Foreach, input.to_owned(), snapshot)),
      b"from" => Some(Token::new(TokenType::From, input.to_owned(), snapshot)),
      b"function" => Some(Token::new(TokenType::Function, input.to_owned(), snapshot)),
      b"get" => Some(Token::new(TokenType::Get, input.to_owned(), snapshot)),
      b"global" => Some(Token::new(TokenType::Global, input.to_owned(), snapshot)),
      b"goto" => Some(Token::new(TokenType::Goto, input.to_owned(), snapshot)),
      b"if" => Some(Token::new(TokenType::If, input.to_owned(), snapshot)),
      b"implements" => Some(Token::new(TokenType::Implements, input.to_owned(), snapshot)),
      b"include" => Some(Token::new(TokenType::Include, input.to_owned(), snapshot)),
      b"include_once" => Some(Token::new(TokenType::IncludeOnce, input.to_owned(), snapshot)),
      b"instanceof" => Some(Token::new(TokenType::InstanceOf, input.to_owned(), snapshot)),
      b"insteadof" => Some(Token::new(TokenType::InsteadOf, input.to_owned(), snapshot)),
      b"interface" => Some(Token::new(TokenType::Interface, input.to_owned(), snapshot)),
      b"list" => Some(Token::new(TokenType::List, input.to_owned(), snapshot)),
      b"and" => Some(Token::new(TokenType::And, input.to_owned(), snapshot)),
      b"or" => Some(Token::new(TokenType::Or, input.to_owned(), snapshot)),
      b"match" => Some(Token::new(TokenType::Match, input.to_owned(), snapshot)),
      b"namespace" => Some(Token::new(TokenType::Namespace, input.to_owned(), snapshot)),
      b"new" => Some(Token::new(TokenType::New, input.to_owned(), snapshot)),
      b"null" => Some(Token::new(TokenType::Null, input.to_owned(), snapshot)),
      b"print" => Some(Token::new(TokenType::Print, input.to_owned(), snapshot)),
      b"readonly" => Some(Token::new(TokenType::Readonly, input.to_owned(), snapshot)),
      b"require" => Some(Token::new(TokenType::Require, input.to_owned(), snapshot)),
      b"require_once" => Some(Token::new(TokenType::RequireOnce, input.to_owned(), snapshot)),
      b"return" => Some(Token::new(TokenType::Return, input.to_owned(), snapshot)),
      b"static" => Some(Token::new(TokenType::Static, input.to_owned(), snapshot)),
      b"parent" => Some(Token::new(TokenType::Parent, input.to_owned(), snapshot)),
      b"self" => Some(Token::new(TokenType::SelfKeyword, input.to_owned(), snapshot)),
      b"set" => Some(Token::new(TokenType::Set, input.to_owned(), snapshot)),
      b"switch" => Some(Token::new(TokenType::Switch, input.to_owned(), snapshot)),
      b"throw" => Some(Token::new(TokenType::Throw, input.to_owned(), snapshot)),
      b"trait" => Some(Token::new(TokenType::Trait, input.to_owned(), snapshot)),
      b"true" => Some(Token::new(TokenType::True, input.to_owned(), snapshot)),
      b"try" => Some(Token::new(TokenType::Try, input.to_owned(), snapshot)),
      b"use" => Some(Token::new(TokenType::Use, input.to_owned(), snapshot)),
      b"var" => Some(Token::new(TokenType::Var, input.to_owned(), snapshot)),
      b"while" => Some(Token::new(TokenType::While, input.to_owned(), snapshot)),
      b"yield" => Some(Token::new(TokenType::Yield, input.to_owned(), snapshot)),
      b"xor" => Some(Token::new(TokenType::Xor, input.to_owned(), snapshot)),
      b"private" =>
        Some(Self::lex_visibility(lexer, input.to_owned(), TokenType::Private, snapshot)),
      b"protected" =>
        Some(Self::lex_visibility(lexer, input.to_owned(), TokenType::Protected, snapshot)),
      b"public" => Some(Self::lex_visibility(lexer, input.to_owned(), TokenType::Public, snapshot)),
      _ => None,
    }
  }

  fn lex_visibility(
    lexer: &mut Lexer,
    mut input: BString,
    token_type: TokenType,
    snapshot: &ControlSnapshot
  ) -> Token {
    if let Some(pos) = lexer.control.peek_char_n(None, 5) {
      if pos == "(get)" {
        lexer.control.consume(5);
        input.push_str(pos);
        return Token::new(
          match token_type {
            TokenType::Private => TokenType::PrivateGet,
            TokenType::Protected => TokenType::ProtectedGet,
            TokenType::Public => TokenType::PublicGet,
            _ => unreachable!(),
          },
          input,
          snapshot
        );
      } else if pos == "(set)" {
        lexer.control.consume(5);
        input.push_str(pos);
        return Token::new(
          match token_type {
            TokenType::Private => TokenType::PrivateSet,
            TokenType::Protected => TokenType::ProtectedSet,
            TokenType::Public => TokenType::PublicSet,
            _ => unreachable!(),
          },
          input,
          snapshot
        );
      }
    }
    Token::new(token_type, input.to_owned(), snapshot)
  }
}
