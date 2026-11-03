<?php

namespace LaravelMax\Loops;

class Loops
{
    /** @param list<string> $columns */
    public function previous(array $columns): void
    {
        foreach ($columns as $column) {
            if (isset($previous)) {
                $previous->foo();
            }
            $previous = strlen($column);
        }
    }

    public function coalesce(int $n): void
    {
        while ($n-- > 0) {
            ($last ?? null)->foo();
            $last = $this;
        }
    }
}
