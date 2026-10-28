<?php

namespace App;

use Generator;

final class YieldProbe
{
    public function a()
    {
        yield 1;
        yield 2;
    }

    public function b(): array
    {
        yield 1;
    }

    public function c(): ?iterable
    {
        yield 1;
    }

    public function d(): Generator
    {
        yield 1;
    }

    public function e()
    {
        foreach ([1] as $x) {
            yield $x;
        }
    }

    public function f()
    {
        $x = yield 1;
        // trailing
    }

    public function g()
    {
        yield from [1];
    }

    public function h(): \Traversable
    {
        yield 1;
    }

    public function i(): YieldProbe
    {
        yield 1;
    }
}

function gen()
{
    yield 1;
}
-----
<?php

namespace App;

use Generator;

final class YieldProbe
{
    public function a(): \Generator
    {
        yield 1;
        yield 2;
    }

    public function b(): \Generator
    {
        yield 1;
    }

    public function c(): \Generator
    {
        yield 1;
    }

    public function d(): Generator
    {
        yield 1;
    }

    public function e()
    {
        foreach ([1] as $x) {
            yield $x;
        }
    }

    public function f()
    {
        $x = yield 1;
        // trailing
    }

    public function g(): \Generator
    {
        yield from [1];
    }

    public function h(): \Traversable
    {
        yield 1;
    }

    public function i(): \App\YieldProbe
    {
        yield 1;
    }
}

function gen(): \Generator
{
    yield 1;
}
