<?php

namespace CovInherited;

class Base
{
    /**
     * @phpstan-assert int $a
     * @phpstan-assert-if-true string $b
     * @phpstan-assert !null $this->prop
     */
    public function check(mixed $a, mixed $b): bool
    {
        return true;
    }

    /**
     * @phpstan-assert int $a
     */
    public static function only(mixed $a): void
    {
    }

    public ?int $prop = null;
}

interface Checks
{
    /**
     * @phpstan-assert int $value
     * @phpstan-assert int $extra
     */
    public function test(mixed $value, mixed $extra = null): void;
}

class Renamed extends Base
{
    public function check(mixed $x, mixed $y): bool
    {
        return true;
    }

    /**
     * @return void
     */
    public static function only(): void
    {
    }
}

class Fewer extends Base
{
    public function check(mixed $x): bool
    {
        return true;
    }
}

class Own extends Base
{
    /**
     * @phpstan-assert int $missing
     */
    public function check(mixed $a, mixed $b): bool
    {
        return true;
    }
}

class Implementer implements Checks
{
    public function test(mixed $other): void
    {
    }
}

class Deeper extends Fewer
{
    public function check(mixed $q): bool
    {
        return true;
    }
}
