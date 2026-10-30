<?php

namespace App;

interface Shape
{
    public function area(): float;

    public function name(): ?string;

    public function me(): self;

    public function many(): int|string;
}

abstract class Base
{
    public function make(): static
    {
        return $this;
    }

    abstract public function data(): array;

    public function plain()
    {
    }

    public function anything(): mixed
    {
        return 1;
    }

    private function hidden(): int
    {
        return 1;
    }
}

final class Square extends Base implements Shape, \Countable, \JsonSerializable, \IteratorAggregate
{
    public function area()
    {
        return 1.0;
    }

    public function name()
    {
        return null;
    }

    public function me()
    {
        return $this;
    }

    public function many()
    {
        return 1;
    }

    public function make()
    {
        return $this;
    }

    public function data()
    {
        return [];
    }

    public function plain()
    {
    }

    public function anything()
    {
        return 1;
    }

    public function hidden()
    {
        return 1;
    }

    public function count()
    {
        return 0;
    }

    public function jsonSerialize()
    {
        return [];
    }

    public function getIterator()
    {
        return new \ArrayIterator([]);
    }
}

$x = new class extends Base {
    public function data()
    {
        return [];
    }
};
-----
<?php

namespace App;

interface Shape
{
    public function area(): float;

    public function name(): ?string;

    public function me(): self;

    public function many(): int|string;
}

abstract class Base
{
    public function make(): static
    {
        return $this;
    }

    abstract public function data(): array;

    public function plain()
    {
    }

    public function anything(): mixed
    {
        return 1;
    }

    private function hidden(): int
    {
        return 1;
    }
}

final class Square extends Base implements Shape, \Countable, \JsonSerializable, \IteratorAggregate
{
    public function area(): float
    {
        return 1.0;
    }

    public function name(): ?string
    {
        return null;
    }

    public function me(): \App\Shape
    {
        return $this;
    }

    public function many(): int|string
    {
        return 1;
    }

    public function make(): static
    {
        return $this;
    }

    public function data(): array
    {
        return [];
    }

    public function plain()
    {
    }

    public function anything(): mixed
    {
        return 1;
    }

    public function hidden()
    {
        return 1;
    }

    public function count()
    {
        return 0;
    }

    public function jsonSerialize()
    {
        return [];
    }

    public function getIterator()
    {
        return new \ArrayIterator([]);
    }
}

$x = new class extends Base {
    public function data(): array
    {
        return [];
    }
};
