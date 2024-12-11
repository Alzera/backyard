use backyard_lexer::lex_eval;

#[test]
fn abstract_test() {
  let arena = bumpalo::Bump::new();
  let tokens = lex_eval(&arena, "abstract").unwrap();
  insta::assert_yaml_snapshot!(tokens);
}

#[test]
fn array_test() {
  let arena = bumpalo::Bump::new();
  let tokens = lex_eval(&arena, "array").unwrap();
  insta::assert_yaml_snapshot!(tokens);
}

#[test]
fn as_test() {
  let arena = bumpalo::Bump::new();
  let tokens = lex_eval(&arena, "as").unwrap();
  insta::assert_yaml_snapshot!(tokens);
}

#[test]
fn break_test() {
  let arena = bumpalo::Bump::new();
  let tokens = lex_eval(&arena, "break").unwrap();
  insta::assert_yaml_snapshot!(tokens);
}

#[test]
fn callable_test() {
  let arena = bumpalo::Bump::new();
  let tokens = lex_eval(&arena, "callable").unwrap();
  insta::assert_yaml_snapshot!(tokens);
}

#[test]
fn case_test() {
  let arena = bumpalo::Bump::new();
  let tokens = lex_eval(&arena, "case").unwrap();
  insta::assert_yaml_snapshot!(tokens);
}

#[test]
fn catch_test() {
  let arena = bumpalo::Bump::new();
  let tokens = lex_eval(&arena, "catch").unwrap();
  insta::assert_yaml_snapshot!(tokens);
}

#[test]
fn class_test() {
  let arena = bumpalo::Bump::new();
  let tokens = lex_eval(&arena, "class").unwrap();
  insta::assert_yaml_snapshot!(tokens);
}

#[test]
fn clone_test() {
  let arena = bumpalo::Bump::new();
  let tokens = lex_eval(&arena, "clone").unwrap();
  insta::assert_yaml_snapshot!(tokens);
}

#[test]
fn const_test() {
  let arena = bumpalo::Bump::new();
  let tokens = lex_eval(&arena, "const").unwrap();
  insta::assert_yaml_snapshot!(tokens);
}

#[test]
fn continue_test() {
  let arena = bumpalo::Bump::new();
  let tokens = lex_eval(&arena, "continue").unwrap();
  insta::assert_yaml_snapshot!(tokens);
}

#[test]
fn declare_test() {
  let arena = bumpalo::Bump::new();
  let tokens = lex_eval(&arena, "declare").unwrap();
  insta::assert_yaml_snapshot!(tokens);
}

#[test]
fn default_test() {
  let arena = bumpalo::Bump::new();
  let tokens = lex_eval(&arena, "default").unwrap();
  insta::assert_yaml_snapshot!(tokens);
}

#[test]
fn do_test() {
  let arena = bumpalo::Bump::new();
  let tokens = lex_eval(&arena, "do").unwrap();
  insta::assert_yaml_snapshot!(tokens);
}

#[test]
fn echo_test() {
  let arena = bumpalo::Bump::new();
  let tokens = lex_eval(&arena, "echo").unwrap();
  insta::assert_yaml_snapshot!(tokens);
}

#[test]
fn else_test() {
  let arena = bumpalo::Bump::new();
  let tokens = lex_eval(&arena, "else").unwrap();
  insta::assert_yaml_snapshot!(tokens);
}

#[test]
fn elseif_test() {
  let arena = bumpalo::Bump::new();
  let tokens = lex_eval(&arena, "elseif").unwrap();
  insta::assert_yaml_snapshot!(tokens);
}

#[test]
fn enddeclare_test() {
  let arena = bumpalo::Bump::new();
  let tokens = lex_eval(&arena, "enddeclare").unwrap();
  insta::assert_yaml_snapshot!(tokens);
}

