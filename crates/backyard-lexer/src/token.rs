use std::fmt::{ Debug, Display };

use bstr::BString;
use serde::{ Deserialize, Serialize };

use crate::lexer::ControlSnapshot;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, Serialize, Deserialize)]
pub enum TokenType {
  Inline,
  Type,
  Magic,
  MagicMethod,

  UnqualifiedName,
  QualifiedName,
  FullyQualifiedName,
  RelativeName,

  NumberBinary,
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
  Get,
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
  PrivateGet,
  PrivateSet,
  Protected,
  ProtectedGet,
  ProtectedSet,
  Public,
  PublicGet,
  PublicSet,
  Readonly,
  Require,
  RequireOnce,
  Return,
  Static,
  Parent,
  SelfKeyword,
  Set,
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

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct Token {
  pub token_type: TokenType,
  pub value: BString,
  pub line: u32,
  pub column: u32,
  pub offset: u32,
}

impl Token {
  pub(crate) fn new(token_type: TokenType, value: BString, snapshot: &ControlSnapshot) -> Self {
    Token {
      token_type,
      value,
      line: snapshot.line,
      column: snapshot.column,
      offset: snapshot.offset,
    }
  }
}

impl Debug for Token {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(
      f,
      "Token {{ token_type: {:?}, value: {:?}, line: {:?}, column: {:?}, offset: {:?} }}",
      self.token_type,
      self.value.to_string(),
      self.line,
      self.column,
      self.offset
    )
  }
}

impl Display for Token {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(
      f,
      "Token {{ token_type: {:?}, value: {:?}, line: {:?}, column: {:?}, offset: {:?} }}",
      self.token_type,
      self.value.to_string(),
      self.line,
      self.column,
      self.offset
    )
  }
}
