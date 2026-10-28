<?php

namespace App;

/**
 * @return int|string
 */
function plain(): int|string
{
    return 1;
}

interface Shape
{
    /**
     * @return int|string
     */
    public function area(): int|string;
}

abstract class Base
{
    /**
     * @return int|string
     * @phpstan-return int|string
     */
    abstract public function run(): int|string;

    /**
     * @return \stdClass|Base
     */
    abstract public function objects(): \stdClass|self;

    /**
     * @return Base|null
     */
    abstract public function nullable(): ?self;

    /**
     * @return array<int>|string
     */
    abstract public function generic(): array|string;

    /**
     * @return bool|int
     */
    abstract public function scalar(): bool|int;

    /**
     * @return false|int
     */
    abstract public function falsy(): false|int;
}
-----
<?php

namespace App;

function plain(): int|string
{
    return 1;
}

interface Shape
{
    public function area(): int|string;
}

abstract class Base
{
    /**
     * @phpstan-return int|string
     */
    abstract public function run(): int|string;

    abstract public function objects(): \stdClass|self;

    abstract public function nullable(): ?self;

    /**
     * @return array<int>|string
     */
    abstract public function generic(): array|string;

    abstract public function scalar(): bool|int;

    abstract public function falsy(): false|int;
}
