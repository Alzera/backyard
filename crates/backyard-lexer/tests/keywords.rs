use backyard_lexer::lex_eval;

#[test]
fn abstract_test() {
  let tokens = lex_eval("abstract").unwrap();
  insta::assert_yaml_snapshot!(tokens);
}

#[test]
fn array_test() {
  let tokens = lex_eval("array").unwrap();
  insta::assert_yaml_snapshot!(tokens);
}

#[test]
fn as_test() {
  let tokens = lex_eval("as").unwrap();
  insta::assert_yaml_snapshot!(tokens);
}

#[test]
fn break_test() {
  let tokens = lex_eval("break").unwrap();
  insta::assert_yaml_snapshot!(tokens);
}

#[test]
fn callable_test() {
  let tokens = lex_eval("callable").unwrap();
  insta::assert_yaml_snapshot!(tokens);
}

#[test]
fn case_test() {
  let tokens = lex_eval("case").unwrap();
  insta::assert_yaml_snapshot!(tokens);
}

#[test]
fn catch_test() {
  let tokens = lex_eval("catch").unwrap();
  insta::assert_yaml_snapshot!(tokens);
}

#[test]
fn class_test() {
  let tokens = lex_eval("class").unwrap();
  insta::assert_yaml_snapshot!(tokens);
}

#[test]
fn clone_test() {
  let tokens = lex_eval("clone").unwrap();
  insta::assert_yaml_snapshot!(tokens);
}

#[test]
fn const_test() {
  let tokens = lex_eval("const").unwrap();
  insta::assert_yaml_snapshot!(tokens);
}

#[test]
fn continue_test() {
  let tokens = lex_eval("continue").unwrap();
  insta::assert_yaml_snapshot!(tokens);
}

#[test]
fn declare_test() {
  let tokens = lex_eval("declare").unwrap();
  insta::assert_yaml_snapshot!(tokens);
}

#[test]
fn default_test() {
  let tokens = lex_eval("default").unwrap();
  insta::assert_yaml_snapshot!(tokens);
}

#[test]
fn do_test() {
  let tokens = lex_eval("do").unwrap();
  insta::assert_yaml_snapshot!(tokens);
}

#[test]
fn echo_test() {
  let tokens = lex_eval("echo").unwrap();
  insta::assert_yaml_snapshot!(tokens);
}

#[test]
fn else_test() {
  let tokens = lex_eval("else").unwrap();
  insta::assert_yaml_snapshot!(tokens);
}

#[test]
fn elseif_test() {
  let tokens = lex_eval("elseif").unwrap();
  insta::assert_yaml_snapshot!(tokens);
}

#[test]
fn enddeclare_test() {
  let tokens = lex_eval("enddeclare").unwrap();
  insta::assert_yaml_snapshot!(tokens);
}

#[test]
fn endfor_test() {
  let tokens = lex_eval("endfor").unwrap();
  insta::assert_yaml_snapshot!(tokens);
}

#[test]
fn endforeach_test() {
  let tokens = lex_eval("endforeach").unwrap();
  insta::assert_yaml_snapshot!(tokens);
}

#[test]
fn endif_test() {
  let tokens = lex_eval("endif").unwrap();
  insta::assert_yaml_snapshot!(tokens);
}

#[test]
fn endswitch_test() {
  let tokens = lex_eval("endswitch").unwrap();
  insta::assert_yaml_snapshot!(tokens);
}

#[test]
fn endwhile_test() {
  let tokens = lex_eval("endwhile").unwrap();
  insta::assert_yaml_snapshot!(tokens);
}

#[test]
fn enum_test() {
  let tokens = lex_eval("enum").unwrap();
  insta::assert_yaml_snapshot!(tokens);
}

#[test]
fn exit_test() {
  let tokens = lex_eval("exit").unwrap();
  insta::assert_yaml_snapshot!(tokens);
}

#[test]
fn eval_test() {
  let tokens = lex_eval("eval").unwrap();
  insta::assert_yaml_snapshot!(tokens);
}

#[test]
fn die_test() {
  let tokens = lex_eval("die").unwrap();
  insta::assert_yaml_snapshot!(tokens);
}

#[test]
fn extends_test() {
  let tokens = lex_eval("extends").unwrap();
  insta::assert_yaml_snapshot!(tokens);
}

#[test]
fn false_test() {
  let tokens = lex_eval("false").unwrap();
  insta::assert_yaml_snapshot!(tokens);
}

#[test]
fn final_test() {
  let tokens = lex_eval("final").unwrap();
  insta::assert_yaml_snapshot!(tokens);
}

#[test]
fn finally_test() {
  let tokens = lex_eval("finally").unwrap();
  insta::assert_yaml_snapshot!(tokens);
}

#[test]
fn fn_test() {
  let tokens = lex_eval("fn").unwrap();
  insta::assert_yaml_snapshot!(tokens);
}

#[test]
fn for_test() {
  let tokens = lex_eval("for").unwrap();
  insta::assert_yaml_snapshot!(tokens);
}

#[test]
fn foreach_test() {
  let tokens = lex_eval("foreach").unwrap();
  insta::assert_yaml_snapshot!(tokens);
}

#[test]
fn from_test() {
  let tokens = lex_eval("from").unwrap();
  insta::assert_yaml_snapshot!(tokens);
}

#[test]
fn function_test() {
  let tokens = lex_eval("function").unwrap();
  insta::assert_yaml_snapshot!(tokens);
}

