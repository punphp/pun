<?php

namespace App;

interface Enumerable
{
}

trait EnumeratesValues
{
    public static function wrap($value, ...$args)
    {
        return $value instanceof Enumerable
            ? new static($value, ...$args)
            : new static([$value], ...$args);
    }

    public function me()
    {
        return $this;
    }

    public function fresh()
    {
        return new static();
    }

    public function copy()
    {
        return new self();
    }
}
-----
<?php

namespace App;

interface Enumerable
{
}

trait EnumeratesValues
{
    public static function wrap($value, ...$args): static
    {
        return $value instanceof Enumerable
            ? new static($value, ...$args)
            : new static([$value], ...$args);
    }

    public function me()
    {
        return $this;
    }

    public function fresh(): static
    {
        return new static();
    }

    public function copy(): self
    {
        return new self();
    }
}
