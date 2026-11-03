<?php

declare(strict_types=1);

namespace Termwind\Exceptions;

use InvalidArgumentException;

/**
 * @internal
 */
final class StyleNotFound extends InvalidArgumentException
{
    /**
     * Creates a new style not found instance.
     */
    private function __construct(string $message)
    {}

    /**
     * Creates a new style not found instance from the given style.
     */
    public static function fromStyle(string $style): self
    {}
}
