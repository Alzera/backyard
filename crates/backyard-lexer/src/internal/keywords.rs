use compact_str::format_compact;

use crate::{ error::LexResult, lexer::{ ControlSnapshot, Lexer }, token::{ Token, TokenType } };

pub struct KeywordToken;

impl KeywordToken {
  const KEYS: [&'static str; 75] = [
    "abstract",
    "array",
    "as",
    "break",
    "callable",
    "case",
    "catch",
    "class",
    "clone",
    "const",
    "continue",
    "declare",
    "default",
    "do",
    "echo",
    "else",
    "elseif",
    "enddeclare",
    "endfor",
    "endforeach",
    "endif",
    "endswitch",
    "endwhile",
    "enum",
    "exit",
    "eval",
    "die",
    "extends",
    "false",
    "final",
    "finally",
    "fn",
    "for",
    "foreach",
    "from",
    "function",
    "get",
    "global",
    "goto",
    "if",
    "implements",
    "include",
    "include_once",
    "instanceof",
    "insteadof",
    "interface",
    "list",
    "and",
    "or",
    "match",
    "namespace",
    "new",
    "null",
    "print",
    "private",
    "protected",
    "public",
    "readonly",
    "require",
    "require_once",
    "return",
    "static",
    "true",
    "parent",
    "self",
    "set",
    "switch",
    "throw",
    "trait",
    "try",
    "use",
    "var",
    "while",
    "yield",
    "xor",
  ];

  pub fn is_keyword(input: &str) -> bool {
    Self::KEYS.contains(&input)
  }

