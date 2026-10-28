<?php

namespace App;

use Illuminate\Support\Traits\Tappable;

class T
{
    use Tappable;

    public function self_tap()
    {
        $this->tap(function ($t) {
        });
        $this->tap(function (self $t) {
        });
        $this->tap(function (T $t) {
        });
        $this->tap(function (\App\T $t) {
        });
        $this->tap(function (Other $t) {
        });
    }
}

class Other {}

function f(T $t, ?T $n, int $i, string $s, bool $b, float $fl, array $a, Other $o, int|string $u)
{
    $t->tap(function ($x) {
    });
    $t->tap(function ($x, $y) {
    });
    $t->tap(fn ($x) => 1);
    $t->tap(function () {
    });
    $t->tap(function (&$x) {
    });
    $t->tap(function (...$x) {
    });
    $t->TAP(function ($x) {
    });
    $t->tap(callback: function ($x) {
    });
    $o->tap(function ($x) {
    });
    tap($t, function ($x) {
    });
    tap($i, function ($x) {
    });
    tap(5, function ($x) {
    });
    tap($i, function (int $x) {
    });
    tap($i, function (INT $x) {
    });
    tap(5, function (int $x) {
    });
    tap($s, function ($x) {
    });
    tap('a', function (string $x) {
    });
    tap($b, function ($x) {
    });
    tap(true, function ($x) {
    });
    tap(true, function (bool $x) {
    });
    tap($fl, function ($x) {
    });
    tap($a, function ($x) {
    });
    tap([1], function (array $x) {
    });
    tap($n, function ($x) {
    });
    tap($n, function (?T $x) {
    });
    tap($n, function (T|null $x) {
    });
    tap($u, function ($x) {
    });
    tap($u, function (string|int $x) {
    });
    tap(null, function (T $x) {
    });
    tap(null, function ($x) {
    });
    tap(new T, function (Other $x) {
    });
    tap(function () {}, function ($x) {
    });
    tap($t, function ($x) {}, 3);
    tap($t);
    \tap($t, function ($x) {
    });
    TAP($t, function ($x) {
    });
    tap(callback: function ($x) {
    }, value: $t);
}
-----
<?php

namespace App;

use Illuminate\Support\Traits\Tappable;

class T
{
    use Tappable;

    public function self_tap()
    {
        $this->tap(function (self $t) {
        });
        $this->tap(function (self $t) {
        });
        $this->tap(function (T $t) {
        });
        $this->tap(function (\App\T $t) {
        });
        $this->tap(function (self $t) {
        });
    }
}

class Other {}

function f(T $t, ?T $n, int $i, string $s, bool $b, float $fl, array $a, Other $o, int|string $u)
{
    $t->tap(function (\App\T $x) {
    });
    $t->tap(function (\App\T $x, $y) {
    });
    $t->tap(fn ($x) => 1);
    $t->tap(function () {
    });
    $t->tap(function (\App\T &$x) {
    });
    $t->tap(function (\App\T ...$x) {
    });
    $t->TAP(function (\App\T $x) {
    });
    $t->tap(callback: function (\App\T $x) {
    });
    $o->tap(function ($x) {
    });
    tap($t, function (\App\T $x) {
    });
    tap($i, function (int $x) {
    });
    tap(5, function (int $x) {
    });
    tap($i, function (int $x) {
    });
    tap($i, function (INT $x) {
    });
    tap(5, function (int $x) {
    });
    tap($s, function (string $x) {
    });
    tap('a', function (string $x) {
    });
    tap($b, function (bool $x) {
    });
    tap(true, function (true $x) {
    });
    tap(true, function (bool $x) {
    });
    tap($fl, function (float $x) {
    });
    tap($a, function (array $x) {
    });
    tap([1], function (array $x) {
    });
    tap($n, function (?\App\T $x) {
    });
    tap($n, function (?T $x) {
    });
    tap($n, function (T|null $x) {
    });
    tap($u, function (int|string $x) {
    });
    tap($u, function (string|int $x) {
    });
    tap(null, function ($x) {
    });
    tap(null, function ($x) {
    });
    tap(new T, function (\App\T $x) {
    });
    tap(function () {}, function (\Closure $x) {
    });
    tap($t, function (\App\T $x) {}, 3);
    tap($t);
    \tap($t, function (\App\T $x) {
    });
    TAP($t, function (\App\T $x) {
    });
    tap(callback: function ($x) {
    }, value: $t);
}
