<?php

namespace LaravelMax\Callables;

class Callables
{
    /** @param callable $callback */
    public function tagged($callback): void
    {
        new \ReflectionFunction($callback);
    }

    /** @param \Closure|string|array<mixed> $class */
    public function closure($class): void
    {
        $this->takesString($class);
    }

    /** @param \Closure $closure */
    public function closureTag($closure): void
    {
        $this->takesString($closure);
    }

    public function takesString(string $string): void
    {
    }
}
