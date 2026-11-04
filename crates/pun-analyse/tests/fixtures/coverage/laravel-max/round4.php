<?php

namespace LaravelMax\Round4;

/**
 * `is_int()` of a benevolent `(int|string)` key is maybe an int: no function.alreadyNarrowedType.
 *
 * @param array<mixed> $files
 */
function keys(array $files): int
{
    $n = 0;
    foreach ($files as $file => $options) {
        if (is_int($file)) {
            $n++;
        }
    }

    return $n;
}

/**
 * A `@mixin TValue` reached through a template argument binds `$this` to that template.
 *
 * @template TValue
 * @mixin TValue
 */
class Proxy
{
    /**
     * @param array<mixed> $a
     * @return mixed
     */
    public function __call(string $m, array $a)
    {
        return null;
    }
}

class Model
{
    /** @return $this */
    public function makeHidden(string $a)
    {
        return $this;
    }
}

/**
 * @template TModel of Model
 * @property-read Proxy<TModel> $each
 */
class Models
{
    /** @return $this */
    public function makeHidden(string $a)
    {
        return $this->each->makeHidden($a);
    }

    public function __get(string $n): mixed
    {
        return null;
    }
}

/**
 * MethodSignatureRule: PHPDoc return and parameter types incompatible with the parent's.
 */
class Command
{
    /** @return bool|null */
    public function handle()
    {
        return null;
    }

    /** @param int $x */
    public function take($x): void
    {
    }

    /** @return $this */
    public function fluent()
    {
        return $this;
    }
}

class SeederCommand extends Command
{
    /** @return void */
    public function handle()
    {
    }

    /** @param string $x */
    public function take($x): void
    {
    }

    /** @return void */
    public function fluent()
    {
    }
}

/**
 * @template TKey of array-key
 * @template TValue
 */
interface Paginator
{
    /** @return string */
    public function render();

    /** @return TValue */
    public function first();
}

/**
 * @implements Paginator<int, string>
 */
class StringPage implements Paginator
{
    /** @return string */
    public function render()
    {
        return '';
    }

    /** @return int */
    public function first()
    {
        return 1;
    }
}

trait Counts
{
    /** @return int */
    abstract public function total();

    /** @return $this */
    public function chain()
    {
        return $this;
    }
}

class UsesCounts
{
    use Counts;

    /** @return string */
    public function total()
    {
        return '';
    }
}

class ChildOfCounts extends UsesCounts
{
    /** @return void */
    public function chain()
    {
    }
}

/**
 * An offset write or unset of a property assigns the property's whole new value.
 */
class Offsets
{
    /** @var array<string> */
    protected array $columns = [];

    /** @var array<string, array<int, string>> */
    protected array $listeners = [];

    /** @var array<array<mixed>> */
    protected array $callbacks = [];

    public function add(mixed $c, string $k, int $i, \Closure|string $abstract): void
    {
        $this->columns[] = $c;
        $this->listeners[$k][] = $i;
        $this->callbacks[$abstract][] = $i;
        unset($this->columns[$k]);
    }
}
