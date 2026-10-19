<?php

namespace App;

class Builder
{
    private array $wheres = [];

    /** @return $this */
    public function where($column)
    {
        $this->wheres[] = $column;

        return $this;
    }

    /** @return $this */
    public function orWhere($column)
    {
        return $this->untyped($column);
    }

    /** @return $this */
    public function untyped($column)
    {
        if ($column) {
            return $this;
        }
    }

    public function fresh()
    {
        return new static();
    }

    public function copy()
    {
        return new self();
    }

    public function maybe($x)
    {
        if ($x) {
            return $this;
        }
    }

    public function other()
    {
        return new Other();
    }

    public function chain()
    {
        return $this->untyped('a')->untyped('b');
    }
}

final class Done
{
    public function me()
    {
        return $this;
    }
}

class Other
{
}

trait Fluent
{
    public function self()
    {
        return $this;
    }
}

enum Suit
{
    case Hearts;

    public function me()
    {
        return $this;
    }
}
-----
<?php

namespace App;

class Builder
{
    private array $wheres = [];

    /** @return $this */
    public function where($column): static
    {
        $this->wheres[] = $column;

        return $this;
    }

    /** @return $this */
    public function orWhere($column)
    {
        return $this->untyped($column);
    }

    /** @return $this */
    public function untyped($column)
    {
        if ($column) {
            return $this;
        }
    }

    public function fresh(): static
    {
        return new static();
    }

    public function copy(): self
    {
        return new self();
    }

    public function maybe($x)
    {
        if ($x) {
            return $this;
        }
    }

    public function other()
    {
        return new Other();
    }

    public function chain()
    {
        return $this->untyped('a')->untyped('b');
    }
}

final class Done
{
    public function me(): self
    {
        return $this;
    }
}

class Other
{
}

trait Fluent
{
    public function self()
    {
        return $this;
    }
}

enum Suit
{
    case Hearts;

    public function me(): self
    {
        return $this;
    }
}
