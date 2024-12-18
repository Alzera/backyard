use backyard_lexer::lex;

#[test]
fn abstract_test() {
  let tokens = lex(true, "abstract").unwrap();
  insta::assert_yaml_snapshot!(tokens);
}

#[test]
fn array_test() {
  let tokens = lex(true, "array").unwrap();
  insta::assert_yaml_snapshot!(tokens);
}

#[test]
fn as_test() {
  let tokens = lex(true, "as").unwrap();
  insta::assert_yaml_snapshot!(tokens);
}

#[test]
fn break_test() {
  let tokens = lex(true, "break").unwrap();
  insta::assert_yaml_snapshot!(tokens);
}

#[test]
fn callable_test() {
  let tokens = lex(true, "callable").unwrap();
  insta::assert_yaml_snapshot!(tokens);
}

#[test]
fn case_test() {
  let tokens = lex(true, "case").unwrap();
  insta::assert_yaml_snapshot!(tokens);
}

#[test]
fn catch_test() {
  let tokens = lex(true, "catch").unwrap();
  insta::assert_yaml_snapshot!(tokens);
}

#[test]
fn class_test() {
  let tokens = lex(true, "class").unwrap();
  insta::assert_yaml_snapshot!(tokens);
}

#[test]
fn clone_test() {
  let tokens = lex(true, "clone").unwrap();
  insta::assert_yaml_snapshot!(tokens);
}

#[test]
fn const_test() {
  let tokens = lex(true, "const").unwrap();
  insta::assert_yaml_snapshot!(tokens);
}

#[test]
fn continue_test() {
  let tokens = lex(true, "continue").unwrap();
  insta::assert_yaml_snapshot!(tokens);
}

#[test]
fn declare_test() {
  let tokens = lex(true, "declare").unwrap();
  insta::assert_yaml_snapshot!(tokens);
}

#[test]
fn default_test() {
  let tokens = lex(true, "default").unwrap();
  insta::assert_yaml_snapshot!(tokens);
}

#[test]
fn do_test() {
  let tokens = lex(true, "do").unwrap();
  insta::assert_yaml_snapshot!(tokens);
}

#[test]
fn echo_test() {
  let tokens = lex(true, "echo").unwrap();
  insta::assert_yaml_snapshot!(tokens);
}

#[test]
fn else_test() {
  let tokens = lex(true, "else").unwrap();
  insta::assert_yaml_snapshot!(tokens);
}

#[test]
fn elseif_test() {
  let tokens = lex(true, "elseif").unwrap();
  insta::assert_yaml_snapshot!(tokens);
}

#[test]
fn enddeclare_test() {
  let tokens = lex(true, "enddeclare").unwrap();
  insta::assert_yaml_snapshot!(tokens);
}

#[test]
fn endfor_test() {
  let tokens = lex(true, "endfor").unwrap();
  insta::assert_yaml_snapshot!(tokens);
}

#[test]
fn endforeach_test() {
  let tokens = lex(true, "endforeach").unwrap();
  insta::assert_yaml_snapshot!(tokens);
}

#[test]
fn endif_test() {
  let tokens = lex(true, "endif").unwrap();
  insta::assert_yaml_snapshot!(tokens);
}

#[test]
fn endswitch_test() {
  let tokens = lex(true, "endswitch").unwrap();
  insta::assert_yaml_snapshot!(tokens);
}

#[test]
fn endwhile_test() {
  let tokens = lex(true, "endwhile").unwrap();
  insta::assert_yaml_snapshot!(tokens);
}

#[test]
fn enum_test() {
  let tokens = lex(true, "enum").unwrap();
  insta::assert_yaml_snapshot!(tokens);
}

#[test]
fn exit_test() {
  let tokens = lex(true, "exit").unwrap();
  insta::assert_yaml_snapshot!(tokens);
}

#[test]
fn eval_test() {
  let tokens = lex(true, "eval").unwrap();
  insta::assert_yaml_snapshot!(tokens);
}

#[test]
fn die_test() {
  let tokens = lex(true, "die").unwrap();
  insta::assert_yaml_snapshot!(tokens);
}

#[test]
fn extends_test() {
  let tokens = lex(true, "extends").unwrap();
  insta::assert_yaml_snapshot!(tokens);
}

#[test]
fn false_test() {
  let tokens = lex(true, "false").unwrap();
  insta::assert_yaml_snapshot!(tokens);
}

#[test]
fn final_test() {
  let tokens = lex(true, "final").unwrap();
  insta::assert_yaml_snapshot!(tokens);
}

#[test]
fn finally_test() {
  let tokens = lex(true, "finally").unwrap();
  insta::assert_yaml_snapshot!(tokens);
}

#[test]
fn fn_test() {
  let tokens = lex(true, "fn").unwrap();
  insta::assert_yaml_snapshot!(tokens);
}

#[test]
fn for_test() {
  let tokens = lex(true, "for").unwrap();
  insta::assert_yaml_snapshot!(tokens);
}

#[test]
fn foreach_test() {
  let tokens = lex(true, "foreach").unwrap();
  insta::assert_yaml_snapshot!(tokens);
}

#[test]
fn from_test() {
  let tokens = lex(true, "from").unwrap();
  insta::assert_yaml_snapshot!(tokens);
}

#[test]
fn function_test() {
  let tokens = lex(true, "function").unwrap();
  insta::assert_yaml_snapshot!(tokens);
}

#[test]
fn global_test() {
  let tokens = lex(true, "global").unwrap();
  insta::assert_yaml_snapshot!(tokens);
}

