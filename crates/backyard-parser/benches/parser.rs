use std::hint::black_box;
use backyard_parser::parse;
use criterion::{ criterion_group, criterion_main, Criterion };

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

fn criterion_benchmark(c: &mut Criterion) {
  c.bench_function("parser_basic", |b| {
    b.iter(|| {
      let _ = parse(black_box(&CONTENT));
    });
  });
}

criterion_group! {
  name = benches;
  config = Criterion::default()
    .warm_up_time(std::time::Duration::from_secs(5))
    .measurement_time(std::time::Duration::from_secs(60))
    .sample_size(500);
  targets = criterion_benchmark,
}
criterion_main!(benches);