#[test]
fn endfor_test() {
  let arena = bumpalo::Bump::new();
  let tokens = lex_eval(&arena, "endfor").unwrap();
  insta::assert_yaml_snapshot!(tokens);
}

#[test]
fn endforeach_test() {
  let arena = bumpalo::Bump::new();
  let tokens = lex_eval(&arena, "endforeach").unwrap();
  insta::assert_yaml_snapshot!(tokens);
}

#[test]
fn endif_test() {
  let arena = bumpalo::Bump::new();
  let tokens = lex_eval(&arena, "endif").unwrap();
  insta::assert_yaml_snapshot!(tokens);
}

#[test]
fn endswitch_test() {
  let arena = bumpalo::Bump::new();
  let tokens = lex_eval(&arena, "endswitch").unwrap();
  insta::assert_yaml_snapshot!(tokens);
}

#[test]
fn endwhile_test() {
  let arena = bumpalo::Bump::new();
  let tokens = lex_eval(&arena, "endwhile").unwrap();
  insta::assert_yaml_snapshot!(tokens);
}

#[test]
fn enum_test() {
  let arena = bumpalo::Bump::new();
  let tokens = lex_eval(&arena, "enum").unwrap();
  insta::assert_yaml_snapshot!(tokens);
}

#[test]
fn exit_test() {
  let arena = bumpalo::Bump::new();
  let tokens = lex_eval(&arena, "exit").unwrap();
  insta::assert_yaml_snapshot!(tokens);
}

#[test]
fn eval_test() {
  let arena = bumpalo::Bump::new();
  let tokens = lex_eval(&arena, "eval").unwrap();
  insta::assert_yaml_snapshot!(tokens);
}

#[test]
fn die_test() {
  let arena = bumpalo::Bump::new();
  let tokens = lex_eval(&arena, "die").unwrap();
  insta::assert_yaml_snapshot!(tokens);
}

#[test]
fn extends_test() {
  let arena = bumpalo::Bump::new();
  let tokens = lex_eval(&arena, "extends").unwrap();
  insta::assert_yaml_snapshot!(tokens);
}

#[test]
fn false_test() {
  let arena = bumpalo::Bump::new();
  let tokens = lex_eval(&arena, "false").unwrap();
  insta::assert_yaml_snapshot!(tokens);
}

#[test]
fn final_test() {
  let arena = bumpalo::Bump::new();
  let tokens = lex_eval(&arena, "final").unwrap();
  insta::assert_yaml_snapshot!(tokens);
}

#[test]
fn finally_test() {
  let arena = bumpalo::Bump::new();
  let tokens = lex_eval(&arena, "finally").unwrap();
  insta::assert_yaml_snapshot!(tokens);
}

#[test]
fn fn_test() {
  let arena = bumpalo::Bump::new();
  let tokens = lex_eval(&arena, "fn").unwrap();
  insta::assert_yaml_snapshot!(tokens);
}

#[test]
fn for_test() {
  let arena = bumpalo::Bump::new();
  let tokens = lex_eval(&arena, "for").unwrap();
  insta::assert_yaml_snapshot!(tokens);
}

#[test]
fn foreach_test() {
  let arena = bumpalo::Bump::new();
  let tokens = lex_eval(&arena, "foreach").unwrap();
  insta::assert_yaml_snapshot!(tokens);
}

#[test]
fn from_test() {
  let arena = bumpalo::Bump::new();
  let tokens = lex_eval(&arena, "from").unwrap();
  insta::assert_yaml_snapshot!(tokens);
}

#[test]
fn function_test() {
  let arena = bumpalo::Bump::new();
  let tokens = lex_eval(&arena, "function").unwrap();
  insta::assert_yaml_snapshot!(tokens);
}

#[test]
fn global_test() {
  let arena = bumpalo::Bump::new();
  let tokens = lex_eval(&arena, "global").unwrap();
  insta::assert_yaml_snapshot!(tokens);
}

