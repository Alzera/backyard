use backyard_lexer::lex;
use backyard_parser::parse_base;
use bumpalo::Bump;

const CONTENT: &str =
  "<?php
declare(strict_types = 1);
require_once 'foo.php';

namespace sample;

use foo\\ {
  baz,
  bar as fooBar
};

global $b;
const FOOBAR = 'BARFOO';

trait foo {}

interface bar {}

/* comment block */
enum foo {
  case bar;
}

/**
 * comment doc
 */
class A {
  use foo, bar {
    foo::baz insteadof bar;
    bar::foo as baz;
  }

  const FOOBAR = 'BARFOO';
  protected (A&B)|null $foo = null;

  function __construct(public $a){
    list($a, $b) = match ($a) {
      __DIR__ => clone 2,
      default => (int) 3,
    };
    if(Foo > 5) {
      do {
        ?>Test<?php
      } while (false);
    } else {
      echo (function() {

      })(fn() => 5);
    }
  }

  // comment line
  #[attr]
  private function test(int $x, string &...$y = 0) {
    LABEL:
    try {
      while (false) {
        yield @$this->a(...$a) ? !static::class : (self[0]++)(a: 1);
        yield from ++$a;
        return new class {};
      }
      throw new \\ComeToHome(<<<'BAR'
  NowDoc
BAR);
      goto LABEL;
      eval(\"return \" . \"'This is $a!';\");
      exit;
    } catch (Exception $e) {
      switch (parent) {
        case 1:
          foreach ($a as $b => $c) {
          }
          break;
        case 2:
          for($i = 0; $i < 5; $i++) {
          }
          continue;
        default:
          break;
      }
    } finally {
      print <<<BAR
  HereDoc
BAR;
    }
  }
}";

fn main() {
  let lexer_arena = Bump::new();
  let tokens = lex(&lexer_arena, CONTENT);
  for _ in 0..100 {
    let arena = Bump::new();
    let _ = parse_base(&arena, &tokens);
  }
}
