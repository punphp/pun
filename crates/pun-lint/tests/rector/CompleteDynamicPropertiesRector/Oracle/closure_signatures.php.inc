<?php
namespace App;
class K {}
class Closures
{
    public function set()
    {
        $this->a = function (int $a, $b, K $k, string ...$rest): string { return ''; };
        $this->b = fn (?int $x = null) => $x;
        $this->c = function () use (&$z) { return new K(); };
        $this->d = static fn (int|string $u): int|string => $u;
        $this->e = function (&$r) {};
        $this->f = function () { yield 1; };
    }
}
-----
<?php
namespace App;
class K {}
class Closures
{
    /**
     * @var \Closure(int $a, mixed $b, App\K $k, string ...$rest=): string
     */
    public $a;
    /**
     * @var \Closure((int | null) $x=): (int | null)
     */
    public $b;
    /**
     * @var \Closure(): App\K
     */
    public $c;
    /**
     * @var \Closure((int | string) $u): (int | string)
     */
    public $d;
    /**
     * @var \Closure(mixed &$r): void
     */
    public $e;
    /**
     * @var \Closure(): Generator<int, int, mixed, void>
     */
    public $f;
    public function set()
    {
        $this->a = function (int $a, $b, K $k, string ...$rest): string { return ''; };
        $this->b = fn (?int $x = null) => $x;
        $this->c = function () use (&$z) { return new K(); };
        $this->d = static fn (int|string $u): int|string => $u;
        $this->e = function (&$r) {};
        $this->f = function () { yield 1; };
    }
}
