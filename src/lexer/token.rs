use std::collections::HashSet;

#[napi(string_enum)]
#[derive(Debug, PartialEq, Eq, Hash)]
pub enum TokenType {
  Type,
  Identifier,
  Magic,
  Whitespace,

  NumberHex,
  Number,

  // $
  Variable,
  VariableBracketOpen,
  VariableBracketClose,

  // =
  Arrow,
  Assignment,
  IsEqual,
  IsIdentical,

  // &
  Reference,
  BitwiseAndAssignment,
  BitwiseAnd,
  BooleanAnd,

  // #
  Attribute,
  CommentLine,

  // ?
  CloseTag,
  NullsafeObjectAccess,
  NullsafeObjectAccessBracketOpen,
  CoalesceAssignment,
  Coalesce,
  QuestionMark,

  // %
  CloseTagShort,
  ModulusAssignment,
  Modulus,

  // ^
  BitwiseXorAssignment,
  BitwiseXor,

  // *
  ExponentiationAssignment,
  MultiplicationAssignment,
  Exponentiation,
  Multiplication,

  // /
  DivisionAssignment,
  CommentDoc,
  CommentBlock,
  Division,

  // .
  ConcatenationAssignment,
  Ellipsis,
  Concatenation,

  // |
  BitwiseOrAssignment,
  BooleanOr,
  BitwiseOr,

  // -
  SubtractionAssignment,
  ObjectAccessBracketOpen,
  ObjectAccessBracketClose,
  ObjectAccess,
  Subtraction,

  // >
  IsGreaterOrEqual,
  IsGreater,
  BitwiseShiftRightAssignment,
  BitwiseShiftRight,

  // <
  IsLesserOrEqual,
  IsLesser,
  IsNotEqual,
  BitwiseShiftLeftAssignment,
  BitwiseShiftLeft,
  OpenTagShort,
  OpenTag,
  OpenTagEcho,
  Spaceship,
  HeredocStart,

  // :
  Colon,
  DoubleColon,

  // !
  BooleanNegate,
  IsNotIdentical,

  // +
  AdditionAssignment,
  Addition,

  // {
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
  BackSlash,
  Comma,
  Semicolon,
  ShortForm,
  ShortFormStart,
  // LineBreak,

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
  Throw,
  Trait,
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

#[napi(object)]
#[derive(Debug)]
pub struct Token {
  pub token_type: TokenType,
  pub value: String,
}

impl Token {
  pub fn new<T: AsRef<str>>(token_type: TokenType, value: T) -> Self {
    Token { token_type, value: value.as_ref().to_string() }
  }
}

impl Clone for Token {
  fn clone(&self) -> Self {
    Self { token_type: self.token_type.clone(), value: self.value.clone() }
  }
}
