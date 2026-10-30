<?php

namespace App;

use Illuminate\Support\Collection;
use Illuminate\Support\Traits\Tappable;

/**
 * @template T
 */
class Box
{
    use Tappable;
}

class Base
{
    use Tappable;
}

class Child extends Base
{
    public function run()
    {
        $this->tap(function (parent $x) {
        });
        static::make()->tap(function (Child $x) {
        });
        tap($this, function (Base $x) {
        });
    }

    /** @return static */
    public static function make()
    {
        return new static();
    }
}

interface I {}
interface J {}

/**
 * @param Box<int> $g
 * @param I&J $ij
 * @param int|false $if
 * @param class-string $cs
 * @param non-empty-string $ne
 * @param positive-int $pi
 * @param list<int> $l
 */
function g(Box $b, $g, $ij, $if, $cs, $ne, $pi, $l, Collection $c, iterable $it, callable $cb, object $ob)
{
    $b->tap(function ($x) {
    });
    tap($ij, function ($x) {
    });
    tap($if, function ($x) {
    });
    tap($cs, function ($x) {
    });
    tap($cs, function (string $x) {
    });
    tap($ne, function (string $x) {
    });
    tap($pi, function (int $x) {
    });
    tap($pi, function (INT $x) {
    });
    tap($l, function (array $x) {
    });
    tap($c, function ($x) {
    });
    tap($it, function ($x) {
    });
    tap($cb, function ($x) {
    });
    tap($ob, function (object $x) {
    });
    tap(1.5, function (float $x) {
    });
    tap(1.5, function (FLOAT $x) {
    });
    tap(1.5, function ($x) {
    });
    tap(rand(0, 1) ? 1 : 'a', function ($x) {
    });
    tap(rand(0, 1) ? new Base : null, function (Base $x) {
    });
}
-----
<?php

namespace App;

use Illuminate\Support\Collection;
use Illuminate\Support\Traits\Tappable;

/**
 * @template T
 */
class Box
{
    use Tappable;
}

class Base
{
    use Tappable;
}

class Child extends Base
{
    public function run()
    {
        $this->tap(function (self $x) {
        });
        static::make()->tap(function (Child $x) {
        });
        tap($this, function (self $x) {
        });
    }

    /** @return static */
    public static function make()
    {
        return new static();
    }
}

interface I {}
interface J {}

/**
 * @param Box<int> $g
 * @param I&J $ij
 * @param int|false $if
 * @param class-string $cs
 * @param non-empty-string $ne
 * @param positive-int $pi
 * @param list<int> $l
 */
function g(Box $b, $g, $ij, $if, $cs, $ne, $pi, $l, Collection $c, iterable $it, callable $cb, object $ob)
{
    $b->tap(function (\App\Box $x) {
    });
    tap($ij, function (\App\I&\App\J $x) {
    });
    tap($if, function (int|false $x) {
    });
    tap($cs, function (string $x) {
    });
    tap($cs, function (string $x) {
    });
    tap($ne, function (string $x) {
    });
    tap($pi, function (int $x) {
    });
    tap($pi, function (int $x) {
    });
    tap($l, function (array $x) {
    });
    tap($c, function (\Illuminate\Support\Collection $x) {
    });
    tap($it, function (iterable $x) {
    });
    tap($cb, function (callable $x) {
    });
    tap($ob, function (object $x) {
    });
    tap(1.5, function (float $x) {
    });
    tap(1.5, function (FLOAT $x) {
    });
    tap(1.5, function (float $x) {
    });
    tap(rand(0, 1) ? 1 : 'a', function (int|string $x) {
    });
    tap(rand(0, 1) ? new Base : null, function (?\App\Base $x) {
    });
}
