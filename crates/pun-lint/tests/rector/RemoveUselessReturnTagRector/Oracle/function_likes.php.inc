<?php

namespace App;

/**
 * @return int
 */
function plain(): int
{
    return 1;
}

interface Shape
{
    /**
     * @return int
     */
    public function area(): int;
}

abstract class Base
{
    /**
     * @phpstan-return int
     * @return int
     */
    abstract public function run(): int;

    /**
     * @return int|string
     */
    abstract public function union(): string|int;

    /**
     * @return array
     */
    abstract public function list(): array;

    /**
     * @return mixed
     */
    abstract public function anything(): mixed;

    /**
     * @return null
     */
    abstract public function nothing(): null;

    /**
     * @return \stdClass
     */
    abstract public function object(): object;
}

$closure = function (): int {
    /** @return int */
    return 1;
};
-----
<?php

namespace App;

function plain(): int
{
    return 1;
}

interface Shape
{
    public function area(): int;
}

abstract class Base
{
    /**
     * @phpstan-return int
     */
    abstract public function run(): int;

    abstract public function union(): string|int;

    abstract public function list(): array;

    abstract public function anything(): mixed;

    abstract public function nothing(): null;

    /**
     * @return \stdClass
     */
    abstract public function object(): object;
}

$closure = function (): int {
    /** @return int */
    return 1;
};
