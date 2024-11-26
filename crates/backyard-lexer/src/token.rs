use std::collections::HashSet;

use serde::{ Deserialize, Serialize };
use ts_rs::TS;

use crate::lexer::ControlSnapshot;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, Serialize, Deserialize, TS)]
#[ts(export)]
pub enum TokenType {
  Inline,
  Type,
  Identifier,
  Magic,
  Whitespace,

  NumberHex,
  Number,

  Variable,
  VariableBracketOpen,
  VariableBracketClose,

  Arrow,
  Assignment,
  IsEqual,
  IsIdentical,

  BitwiseAndAssignment,
  BitwiseAnd,
  BooleanAnd,
  ReferenceAssignment,

  Attribute,
  CommentLine,

  NullsafeObjectAccess,
  CoalesceAssignment,
  Coalesce,
  QuestionMark,
  Elvis,

  ModulusAssignment,
  Modulus,

  BitwiseXorAssignment,
  BitwiseXor,

  ExponentiationAssignment,
  MultiplicationAssignment,
  Exponentiation,
  Multiplication,

  DivisionAssignment,
  CommentDoc,
  CommentBlock,
  Division,

  ConcatenationAssignment,
  Ellipsis,
  Concatenation,

  BitwiseOrAssignment,
  BooleanOr,
  BitwiseOr,

  SubtractionAssignment,
  ObjectAccess,
  Subtraction,

  IsGreaterOrEqual,
  IsGreater,
  BitwiseShiftRightAssignment,
  BitwiseShiftRight,

  IsLesserOrEqual,
  IsLesser,
  IsNotEqual,
  BitwiseShiftLeftAssignment,
  BitwiseShiftLeft,
  Spaceship,

  Colon,
  DoubleColon,

  BooleanNegate,
  IsNotIdentical,

  AdditionAssignment,
  Addition,

  LeftCurlyBracket,

  AdvanceInterpolationOpen,
  AdvanceInterpolationClose,
  EncapsedStringOpen,
  EncapsedStringClose,
  EncapsedString,
  String,

  PostDecrement,
  PostIncrement,
  PreDecrement,
  PreIncrement,

  LeftParenthesis,
  RightParenthesis,
  RightCurlyBracket,
  LeftSquareBracket,
  RightSquareBracket,
  Name,
  Comma,
  Semicolon,
  AtSign,

  NowDocOpen,
  NowDocClose,
  HeredocOpen,
  HeredocClose,

  Abstract,
  Array,
  As,
  Break,
  Callable,
  Case,
  Catch,
  Class,
  Clone,
  Const,
  Continue,
  Declare,
  Default,
  Do,
  Echo,
  Else,
  ElseIf,
  EndDeclare,
  EndFor,
  EndForeach,
  EndIf,
  EndSwitch,
  EndWhile,
  Enum,
  Exit,
  Eval,
  Die,
  Extends,
  False,
  Final,
  Finally,
  Fn,
  For,
  Foreach,
  From,
  Function,
  Global,
  Goto,
  If,
  Implements,
  Include,
  IncludeOnce,
  InstanceOf,
  InsteadOf,
  Interface,
  List,
  And,
  Or,
  Match,
  Namespace,
  New,
  Null,
  Print,
  Private,
  Protected,
  Public,
  Readonly,
  Require,
  RequireOnce,
  Return,
  Static,
  Parent,
  SelfKeyword,
  Switch,
  This,
  Throw,
  Trait,
  True,
  Try,
  Use,
  Var,
  While,
  Yield,
  Xor,
}

pub trait TokenTypeArrayCombine {
  fn combine(self, tokens: &[TokenType]) -> Vec<TokenType>;
}

impl TokenTypeArrayCombine for &[TokenType] {
  fn combine(self, tokens: &[TokenType]) -> Vec<TokenType> {
    let combined: Vec<TokenType> = [self, tokens].concat();
    let unique: HashSet<_> = combined.into_iter().collect();
    unique.into_iter().collect()
  }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct Token {
  pub token_type: TokenType,
  pub value: String,
  pub line: usize,
  pub column: usize,
}

impl Token {
  pub(crate) fn new<T: AsRef<str>>(
    token_type: TokenType,
    value: T,
    snapshot: &ControlSnapshot
  ) -> Self {
    Token {
      token_type,
      value: value.as_ref().to_string(),
      line: snapshot.line,
      column: snapshot.column,
    }
  }
}
