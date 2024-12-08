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
    let token = match input {
      "abstract" => Token::new(TokenType::Abstract, input.into(), snapshot),
      "array" => Token::new(TokenType::Array, input.into(), snapshot),
      "as" => Token::new(TokenType::As, input.into(), snapshot),
      "break" => Token::new(TokenType::Break, input.into(), snapshot),
      "callable" => Token::new(TokenType::Callable, input.into(), snapshot),
      "case" => Token::new(TokenType::Case, input.into(), snapshot),
      "catch" => Token::new(TokenType::Catch, input.into(), snapshot),
      "class" => Token::new(TokenType::Class, input.into(), snapshot),
      "clone" => Token::new(TokenType::Clone, input.into(), snapshot),
      "const" => Token::new(TokenType::Const, input.into(), snapshot),
      "continue" => Token::new(TokenType::Continue, input.into(), snapshot),
      "declare" => Token::new(TokenType::Declare, input.into(), snapshot),
      "default" => Token::new(TokenType::Default, input.into(), snapshot),
      "do" => Token::new(TokenType::Do, input.into(), snapshot),
      "echo" => Token::new(TokenType::Echo, input.into(), snapshot),
      "else" => Token::new(TokenType::Else, input.into(), snapshot),
      "elseif" => Token::new(TokenType::ElseIf, input.into(), snapshot),
      "enddeclare" => Token::new(TokenType::EndDeclare, input.into(), snapshot),
      "endfor" => Token::new(TokenType::EndFor, input.into(), snapshot),
      "endforeach" => Token::new(TokenType::EndForeach, input.into(), snapshot),
      "endif" => Token::new(TokenType::EndIf, input.into(), snapshot),
      "endswitch" => Token::new(TokenType::EndSwitch, input.into(), snapshot),
      "endwhile" => Token::new(TokenType::EndWhile, input.into(), snapshot),
      "enum" => Token::new(TokenType::Enum, input.into(), snapshot),
      "exit" => Token::new(TokenType::Exit, input.into(), snapshot),
      "eval" => Token::new(TokenType::Eval, input.into(), snapshot),
      "die" => Token::new(TokenType::Die, input.into(), snapshot),
      "extends" => Token::new(TokenType::Extends, input.into(), snapshot),
      "false" => Token::new(TokenType::False, input.into(), snapshot),
      "final" => Token::new(TokenType::Final, input.into(), snapshot),
      "finally" => Token::new(TokenType::Finally, input.into(), snapshot),
      "fn" => Token::new(TokenType::Fn, input.into(), snapshot),
      "for" => Token::new(TokenType::For, input.into(), snapshot),
      "foreach" => Token::new(TokenType::Foreach, input.into(), snapshot),
      "from" => Token::new(TokenType::From, input.into(), snapshot),
      "function" => Token::new(TokenType::Function, input.into(), snapshot),
      "get" => Token::new(TokenType::Get, input.into(), snapshot),
      "global" => Token::new(TokenType::Global, input.into(), snapshot),
      "goto" => Token::new(TokenType::Goto, input.into(), snapshot),
      "if" => Token::new(TokenType::If, input.into(), snapshot),
      "implements" => Token::new(TokenType::Implements, input.into(), snapshot),
      "include" => Token::new(TokenType::Include, input.into(), snapshot),
      "include_once" => Token::new(TokenType::IncludeOnce, input.into(), snapshot),
      "instanceof" => Token::new(TokenType::InstanceOf, input.into(), snapshot),
      "insteadof" => Token::new(TokenType::InsteadOf, input.into(), snapshot),
      "interface" => Token::new(TokenType::Interface, input.into(), snapshot),
      "list" => Token::new(TokenType::List, input.into(), snapshot),
      "and" => Token::new(TokenType::And, input.into(), snapshot),
      "or" => Token::new(TokenType::Or, input.into(), snapshot),
      "match" => Token::new(TokenType::Match, input.into(), snapshot),
      "namespace" => Token::new(TokenType::Namespace, input.into(), snapshot),
      "new" => Token::new(TokenType::New, input.into(), snapshot),
      "null" => Token::new(TokenType::Null, input.into(), snapshot),
      "print" => Token::new(TokenType::Print, input.into(), snapshot),
      // "private" => Token::new(TokenType::Private, input.into(), snapshot),
      // "protected" => Token::new(TokenType::Protected, input.into(), snapshot),
      // "public" => Token::new(TokenType::Public, input.into(), snapshot),
      "readonly" => Token::new(TokenType::Readonly, input.into(), snapshot),
      "require" => Token::new(TokenType::Require, input.into(), snapshot),
      "require_once" => Token::new(TokenType::RequireOnce, input.into(), snapshot),
      "return" => Token::new(TokenType::Return, input.into(), snapshot),
      "static" => Token::new(TokenType::Static, input.into(), snapshot),
      "parent" => Token::new(TokenType::Parent, input.into(), snapshot),
      "self" => Token::new(TokenType::SelfKeyword, input.into(), snapshot),
      "set" => Token::new(TokenType::Set, input.into(), snapshot),
      "switch" => Token::new(TokenType::Switch, input.into(), snapshot),
      "throw" => Token::new(TokenType::Throw, input.into(), snapshot),
      "trait" => Token::new(TokenType::Trait, input.into(), snapshot),
      "true" => Token::new(TokenType::True, input.into(), snapshot),
      "try" => Token::new(TokenType::Try, input.into(), snapshot),
      "use" => Token::new(TokenType::Use, input.into(), snapshot),
      "var" => Token::new(TokenType::Var, input.into(), snapshot),
      "while" => Token::new(TokenType::While, input.into(), snapshot),
      "yield" => Token::new(TokenType::Yield, input.into(), snapshot),
      "xor" => Token::new(TokenType::Xor, input.into(), snapshot),
      _ => {
        return Self::check_visibility(lexer, input, snapshot);
      }
    };
    lexer.tokens.push(token);
    Ok(())
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
        return Ok(
          lexer.tokens.push(Token::new(token_type[1], format_compact!("{}(get)", input), snapshot))
        );
      } else if pos == "(set)" {
        lexer.control.consume(5);
        return Ok(
          lexer.tokens.push(Token::new(token_type[2], format_compact!("{}(set)", input), snapshot))
        );
      }
    }
    lexer.tokens.push(Token::new(token_type[0], input.into(), snapshot));
    Ok(())
  }
}
