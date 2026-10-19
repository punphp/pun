<?php

namespace App;

use Countable;

final class P
{
    private int $a;
    private ?string $b;
    private int|string|null $c;
    private self $d;
    private Countable $e;
    private ?Countable $f;
    private array $g;
    private static float $h;
    private \Closure $i;
    private iterable $j;
    private mixed $k;
    private object $l;
    private Countable&\Stringable $m;
    private string|int $n;
    private bool $o;
    private null|int|string|float $p;
    private ?P $q;
    public $untyped;

    public function a() { return $this->a; }
    public function b() { return $this->b; }
    public function c() { return $this->c; }
    public function d() { return $this->d; }
    public function e() { return $this->e; }
    public function f() { return $this->f; }
    public function g() { return $this->g; }
    public function h() { return self::$h; }
    public function i() { return $this->i; }
    public function j() { return $this->j; }
    public function k() { return $this->k; }
    public function l() { return $this->l; }
    public function m() { return $this->m; }
    public function n() { return $this->n; }
    public function o() { return $this->o; }
    public function p() { return $this->p; }
    public function q() { return $this->q; }
    public function ab($x) { if ($x) { return $this->a; } return $this->b; }
    public function ae($x) { if ($x) { return $this->a; } return $this->e; }
    public function untyped() { return $this->untyped; }
}
-----
<?php

namespace App;

use Countable;

final class P
{
    private int $a;
    private ?string $b;
    private int|string|null $c;
    private self $d;
    private Countable $e;
    private ?Countable $f;
    private array $g;
    private static float $h;
    private \Closure $i;
    private iterable $j;
    private mixed $k;
    private object $l;
    private Countable&\Stringable $m;
    private string|int $n;
    private bool $o;
    private null|int|string|float $p;
    private ?P $q;
    public $untyped;

    public function a(): int { return $this->a; }
    public function b(): ?string { return $this->b; }
    public function c(): int|string|null { return $this->c; }
    public function d(): \App\P { return $this->d; }
    public function e(): \Countable { return $this->e; }
    public function f(): ?\Countable { return $this->f; }
    public function g(): array { return $this->g; }
    public function h(): float { return self::$h; }
    public function i(): \Closure { return $this->i; }
    public function j(): iterable { return $this->j; }
    public function k() { return $this->k; }
    public function l(): object { return $this->l; }
    public function m(): \Countable&\Stringable { return $this->m; }
    public function n(): string|int { return $this->n; }
    public function o(): bool { return $this->o; }
    public function p(): int|string|float|null { return $this->p; }
    public function q(): ?\App\P { return $this->q; }
    public function ab($x): int|string|null { if ($x) { return $this->a; } return $this->b; }
    public function ae($x): int|\Countable { if ($x) { return $this->a; } return $this->e; }
    public function untyped() { return $this->untyped; }
}
