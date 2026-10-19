<?php

class A
{
    public function a($a = 1, $b, $c, ...$d) {}
    public function b(int $a = 1, string $b, int $c, float $d, bool $e, array $f, true $g, false $h) {}
    public function c($a = 1, ?Foo $b, Foo $c, callable $d, mixed $e, null $f, self $g, iterable $h, object $i) {}
    public function d($a = 1, Foo|Bar $b, int|string $c, Foo|null $d, Foo|string $e, A&B $f, (A&B)|C $g) {}
    public function e($a, $b = 1, $c) {}
    public function f($a, $b) {}
    public function __construct(
        private int $a = 1,
        private readonly Foo $b,
        #[Attr] $c,
        &$d,
        $e = null,
        String $f,
    ) {}
}

function g($a = [], $b) {}

$c = function ($a = 1, $b) {};
$d = fn ($a = 1, $b) => 1;
-----
<?php

class A
{
    public function a($a = 1, $b = null, $c = null, ...$d) {}
    public function b(int $a = 1, string $b = '', int $c = 0, float $d = 0.0, bool $e = false, array $f = [], true $g = true, false $h = false) {}
    public function c($a = 1, ?Foo $b = null, ?Foo $c = null, ?callable $d = null, mixed $e = null, null $f = null, ?self $g = null, ?iterable $h = null, ?object $i = null) {}
    public function d($a = 1, Foo|Bar|null $b = null, int|string $c = 0, Foo|null $d = null, Foo|string $e = '', (A&B)|null $f = null, (A&B)|C|null $g = null) {}
    public function e($a, $b = 1, $c = null) {}
    public function f($a, $b) {}
    public function __construct(
        private int $a = 1,
        private readonly ?Foo $b = null,
        #[Attr] $c = null,
        &$d = null,
        $e = null,
        String $f = '',
    ) {}
}

function g($a = [], $b = null) {}

$c = function ($a = 1, $b = null) {};
$d = fn ($a = 1, $b) => 1;
