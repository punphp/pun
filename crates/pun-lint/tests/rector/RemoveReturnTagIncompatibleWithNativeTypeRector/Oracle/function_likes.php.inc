<?php

namespace App;

interface Shape
{
    /**
     * @return string
     */
    public function area(): int;
}

abstract class Base
{
    /**
     * @return \stdClass
     */
    abstract public function run(): self;

    /**
     * @return self
     */
    abstract public function same(): static;

    /**
     * @return Base
     */
    abstract public function base(): self;

    /**
     * @return array<int>
     */
    abstract public function generic(): array;

    /**
     * @return \Generator<int>
     */
    abstract public function gen(): iterable;

    /**
     * @return string|null
     */
    abstract public function nullable(): ?int;

    /**
     * @phpstan-return string
     */
    abstract public function phpstan(): int;
}

$closure = /** @return string */ function (): int {
    return 1;
};
-----
<?php

namespace App;

interface Shape
{
    public function area(): int;
}

abstract class Base
{
    abstract public function run(): self;

    /**
     * @return self
     */
    abstract public function same(): static;

    /**
     * @return Base
     */
    abstract public function base(): self;

    /**
     * @return array<int>
     */
    abstract public function generic(): array;

    /**
     * @return \Generator<int>
     */
    abstract public function gen(): iterable;

    /**
     * @return string|null
     */
    abstract public function nullable(): ?int;

    /**
     * @phpstan-return string
     */
    abstract public function phpstan(): int;
}

$closure = /** @return string */ function (): int {
    return 1;
};
