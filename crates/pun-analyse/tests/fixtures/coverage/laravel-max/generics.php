<?php

namespace LaravelMax\Generics;

/**
 * @template TKey of array-key
 * @template TValue
 */
interface Arrayable
{
    /** @return array<TKey, TValue> */
    public function toArray(): array;
}

/**
 * @template TKey of array-key
 * @template TValue
 */
class Fluent
{
}

/**
 * @template TRelated
 * @template TDeclaring of object
 */
class Relation
{
}

/**
 * @template TModel of object
 */
class Builder
{
}

class Generics
{
    /** @var Fluent[] */
    public array $commands = [];

    /** @param Arrayable|array<mixed> $data */
    public function written($data): void
    {
        $this->takesArray($data);
    }

    /** @param Relation<*, *>|string $relation */
    public function wildcard($relation): int
    {
        return $relation;
    }

    public function element(): void
    {
        foreach ($this->commands as $command) {
            if ($command->shouldBeSkipped) {
                continue;
            }
        }
    }

    /** @param callable(Builder<\stdClass>): void $callback */
    public function has(callable $callback): void
    {
    }

    public function closure(): void
    {
        $this->has(function (Builder $query): void {
            $this->takesInt($query);
        });
    }

    /** @param (\Closure(Builder<\stdClass>): void)|null $callback */
    public function hasNullable(?\Closure $callback = null): void
    {
    }

    public function nullableClosure(): void
    {
        $this->hasNullable(function (Builder $query): void {
            $this->takesInt($query);
        });
    }

    /** @param array<mixed> $array */
    public function takesArray(array $array): void
    {
    }

    public function takesInt(int $int): void
    {
    }

    public function macro(string $name, \Closure $macro): void
    {
    }

    /** @param Builder<Generics> $builder */
    public function closureParameter(Builder $builder): void
    {
        $this->macro('withTrashed', function (Builder $builder) {
            return $builder->withTrashed();
        });
    }
}
