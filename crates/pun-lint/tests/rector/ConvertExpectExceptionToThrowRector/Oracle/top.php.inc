<?php

declare(strict_types=1);

use Foo\Bar;

it('keeps', function () {
    expect(1)->toBe(1);
});

// a comment
$this->expectException(RuntimeException::class);
doSomething();

$x  =  1;
-----
<?php

declare(strict_types=1);

use Foo\Bar;

it('keeps', function () {
    expect(1)->toBe(1);
});
// a comment
expect(fn() => doSomething())->toThrow(RuntimeException::class);

$x  =  1;