  pub fn lex(lexer: &mut Lexer, input: &str, snapshot: &ControlSnapshot) -> LexResult {
    match input {
      "abstract" => Ok(vec![Token::new(TokenType::Abstract, input.into(), snapshot)]),
      "array" => Ok(vec![Token::new(TokenType::Array, input.into(), snapshot)]),
      "as" => Ok(vec![Token::new(TokenType::As, input.into(), snapshot)]),
      "break" => Ok(vec![Token::new(TokenType::Break, input.into(), snapshot)]),
      "callable" => Ok(vec![Token::new(TokenType::Callable, input.into(), snapshot)]),
      "case" => Ok(vec![Token::new(TokenType::Case, input.into(), snapshot)]),
      "catch" => Ok(vec![Token::new(TokenType::Catch, input.into(), snapshot)]),
      "class" => Ok(vec![Token::new(TokenType::Class, input.into(), snapshot)]),
      "clone" => Ok(vec![Token::new(TokenType::Clone, input.into(), snapshot)]),
      "const" => Ok(vec![Token::new(TokenType::Const, input.into(), snapshot)]),
      "continue" => Ok(vec![Token::new(TokenType::Continue, input.into(), snapshot)]),
      "declare" => Ok(vec![Token::new(TokenType::Declare, input.into(), snapshot)]),
      "default" => Ok(vec![Token::new(TokenType::Default, input.into(), snapshot)]),
      "do" => Ok(vec![Token::new(TokenType::Do, input.into(), snapshot)]),
      "echo" => Ok(vec![Token::new(TokenType::Echo, input.into(), snapshot)]),
      "else" => Ok(vec![Token::new(TokenType::Else, input.into(), snapshot)]),
      "elseif" => Ok(vec![Token::new(TokenType::ElseIf, input.into(), snapshot)]),
      "enddeclare" => Ok(vec![Token::new(TokenType::EndDeclare, input.into(), snapshot)]),
      "endfor" => Ok(vec![Token::new(TokenType::EndFor, input.into(), snapshot)]),
      "endforeach" => Ok(vec![Token::new(TokenType::EndForeach, input.into(), snapshot)]),
      "endif" => Ok(vec![Token::new(TokenType::EndIf, input.into(), snapshot)]),
      "endswitch" => Ok(vec![Token::new(TokenType::EndSwitch, input.into(), snapshot)]),
      "endwhile" => Ok(vec![Token::new(TokenType::EndWhile, input.into(), snapshot)]),
      "enum" => Ok(vec![Token::new(TokenType::Enum, input.into(), snapshot)]),
      "exit" => Ok(vec![Token::new(TokenType::Exit, input.into(), snapshot)]),
      "eval" => Ok(vec![Token::new(TokenType::Eval, input.into(), snapshot)]),
      "die" => Ok(vec![Token::new(TokenType::Die, input.into(), snapshot)]),
      "extends" => Ok(vec![Token::new(TokenType::Extends, input.into(), snapshot)]),
      "false" => Ok(vec![Token::new(TokenType::False, input.into(), snapshot)]),
      "final" => Ok(vec![Token::new(TokenType::Final, input.into(), snapshot)]),
      "finally" => Ok(vec![Token::new(TokenType::Finally, input.into(), snapshot)]),
      "fn" => Ok(vec![Token::new(TokenType::Fn, input.into(), snapshot)]),
      "for" => Ok(vec![Token::new(TokenType::For, input.into(), snapshot)]),
      "foreach" => Ok(vec![Token::new(TokenType::Foreach, input.into(), snapshot)]),
      "from" => Ok(vec![Token::new(TokenType::From, input.into(), snapshot)]),
      "function" => Ok(vec![Token::new(TokenType::Function, input.into(), snapshot)]),
      "get" => Ok(vec![Token::new(TokenType::Get, input.into(), snapshot)]),
      "global" => Ok(vec![Token::new(TokenType::Global, input.into(), snapshot)]),
      "goto" => Ok(vec![Token::new(TokenType::Goto, input.into(), snapshot)]),
      "if" => Ok(vec![Token::new(TokenType::If, input.into(), snapshot)]),
      "implements" => Ok(vec![Token::new(TokenType::Implements, input.into(), snapshot)]),
      "include" => Ok(vec![Token::new(TokenType::Include, input.into(), snapshot)]),
      "include_once" => Ok(vec![Token::new(TokenType::IncludeOnce, input.into(), snapshot)]),
      "instanceof" => Ok(vec![Token::new(TokenType::InstanceOf, input.into(), snapshot)]),
      "insteadof" => Ok(vec![Token::new(TokenType::InsteadOf, input.into(), snapshot)]),
      "interface" => Ok(vec![Token::new(TokenType::Interface, input.into(), snapshot)]),
      "list" => Ok(vec![Token::new(TokenType::List, input.into(), snapshot)]),
      "and" => Ok(vec![Token::new(TokenType::And, input.into(), snapshot)]),
      "or" => Ok(vec![Token::new(TokenType::Or, input.into(), snapshot)]),
      "match" => Ok(vec![Token::new(TokenType::Match, input.into(), snapshot)]),
      "namespace" => Ok(vec![Token::new(TokenType::Namespace, input.into(), snapshot)]),
      "new" => Ok(vec![Token::new(TokenType::New, input.into(), snapshot)]),
      "null" => Ok(vec![Token::new(TokenType::Null, input.into(), snapshot)]),
      "print" => Ok(vec![Token::new(TokenType::Print, input.into(), snapshot)]),
      // "private" => Ok(vec![Token::new(TokenType::Private, input.into(), snapshot)]),
      // "protected" => Ok(vec![Token::new(TokenType::Protected, input.into(), snapshot)]),
      // "public" => Ok(vec![Token::new(TokenType::Public, input.into(), snapshot)]),
      "readonly" => Ok(vec![Token::new(TokenType::Readonly, input.into(), snapshot)]),
      "require" => Ok(vec![Token::new(TokenType::Require, input.into(), snapshot)]),
      "require_once" => Ok(vec![Token::new(TokenType::RequireOnce, input.into(), snapshot)]),
      "return" => Ok(vec![Token::new(TokenType::Return, input.into(), snapshot)]),
      "static" => Ok(vec![Token::new(TokenType::Static, input.into(), snapshot)]),
      "parent" => Ok(vec![Token::new(TokenType::Parent, input.into(), snapshot)]),
      "self" => Ok(vec![Token::new(TokenType::SelfKeyword, input.into(), snapshot)]),
      "set" => Ok(vec![Token::new(TokenType::Set, input.into(), snapshot)]),
      "switch" => Ok(vec![Token::new(TokenType::Switch, input.into(), snapshot)]),
      "throw" => Ok(vec![Token::new(TokenType::Throw, input.into(), snapshot)]),
      "trait" => Ok(vec![Token::new(TokenType::Trait, input.into(), snapshot)]),
      "true" => Ok(vec![Token::new(TokenType::True, input.into(), snapshot)]),
      "try" => Ok(vec![Token::new(TokenType::Try, input.into(), snapshot)]),
      "use" => Ok(vec![Token::new(TokenType::Use, input.into(), snapshot)]),
      "var" => Ok(vec![Token::new(TokenType::Var, input.into(), snapshot)]),
      "while" => Ok(vec![Token::new(TokenType::While, input.into(), snapshot)]),
      "yield" => Ok(vec![Token::new(TokenType::Yield, input.into(), snapshot)]),
      "xor" => Ok(vec![Token::new(TokenType::Xor, input.into(), snapshot)]),
      _ => Self::check_visibility(lexer, input, snapshot),
    }
  }

  fn check_visibility(lexer: &mut Lexer, input: &str, snapshot: &ControlSnapshot) -> LexResult {
    let token_type = match input {
      "private" => [TokenType::Private, TokenType::PrivateGet, TokenType::PrivateSet],
      "protected" => [TokenType::Protected, TokenType::ProtectedGet, TokenType::ProtectedSet],
      "public" => [TokenType::Public, TokenType::PublicGet, TokenType::PublicSet],
      _ => {
        return Err(lexer.control.error_unrecognized(input));
      }
    };
    if let Some(pos) = lexer.control.peek_char_n(None, 5) {
      if pos == "(get)" {
        lexer.control.consume(5);
        return Ok(vec![Token::new(token_type[1], format_compact!("{}(get)", input), snapshot)]);
      } else if pos == "(set)" {
        lexer.control.consume(5);
        return Ok(vec![Token::new(token_type[2], format_compact!("{}(set)", input), snapshot)]);
      }
    }
    Ok(vec![Token::new(token_type[0], input.into(), snapshot)])
  }
}
