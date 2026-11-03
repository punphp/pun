<?php

namespace Orchestra\Sidekick;

use JsonSerializable;
use Stringable;

/**
 * @api
 *
 * @template TValue of mixed
 */
final class SensitiveValue implements JsonSerializable, Stringable
{
    /**
     * Construct a new sensitive value.
     *
     * @param  TValue  $value
     */
    public function __construct(
        private readonly mixed $value
    ) {}

    /**
     * Get the original value.
     *
     * @return TValue
     */
    public function getValue(): mixed
    {}

    /**
     * Transform the value for debugging.
     */
    public function __debugInfo(): array
    {}

    /**
     * Get the value for string serialization.
     */
    public function __toString(): string
    {}

    /**
     * Get the value for JSON serialization.
     */
    public function jsonSerialize(): string
    {}
}
