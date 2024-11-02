use crate::lexer::{ lexer::Lexer, token::{ Token, TokenType }, utils::get_tokens_level };

pub struct KeywordToken {}

impl KeywordToken {
  const KEYS: [&str; 70] = [
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
    "final",
    "finally",
    "fn",
    "for",
    "foreach",
    "from",
    "function",
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
    "print",
    "private",
    "protected",
    "public",
    "readonly",
    "require",
    "require_once",
    "return",
    "static",
    "parent",
    "self",
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

  pub fn is_keyword(input: &String) -> bool {
    Self::KEYS.contains(&input.as_str())
  }

  pub fn lex(input: &String, lexer: &mut Lexer) -> Option<Vec<Token>> {
    match input.as_str() {
      "abstract" => Some(vec![Token::new(TokenType::Abstract, input)]),
      "array" => Some(vec![Token::new(TokenType::Array, input)]),
      "as" => Some(vec![Token::new(TokenType::As, input)]),
      "break" => Some(vec![Token::new(TokenType::Break, input)]),
      "callable" => Some(vec![Token::new(TokenType::Callable, input)]),
      "case" => KeywordToken::short_form(TokenType::Case, input, lexer),
      "catch" => Some(vec![Token::new(TokenType::Catch, input)]),
      "class" => Some(vec![Token::new(TokenType::Class, input)]),
      "clone" => Some(vec![Token::new(TokenType::Clone, input)]),
      "const" => Some(vec![Token::new(TokenType::Const, input)]),
      "continue" => Some(vec![Token::new(TokenType::Continue, input)]),
      "declare" => KeywordToken::detect_short_form_complex(TokenType::Declare, input, lexer),
      "default" => KeywordToken::detect_short_form_simple(TokenType::Default, input, lexer),
      "do" => Some(vec![Token::new(TokenType::Do, input)]),
      "echo" => Some(vec![Token::new(TokenType::Echo, input)]),
      "else" => KeywordToken::detect_short_form_simple(TokenType::Else, input, lexer),
      "elseif" => KeywordToken::detect_short_form_complex(TokenType::ElseIf, input, lexer),
      "enddeclare" => Some(vec![Token::new(TokenType::EndDeclare, input)]),
      "endfor" => Some(vec![Token::new(TokenType::EndFor, input)]),
      "endforeach" => Some(vec![Token::new(TokenType::EndForeach, input)]),
      "endif" => Some(vec![Token::new(TokenType::EndIf, input)]),
      "endswitch" => Some(vec![Token::new(TokenType::EndSwitch, input)]),
      "endwhile" => Some(vec![Token::new(TokenType::EndWhile, input)]),
      "enum" => Some(vec![Token::new(TokenType::Enum, input)]),
      "exit" => Some(vec![Token::new(TokenType::Exit, input)]),
      "eval" => Some(vec![Token::new(TokenType::Eval, input)]),
      "die" => Some(vec![Token::new(TokenType::Die, input)]),
      "extends" => Some(vec![Token::new(TokenType::Extends, input)]),
      "final" => Some(vec![Token::new(TokenType::Final, input)]),
      "finally" => Some(vec![Token::new(TokenType::Finally, input)]),
      "fn" => Some(vec![Token::new(TokenType::Fn, input)]),
      "for" => KeywordToken::detect_short_form_complex(TokenType::For, input, lexer),
      "foreach" => KeywordToken::detect_short_form_complex(TokenType::Foreach, input, lexer),
      "from" => Some(vec![Token::new(TokenType::From, input)]),
      "function" => Some(vec![Token::new(TokenType::Function, input)]),
      "global" => Some(vec![Token::new(TokenType::Global, input)]),
      "goto" => Some(vec![Token::new(TokenType::Goto, input)]),
      "if" => KeywordToken::detect_short_form_complex(TokenType::If, input, lexer),
      "implements" => Some(vec![Token::new(TokenType::Implements, input)]),
      "include" => Some(vec![Token::new(TokenType::Include, input)]),
      "include_once" => Some(vec![Token::new(TokenType::IncludeOnce, input)]),
      "instanceof" => Some(vec![Token::new(TokenType::InstanceOf, input)]),
      "insteadof" => Some(vec![Token::new(TokenType::InsteadOf, input)]),
      "interface" => Some(vec![Token::new(TokenType::Interface, input)]),
      "list" => Some(vec![Token::new(TokenType::List, input)]),
      "and" => Some(vec![Token::new(TokenType::And, input)]),
      "or" => Some(vec![Token::new(TokenType::Or, input)]),
      "match" => Some(vec![Token::new(TokenType::Match, input)]),
      "namespace" => Some(vec![Token::new(TokenType::Namespace, input)]),
      "new" => Some(vec![Token::new(TokenType::New, input)]),
      "print" => Some(vec![Token::new(TokenType::Print, input)]),
      "private" => Some(vec![Token::new(TokenType::Private, input)]),
      "protected" => Some(vec![Token::new(TokenType::Protected, input)]),
      "public" => Some(vec![Token::new(TokenType::Public, input)]),
      "readonly" => Some(vec![Token::new(TokenType::Readonly, input)]),
      "require" => Some(vec![Token::new(TokenType::Require, input)]),
      "require_once" => Some(vec![Token::new(TokenType::RequireOnce, input)]),
      "return" => Some(vec![Token::new(TokenType::Return, input)]),
      "static" => Some(vec![Token::new(TokenType::Static, input)]),
      "parent" => Some(vec![Token::new(TokenType::Parent, input)]),
      "self" => Some(vec![Token::new(TokenType::SelfKeyword, input)]),
      "switch" => KeywordToken::detect_short_form_complex(TokenType::Switch, input, lexer),
      "throw" => Some(vec![Token::new(TokenType::Throw, input)]),
      "trait" => Some(vec![Token::new(TokenType::Trait, input)]),
      "try" => Some(vec![Token::new(TokenType::Try, input)]),
      "use" => Some(vec![Token::new(TokenType::Use, input)]),
      "var" => Some(vec![Token::new(TokenType::Var, input)]),
      "while" => KeywordToken::detect_short_form_complex(TokenType::While, input, lexer),
      "yield" => Some(vec![Token::new(TokenType::Yield, input)]),
      "xor" => Some(vec![Token::new(TokenType::Xor, input)]),
      _ => None,
    }
  }

  fn short_form(token_type: TokenType, input: &String, lexer: &mut Lexer) -> Option<Vec<Token>> {
    let mut tokens = get_tokens_level(lexer, 1, [].to_vec(), [TokenType::Colon].to_vec());
    tokens.insert(0, Token::new(token_type, input));
    tokens.push(Token::new(TokenType::ShortForm, ":"));
    Some(tokens)
  }

  fn detect_short_form_complex(
    token_type: TokenType,
    input: &String,
    lexer: &mut Lexer
  ) -> Option<Vec<Token>> {
    let mut tokens: Vec<Token> = get_tokens_level(
      lexer,
      0,
      [TokenType::LeftParenthesis].to_vec(),
      [TokenType::RightParenthesis].to_vec()
    );
    tokens.insert(0, Token::new(token_type, input));
    tokens.push(Token::new(TokenType::RightParenthesis, ")"));

    if KeywordToken::detect_short_form(lexer) {
      tokens.push(Token::new(TokenType::ShortFormStart, ":"));
    }
    Some(tokens)
  }

  fn detect_short_form_simple(
    token_type: TokenType,
    input: &String,
    lexer: &mut Lexer
  ) -> Option<Vec<Token>> {
    let mut tokens = vec![Token::new(token_type, input)];
    if KeywordToken::detect_short_form(lexer) {
      tokens.push(Token::new(TokenType::ShortForm, ":"));
    }
    Some(tokens)
  }

  fn detect_short_form(lexer: &mut Lexer) -> bool {
    if let Some(current_char) = lexer.chars.get(lexer.position) {
      if *current_char == ':' {
        lexer.position += 1;
        return true;
      }
    }
    false
  }
}
