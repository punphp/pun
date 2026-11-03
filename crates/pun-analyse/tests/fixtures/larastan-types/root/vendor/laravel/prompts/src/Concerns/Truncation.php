<?php

namespace Laravel\Prompts\Concerns;

use InvalidArgumentException;

trait Truncation
{
    /**
     * Truncate a value with an ellipsis if it exceeds the given width.
     */
    protected function truncate(string $string, int $width): string
    {}
}
