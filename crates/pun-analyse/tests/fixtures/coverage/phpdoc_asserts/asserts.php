<?php

namespace PhpDocAsserts;

trait SomeTrait
{
}

class Other
{
    public int $count = 0;

    public function name(): string
    {
        return '';
    }

    public function untyped()
    {
        return 1;
    }
}

class Foo
{
    /** @var string|null */
    public $prop;

    public $untypedProp;

    /**
     * @phpstan-assert int $i
     * @phpstan-assert int
     * @phpstan-assert string $o->count
     * @phpstan-assert-if-true non-empty-string $o->name()
     * @phpstan-assert-if-false int $o->missing
     * @phpstan-assert int $o->missing()
     * @phpstan-assert int $u->prop
     * @phpstan-assert array $a
     * @phpstan-assert \ArrayIterator $a2
     * @phpstan-assert SomeTrait $t
     * @phpstan-assert int $o->untyped()
     * @phpstan-assert int $this->untypedProp
     * @phpstan-assert !null $this->prop
     */
    public function check(int $i, Other $o, Other|Foo $u, $a, $a2, $t): bool
    {
        return true;
    }

    /**
     * @psalm-assert int $i
     */
    public function psalm(int $i): void
    {
    }

    /**
     * @phan-assert-if-true int $i
     */
    public function phan(int $i): bool
    {
        return true;
    }
}

/**
 * @phpstan-assert int $x
 */
function noAsserts(int $x): void
{
}

/**
 * @template T
 * @param T $t
 * @return (T is int ? int : string)
 */
function templated($t)
{
    return 1;
}

/**
 * @param int $p
 * @return (int is int ? int : string)
 */
function subjectNotTemplate($p)
{
    return 1;
}

/**
 * @param callable(mixed $v): ($v is int ? true : false) $a
 * @param callable(mixed $v): ($v is not int ? true : false) $b
 * @param callable(mixed $v): ($v is int ? bool : false) $c
 * @param \Closure(mixed $v): ($v is int ? true : bool) $d
 * @param callable(mixed $v): ($v is int ? true : true) $e
 * @param callable(mixed $v): ($v is int ? 1 : false) $f
 * @param callable(mixed $v): ($w is int ? true : false) $g
 * @param callable(mixed $v): (string is int ? true : false) $h
 * @param callable(mixed $v): ($v is int ? true : 1) $i
 */
function predicates(callable $a, callable $b, callable $c, \Closure $d, callable $e, callable $f, callable $g, callable $h, callable $i): void
{
}

/**
 * @return ($p is int ? int : string)
 */
function noParams()
{
    return 1;
}

class Base
{
    /**
     * @param int $p
     * @return ($p is int ? int : string)
     */
    public function doFoo($p)
    {
        return 1;
    }

    /**
     * @return ($missing is int ? int : string)
     */
    public function unknownParameter($p)
    {
        return 1;
    }

    /**
     * @template T
     * @param T $t
     * @return (T is int ? int : string)
     */
    public function templated($t)
    {
        return 1;
    }
}

class Child extends Base
{
    public function unknownParameter($x)
    {
        return 1;
    }

    public function templated($u)
    {
        return 1;
    }

    public function doFoo($x)
    {
        return 1;
    }

    /**
     * @return (static is self ? int : string)
     */
    public function staticSubject()
    {
        return 1;
    }
}

/**
 * @template T
 * @param array<T> $a
 * @return (array<T> is array ? int : string)
 */
function nestedTemplate(array $a)
{
    return 1;
}
