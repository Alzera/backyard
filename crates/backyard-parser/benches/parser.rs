use std::hint::black_box;
use backyard_parser::parse;
use criterion::{ criterion_group, criterion_main, Criterion };

const CONTENT: &str =
  "<?php

declare(strict_types=1);
include_once 'foo.php';

/** a comment before the namespace comment **/

namespace foo\\bar {
?>Hello<?php
  # single line comment
  use function foo\\{
    // sample code
    baz,
    // with alias
    bar as fooBar
  };
  use bar as nsBar;

  // generic constant
  const FOOBAR = 'foo & bar';

  /**
   * a class
   */
  abstract class fooBar implements namespace\\fooBaz
  {
    use Line;
    use foo, bar {
      foo::baz insteadof bar;
      bar::foo as baz;
    }
    const FOOBAR = 'BARFOO';
    protected $dwarf = [
      'sneezy' => 'achoum',
      'bashful' => 'tadah'
    ];
    /**
     * Something is done here
     */
    final public function doSomething()
    {
      // do not wanna do
      foreach ($this->dwarf as $name => $greeting) {
        echo \"Hey ho $name, $greeting !\";
        continue $foo;
      }
      throw new \\ComeToHome('Thats it');
    }
  }

  interface Miror extends Object
  {
    public function Am_I_Uggly(): bool;
    function broken(): bool;
    static function isWhiteSnowAlive(): bool;
  }

  function iter()
  {
    yield 'ator' => $foo;
    yield from iter(50);
  }

  trait Line
  {
    public function draw(bool $arrow = false): string
    {
      switch ($this->style) {
        case 'dot':
        case 'point':
          $body = '......';
          break;
        default:
          $body = '-----';
          break;
      }
      return $body . ($arrow ? '>' : '');
    }
    private function shuut()
    {
      return __NAMESPACE__;
    }
  }

  // this is SPARTA !
  function sparta(): ?int
  {
    global $persians;

    $index = -1;
    next:
    $guy = $persians[++$index];
    if (rand(0, 10) > 5 || false) {
      @$guy->kill();
    } else {
      $guy->kick(...$foo);
    }
    if ((int)$index < count($persians)) goto next;

    return 300 | 5;
  }

  $foo = function (?int $bar = 42) use ($something): bool {
    static $banana;
    if ($bar > fooBar::NUMBAR) {
      while ($bar) {
        if ((int)calculateMeaningOfLife() === 42) {
          break foo;
        } else continue;
      }
      do {
  ?>
Caesar: here I was
<?php
      } while (false);
    } else if (!($something instanceof Banana)) {
      try {
        $bipbip = clone $something;
        $bipbip->crazy()->boom([1, 2, 3]);
      } catch (Coco | Nut $ex) {
        $ex->printStackTrace();
      } finally {
        if (isset($bipbip, $ex)) unset($bipbip, $ex);
        return (new class extends fooBar {
          function goatIt()
          {
            return \"meeeh\";
          }
        })->goatIt();
      }
    } else {
      for ($i = 0; $i < count($this->banana); $i++) {
        $x %= ($i * 2) / ($i - 1);
        $what = $this->$x[++$i] ? 'yes!' : 'noo!';
      }
      // @todo $this->a_$foo
      return $$foo ?? false;
    }
    return empty(namespace\\FOOBAR);
  };

  if ($foo):
    echo `bar&`;
  elseif ($bar):
    echo `ls -larth`;
  endif;

  // list version
  list($a, list($b, $c)) = [1, [2, 3]];
  print(<<<BAZ
Hello world
BAZ
  );
  die($foo(
    <<<'BAR'
$foo + $bar
BAR
  ));
  eval(<<<FOO
return 'This is madness!';
FOO);

  // nested blocks
  { { {
        echo \"Hello world\";
      }
    }
  }
}";

fn criterion_benchmark(c: &mut Criterion) {
  c.bench_function("basic", |b| {
    b.iter(|| {
      let _ = parse(black_box(&CONTENT));
    });
  });
}

criterion_group! {
  name = benches;
  config = Criterion::default()
    .warm_up_time(std::time::Duration::from_secs(5))
    .measurement_time(std::time::Duration::from_secs(50))
    .sample_size(500);
  targets = criterion_benchmark,
}
criterion_main!(benches);
