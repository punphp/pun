<?php

namespace CoverageLevel4;

class MyException extends \Exception
{
}

class Thrower
{
    /** @throws MyException */
    public function __construct()
    {
    }

    /** @throws MyException */
    public function boom(): void
    {
    }

    /** @throws MyException */
    public static function staticBoom(): void
    {
    }

    /** @throws void */
    public function quiet(): void
    {
    }

    /** @return Thrower */
    public function plain()
    {
        return $this;
    }

    public function nullable(): ?self
    {
        return $this;
    }

    /** @return self */
    public function docNonNull(): ?self
    {
        return $this;
    }

    public function native(): self
    {
        return $this;
    }
}

/** @throws MyException */
function throwing(): void
{
}

/** @throws void */
function voidThrows(Thrower $t, ?Thrower $n, Thrower|\stdClass $u): void
{
    throw new MyException();
}

/** @throws void */
function voidCalls(Thrower $t, ?Thrower $n): void
{
    throwing();
    $t->boom();
    $n?->boom();
    Thrower::staticBoom();
    new Thrower();
    $t->quiet();
    strlen('a');
    $f = function () {
        throw new MyException();
    };
    $g = fn () => throw new MyException();
    try {
        throw new MyException();
    } catch (MyException $e) {
        if (rand(0, 1)) {
            throw new \RuntimeException();
        }
    } finally {
        throwing();
    }
    try {
        throwing();
    } catch (\LogicException $e) {
    }
    new class {};
    $method = 'boom';
    $t->$method();
}

/**
 * @throws \RuntimeException
 * @throws void
 */
function mixedThrows(): void
{
    throw new MyException();
}

function noDoc(): void
{
    throw new MyException();
}

/** @throws void */
function untyped($e): void
{
    throw $e;
}

class Hooks
{
    public int $a {
        /** @throws void */
        get {
            throw new MyException();
        }
        /** @throws void */
        set => throw new MyException();
    }

    public int $b {
        get => 1;
    }

}

function nullsafe(Thrower $t, ?Thrower $n, mixed $m, Thrower|\stdClass $u): void
{
    $t?->boom();
    $n?->boom();
    $m?->boom();
    $t?->plain()?->boom();
    $t->plain()?->boom();
    $t->nullable()?->boom();
    $t->docNonNull()?->boom();
    $t->native()?->boom();
    $t->{'native'}()?->boom();
    $u->native()?->boom();
    echo $t?->x;
}

function catches(): void
{
    try {
        throwing();
    } catch (\Exception $e) {
    } catch (MyException | \LogicException $e) {
    } catch (\Throwable $e) {
    } catch (\Throwable $e) {
    }
}

function incDec(array $a, string $s, bool $b, ?int $n, int $i, \stdClass $o, mixed $m, $x): void
{
    $a++;
    --$a;
    $s++;
    $s--;
    $b++;
    $n--;
    $n++;
    $i++;
    $o++;
    $m++;
    $x++;
}

/** @template T */
class Gen
{
    /** @param T $t */
    public function inc($t): void
    {
        $t++;
    }
}

class SelfCalls extends Thrower
{
    /** @throws void */
    public function calls(mixed $m, \DateTimeImmutable $d): void
    {
        try {
            self::staticBoom();
            static::staticBoom();
        } catch (MyException $e) {
        }
        $m->foo();
        $d->format('Y');
        $class = \stdClass::class;
        new $class();
    }
}

/** @throws void */
function declares(): void
{
    function innerThrower(): void
    {
        throw new MyException();
    }

    class InnerClass
    {
        public function boom(): void
        {
            throw new MyException();
        }
    }
}
