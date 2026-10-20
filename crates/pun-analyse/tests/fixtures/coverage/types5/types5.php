<?php

namespace Types5;

use Closure;

/**
 * @template A of \DateTime
 * @param A $a
 * @return A
 */
function narrow($a)
{
    throw new \LogicException();
}

/**
 * @template T
 * @param array<T> $a
 * @return T
 */
function first(array $a)
{
    throw new \LogicException();
}

/**
 * @template T
 * @param T|null $a
 * @return T|int
 */
function orNull($a)
{
    return 1;
}

/**
 * @template T
 * @param T $a
 * @return T
 */
function withDefault($a = 1)
{
    throw new \LogicException();
}

/**
 * @template T
 * @param T ...$a
 * @return list<T>
 */
function variadic(...$a)
{
    throw new \LogicException();
}

/**
 * @template T of string
 * @param mixed $a
 * @return ($a is T ? T : int)
 */
function conditional($a)
{
    return 1;
}

/**
 * @template T
 * @param T|null $a
 * @return T|null
 */
function nested($a = null)
{
    throw new \LogicException();
}

/** @template T */
class Box
{
    /** @param T $value */
    public function __construct(public $value)
    {
    }

    /**
     * @param-closure-this \stdClass $c
     */
    public function bind(Closure $c, Closure $d, Closure $e): void
    {
        $_ = Closure::bind($c, new \stdClass());
        $_ = Closure::bind($c, $this);
        $_ = $c->bindTo($this);
        $cx = 1;
        $e = static fn () => $cx;
        $_ = Closure::bind($e, $this);
        $_ = Closure::bind($d, $this);
        $_ = $d->bindTo($this);
    }
}

/**
 * @param-closure-this \stdClass $c
 */
function bindIn(Closure $c, int $n): void
{
    $c = fn () => $n;
    $_ = Closure::bind($c, new Box(1));
    $_ = $c->bindTo(new Box(1));
}

function never(): void
{
    $_ = first([]);
}

function calls(int $option, array $options): void
{
    $_ = narrow(new \DateTimeImmutable());
    $_ = orNull(null);
    $_ = withDefault();
    $_ = variadic();
    $_ = conditional('x');
    $_ = conditional(1);
    $_ = nested();
    $_ = new Box(1);

    $ch = curl_init('https://example.com');
    if ($ch === false) {
        return;
    }
    curl_setopt($ch, CURLOPT_RETURNTRANSFER, 'yes');
    curl_setopt($ch, CURLOPT_RETURNTRANSFER, true);
    curl_setopt($ch, $option, 'x');
    curl_setopt($ch, 1, 'x');
    curl_setopt_array($ch, [CURLOPT_TIMEOUT => 'slow']);
    curl_setopt_array($ch, [1 => 'x']);
    curl_setopt_array($ch, ['a' => 'x']);
    curl_setopt_array($ch, $options);

    $f = fn () => 1;
    $_ = Closure::bind($f, new Box(1));
}

$_ = Closure::bind(fn () => 1, null);
$g = fn () => 1;
$_ = Closure::bind($g, new Box(1));
