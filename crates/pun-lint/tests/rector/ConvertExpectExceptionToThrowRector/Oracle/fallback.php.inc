<?php

test('c', static   function()use($a, &$b):void{
    $this->expectExceptionObject(new RuntimeException('x'));
    $a->run();
});

test('p', function (int $x = 1,  ...$rest) : ?Foo {
    // lead
    $this->expectException(RuntimeException::class);
    $a->run([
        'a' => 1,
    ]);
});

test('q', #[Attr]   fn  () => 1);

it('r', function (
    // c
    string $s,
) {
    $this->expectException(E::class);
    run();
});

  it('s', function () {
      $this->expectException(E::class);
      run(function () {
          return 1;
      });
  });
-----
<?php

test('c', static function () use ($a, &$b): void {
    expect(fn() => $a->run())->toThrow(new RuntimeException('x'));
});

test('p', function (int $x = 1, ...$rest): ?Foo {
    // lead
    expect(fn() => $a->run([
        'a' => 1,
    ]))->toThrow(RuntimeException::class);
});

test('q', #[Attr]   fn  () => 1);

it('r', function (
    // c
    string $s
) {
    expect(fn() => run())->toThrow(E::class);
});

  it('s', function () {
      expect(fn() => run(function () {
          return 1;
      }))->toThrow(E::class);
  });