#[test]
fn goto_test() {
  let arena = bumpalo::Bump::new();
  let tokens = lex_eval(&arena, "goto").unwrap();
  insta::assert_yaml_snapshot!(tokens);
}

#[test]
fn if_test() {
  let arena = bumpalo::Bump::new();
  let tokens = lex_eval(&arena, "if").unwrap();
  insta::assert_yaml_snapshot!(tokens);
}

#[test]
fn implements_test() {
  let arena = bumpalo::Bump::new();
  let tokens = lex_eval(&arena, "implements").unwrap();
  insta::assert_yaml_snapshot!(tokens);
}

#[test]
fn include_test() {
  let arena = bumpalo::Bump::new();
  let tokens = lex_eval(&arena, "include").unwrap();
  insta::assert_yaml_snapshot!(tokens);
}

#[test]
fn include_once_test() {
  let arena = bumpalo::Bump::new();
  let tokens = lex_eval(&arena, "include_once").unwrap();
  insta::assert_yaml_snapshot!(tokens);
}

#[test]
fn instanceof_test() {
  let arena = bumpalo::Bump::new();
  let tokens = lex_eval(&arena, "instanceof").unwrap();
  insta::assert_yaml_snapshot!(tokens);
}

#[test]
fn insteadof_test() {
  let arena = bumpalo::Bump::new();
  let tokens = lex_eval(&arena, "insteadof").unwrap();
  insta::assert_yaml_snapshot!(tokens);
}

#[test]
fn interface_test() {
  let arena = bumpalo::Bump::new();
  let tokens = lex_eval(&arena, "interface").unwrap();
  insta::assert_yaml_snapshot!(tokens);
}

#[test]
fn list_test() {
  let arena = bumpalo::Bump::new();
  let tokens = lex_eval(&arena, "list").unwrap();
  insta::assert_yaml_snapshot!(tokens);
}

#[test]
fn and_test() {
  let arena = bumpalo::Bump::new();
  let tokens = lex_eval(&arena, "and").unwrap();
  insta::assert_yaml_snapshot!(tokens);
}

#[test]
fn or_test() {
  let arena = bumpalo::Bump::new();
  let tokens = lex_eval(&arena, "or").unwrap();
  insta::assert_yaml_snapshot!(tokens);
}

#[test]
fn match_test() {
  let arena = bumpalo::Bump::new();
  let tokens = lex_eval(&arena, "match").unwrap();
  insta::assert_yaml_snapshot!(tokens);
}

#[test]
fn namespace_test() {
  let arena = bumpalo::Bump::new();
  let tokens = lex_eval(&arena, "namespace").unwrap();
  insta::assert_yaml_snapshot!(tokens);
}

#[test]
fn new_test() {
  let arena = bumpalo::Bump::new();
  let tokens = lex_eval(&arena, "new").unwrap();
  insta::assert_yaml_snapshot!(tokens);
}

#[test]
fn null_test() {
  let arena = bumpalo::Bump::new();
  let tokens = lex_eval(&arena, "null").unwrap();
  insta::assert_yaml_snapshot!(tokens);
}

#[test]
fn print_test() {
  let arena = bumpalo::Bump::new();
  let tokens = lex_eval(&arena, "print").unwrap();
  insta::assert_yaml_snapshot!(tokens);
}

#[test]
fn readonly_test() {
  let arena = bumpalo::Bump::new();
  let tokens = lex_eval(&arena, "readonly").unwrap();
  insta::assert_yaml_snapshot!(tokens);
}

#[test]
fn require_test() {
  let arena = bumpalo::Bump::new();
  let tokens = lex_eval(&arena, "require").unwrap();
  insta::assert_yaml_snapshot!(tokens);
}

#[test]
fn require_once_test() {
  let arena = bumpalo::Bump::new();
  let tokens = lex_eval(&arena, "require_once").unwrap();
  insta::assert_yaml_snapshot!(tokens);
}