#[test]
fn goto_test() {
  let tokens = lex(true, "goto").unwrap();
  insta::assert_yaml_snapshot!(tokens);
}

#[test]
fn if_test() {
  let tokens = lex(true, "if").unwrap();
  insta::assert_yaml_snapshot!(tokens);
}

#[test]
fn implements_test() {
  let tokens = lex(true, "implements").unwrap();
  insta::assert_yaml_snapshot!(tokens);
}

#[test]
fn include_test() {
  let tokens = lex(true, "include").unwrap();
  insta::assert_yaml_snapshot!(tokens);
}

#[test]
fn include_once_test() {
  let tokens = lex(true, "include_once").unwrap();
  insta::assert_yaml_snapshot!(tokens);
}

#[test]
fn instanceof_test() {
  let tokens = lex(true, "instanceof").unwrap();
  insta::assert_yaml_snapshot!(tokens);
}

#[test]
fn insteadof_test() {
  let tokens = lex(true, "insteadof").unwrap();
  insta::assert_yaml_snapshot!(tokens);
}

#[test]
fn interface_test() {
  let tokens = lex(true, "interface").unwrap();
  insta::assert_yaml_snapshot!(tokens);
}

#[test]
fn list_test() {
  let tokens = lex(true, "list").unwrap();
  insta::assert_yaml_snapshot!(tokens);
}

#[test]
fn and_test() {
  let tokens = lex(true, "and").unwrap();
  insta::assert_yaml_snapshot!(tokens);
}

#[test]
fn or_test() {
  let tokens = lex(true, "or").unwrap();
  insta::assert_yaml_snapshot!(tokens);
}

#[test]
fn match_test() {
  let tokens = lex(true, "match").unwrap();
  insta::assert_yaml_snapshot!(tokens);
}

#[test]
fn namespace_test() {
  let tokens = lex(true, "namespace").unwrap();
  insta::assert_yaml_snapshot!(tokens);
}

#[test]
fn new_test() {
  let tokens = lex(true, "new").unwrap();
  insta::assert_yaml_snapshot!(tokens);
}

#[test]
fn null_test() {
  let tokens = lex(true, "null").unwrap();
  insta::assert_yaml_snapshot!(tokens);
}

#[test]
fn print_test() {
  let tokens = lex(true, "print").unwrap();
  insta::assert_yaml_snapshot!(tokens);
}

#[test]
fn readonly_test() {
  let tokens = lex(true, "readonly").unwrap();
  insta::assert_yaml_snapshot!(tokens);
}

#[test]
fn require_test() {
  let tokens = lex(true, "require").unwrap();
  insta::assert_yaml_snapshot!(tokens);
}

#[test]
fn require_once_test() {
  let tokens = lex(true, "require_once").unwrap();
  insta::assert_yaml_snapshot!(tokens);
}

#[test]
fn return_test() {
  let tokens = lex(true, "return").unwrap();
  insta::assert_yaml_snapshot!(tokens);
}

#[test]
fn static_test() {
  let tokens = lex(true, "static").unwrap();
  insta::assert_yaml_snapshot!(tokens);
}

#[test]
fn true_test() {
  let tokens = lex(true, "true").unwrap();
  insta::assert_yaml_snapshot!(tokens);
}

#[test]
fn parent_test() {
  let tokens = lex(true, "parent").unwrap();
  insta::assert_yaml_snapshot!(tokens);
}

#[test]
fn self_test() {
  let tokens = lex(true, "self").unwrap();
  insta::assert_yaml_snapshot!(tokens);
}

#[test]
fn switch_test() {
  let tokens = lex(true, "switch").unwrap();
  insta::assert_yaml_snapshot!(tokens);
}

#[test]
fn throw_test() {
  let tokens = lex(true, "throw").unwrap();
  insta::assert_yaml_snapshot!(tokens);
}

#[test]
fn trait_test() {
  let tokens = lex(true, "trait").unwrap();
  insta::assert_yaml_snapshot!(tokens);
}

#[test]
fn try_test() {
  let tokens = lex(true, "try").unwrap();
  insta::assert_yaml_snapshot!(tokens);
}

#[test]
fn use_test() {
  let tokens = lex(true, "use").unwrap();
  insta::assert_yaml_snapshot!(tokens);
}

#[test]
fn var_test() {
  let tokens = lex(true, "var").unwrap();
  insta::assert_yaml_snapshot!(tokens);
}

#[test]
fn while_test() {
  let tokens = lex(true, "while").unwrap();
  insta::assert_yaml_snapshot!(tokens);
}

#[test]
fn yield_test() {
  let tokens = lex(true, "yield").unwrap();
  insta::assert_yaml_snapshot!(tokens);
}

#[test]
fn xor_test() {
  let tokens = lex(true, "xor").unwrap();
  insta::assert_yaml_snapshot!(tokens);
}

#[test]
fn private_test() {
  let tokens = lex(true, "private private(get) private(set)").unwrap();
  insta::assert_yaml_snapshot!(tokens);
}

#[test]
fn protected_test() {
  let tokens = lex(true, "protected protected(get) protected(set)").unwrap();
  insta::assert_yaml_snapshot!(tokens);
}

#[test]
fn public_test() {
  let tokens = lex(true, "public public(get) public(set)").unwrap();
  insta::assert_yaml_snapshot!(tokens);
}

#[test]
fn get_test() {
  let tokens = lex(true, "get").unwrap();
  insta::assert_yaml_snapshot!(tokens);
}

#[test]
fn set_test() {
  let tokens = lex(true, "set").unwrap();
  insta::assert_yaml_snapshot!(tokens);
}