#[test]
fn global_test() {
  let tokens = lex_eval("global").unwrap();
  insta::assert_yaml_snapshot!(tokens);
}

#[test]
fn goto_test() {
  let tokens = lex_eval("goto").unwrap();
  insta::assert_yaml_snapshot!(tokens);
}

#[test]
fn if_test() {
  let tokens = lex_eval("if").unwrap();
  insta::assert_yaml_snapshot!(tokens);
}

#[test]
fn implements_test() {
  let tokens = lex_eval("implements").unwrap();
  insta::assert_yaml_snapshot!(tokens);
}

#[test]
fn include_test() {
  let tokens = lex_eval("include").unwrap();
  insta::assert_yaml_snapshot!(tokens);
}

#[test]
fn include_once_test() {
  let tokens = lex_eval("include_once").unwrap();
  insta::assert_yaml_snapshot!(tokens);
}

#[test]
fn instanceof_test() {
  let tokens = lex_eval("instanceof").unwrap();
  insta::assert_yaml_snapshot!(tokens);
}

#[test]
fn insteadof_test() {
  let tokens = lex_eval("insteadof").unwrap();
  insta::assert_yaml_snapshot!(tokens);
}

#[test]
fn interface_test() {
  let tokens = lex_eval("interface").unwrap();
  insta::assert_yaml_snapshot!(tokens);
}

#[test]
fn list_test() {
  let tokens = lex_eval("list").unwrap();
  insta::assert_yaml_snapshot!(tokens);
}

#[test]
fn and_test() {
  let tokens = lex_eval("and").unwrap();
  insta::assert_yaml_snapshot!(tokens);
}

#[test]
fn or_test() {
  let tokens = lex_eval("or").unwrap();
  insta::assert_yaml_snapshot!(tokens);
}

#[test]
fn match_test() {
  let tokens = lex_eval("match").unwrap();
  insta::assert_yaml_snapshot!(tokens);
}

#[test]
fn namespace_test() {
  let tokens = lex_eval("namespace").unwrap();
  insta::assert_yaml_snapshot!(tokens);
}

#[test]
fn new_test() {
  let tokens = lex_eval("new").unwrap();
  insta::assert_yaml_snapshot!(tokens);
}

#[test]
fn null_test() {
  let tokens = lex_eval("null").unwrap();
  insta::assert_yaml_snapshot!(tokens);
}

#[test]
fn print_test() {
  let tokens = lex_eval("print").unwrap();
  insta::assert_yaml_snapshot!(tokens);
}

#[test]
fn readonly_test() {
  let tokens = lex_eval("readonly").unwrap();
  insta::assert_yaml_snapshot!(tokens);
}

#[test]
fn require_test() {
  let tokens = lex_eval("require").unwrap();
  insta::assert_yaml_snapshot!(tokens);
}

#[test]
fn require_once_test() {
  let tokens = lex_eval("require_once").unwrap();
  insta::assert_yaml_snapshot!(tokens);
}

#[test]
fn return_test() {
  let tokens = lex_eval("return").unwrap();
  insta::assert_yaml_snapshot!(tokens);
}

#[test]
fn static_test() {
  let tokens = lex_eval("static").unwrap();
  insta::assert_yaml_snapshot!(tokens);
}

#[test]
fn true_test() {
  let tokens = lex_eval("true").unwrap();
  insta::assert_yaml_snapshot!(tokens);
}

#[test]
fn parent_test() {
  let tokens = lex_eval("parent").unwrap();
  insta::assert_yaml_snapshot!(tokens);
}

#[test]
fn self_test() {
  let tokens = lex_eval("self").unwrap();
  insta::assert_yaml_snapshot!(tokens);
}

#[test]
fn switch_test() {
  let tokens = lex_eval("switch").unwrap();
  insta::assert_yaml_snapshot!(tokens);
}

#[test]
fn throw_test() {
  let tokens = lex_eval("throw").unwrap();
  insta::assert_yaml_snapshot!(tokens);
}

#[test]
fn trait_test() {
  let tokens = lex_eval("trait").unwrap();
  insta::assert_yaml_snapshot!(tokens);
}

#[test]
fn try_test() {
  let tokens = lex_eval("try").unwrap();
  insta::assert_yaml_snapshot!(tokens);
}

#[test]
fn use_test() {
  let tokens = lex_eval("use").unwrap();
  insta::assert_yaml_snapshot!(tokens);
}

#[test]
fn var_test() {
  let tokens = lex_eval("var").unwrap();
  insta::assert_yaml_snapshot!(tokens);
}

#[test]
fn while_test() {
  let tokens = lex_eval("while").unwrap();
  insta::assert_yaml_snapshot!(tokens);
}

#[test]
fn yield_test() {
  let tokens = lex_eval("yield").unwrap();
  insta::assert_yaml_snapshot!(tokens);
}

#[test]
fn xor_test() {
  let tokens = lex_eval("xor").unwrap();
  insta::assert_yaml_snapshot!(tokens);
}

#[test]
fn private_test() {
  let tokens = lex_eval("private private(get) private(set)").unwrap();
  insta::assert_yaml_snapshot!(tokens);
}

#[test]
fn protected_test() {
  let tokens = lex_eval("protected protected(get) protected(set)").unwrap();
  insta::assert_yaml_snapshot!(tokens);
}

#[test]
fn public_test() {
  let tokens = lex_eval("public public(get) public(set)").unwrap();
  insta::assert_yaml_snapshot!(tokens);
}