#[test]
fn return_test() {
  let arena = bumpalo::Bump::new();
  let tokens = lex_eval(&arena, "return").unwrap();
  insta::assert_yaml_snapshot!(tokens);
}

#[test]
fn static_test() {
  let arena = bumpalo::Bump::new();
  let tokens = lex_eval(&arena, "static").unwrap();
  insta::assert_yaml_snapshot!(tokens);
}

#[test]
fn true_test() {
  let arena = bumpalo::Bump::new();
  let tokens = lex_eval(&arena, "true").unwrap();
  insta::assert_yaml_snapshot!(tokens);
}

#[test]
fn parent_test() {
  let arena = bumpalo::Bump::new();
  let tokens = lex_eval(&arena, "parent").unwrap();
  insta::assert_yaml_snapshot!(tokens);
}

#[test]
fn self_test() {
  let arena = bumpalo::Bump::new();
  let tokens = lex_eval(&arena, "self").unwrap();
  insta::assert_yaml_snapshot!(tokens);
}

#[test]
fn switch_test() {
  let arena = bumpalo::Bump::new();
  let tokens = lex_eval(&arena, "switch").unwrap();
  insta::assert_yaml_snapshot!(tokens);
}

#[test]
fn throw_test() {
  let arena = bumpalo::Bump::new();
  let tokens = lex_eval(&arena, "throw").unwrap();
  insta::assert_yaml_snapshot!(tokens);
}

#[test]
fn trait_test() {
  let arena = bumpalo::Bump::new();
  let tokens = lex_eval(&arena, "trait").unwrap();
  insta::assert_yaml_snapshot!(tokens);
}

#[test]
fn try_test() {
  let arena = bumpalo::Bump::new();
  let tokens = lex_eval(&arena, "try").unwrap();
  insta::assert_yaml_snapshot!(tokens);
}

#[test]
fn use_test() {
  let arena = bumpalo::Bump::new();
  let tokens = lex_eval(&arena, "use").unwrap();
  insta::assert_yaml_snapshot!(tokens);
}

#[test]
fn var_test() {
  let arena = bumpalo::Bump::new();
  let tokens = lex_eval(&arena, "var").unwrap();
  insta::assert_yaml_snapshot!(tokens);
}

#[test]
fn while_test() {
  let arena = bumpalo::Bump::new();
  let tokens = lex_eval(&arena, "while").unwrap();
  insta::assert_yaml_snapshot!(tokens);
}

#[test]
fn yield_test() {
  let arena = bumpalo::Bump::new();
  let tokens = lex_eval(&arena, "yield").unwrap();
  insta::assert_yaml_snapshot!(tokens);
}

#[test]
fn xor_test() {
  let arena = bumpalo::Bump::new();
  let tokens = lex_eval(&arena, "xor").unwrap();
  insta::assert_yaml_snapshot!(tokens);
}

#[test]
fn private_test() {
  let arena = bumpalo::Bump::new();
  let tokens = lex_eval(&arena, "private private(get) private(set)").unwrap();
  insta::assert_yaml_snapshot!(tokens);
}

#[test]
fn protected_test() {
  let arena = bumpalo::Bump::new();
  let tokens = lex_eval(&arena, "protected protected(get) protected(set)").unwrap();
  insta::assert_yaml_snapshot!(tokens);
}

#[test]
fn public_test() {
  let arena = bumpalo::Bump::new();
  let tokens = lex_eval(&arena, "public public(get) public(set)").unwrap();
  insta::assert_yaml_snapshot!(tokens);
}

#[test]
fn get_test() {
  let arena = bumpalo::Bump::new();
  let tokens = lex_eval(&arena, "get").unwrap();
  insta::assert_yaml_snapshot!(tokens);
}

#[test]
fn set_test() {
  let arena = bumpalo::Bump::new();
  let tokens = lex_eval(&arena, "set").unwrap();
  insta::assert_yaml_snapshot!(tokens);
}
