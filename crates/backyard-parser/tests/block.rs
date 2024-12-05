use backyard_parser::parse_eval;

#[test]
fn basic() {
  let asts = parse_eval("{\n}").unwrap();
  insta::assert_yaml_snapshot!(asts);
}

#[test]
fn nested() {
  let asts = parse_eval("{\n\t{\n\t}\n}").unwrap();
  insta::assert_yaml_snapshot!(asts);
}

#[test]
fn test() {
  let asts = parse_eval(
    "#[Entity]
class DoctrineWithEmbedded
{
    #[Id, Column(type: 'smallint')]
    public $id;

    #[Embedded(class: DoctrineEmbeddable::class)]
    protected $embedded;
}
"
  );
  println!("{:?}", asts);
}
