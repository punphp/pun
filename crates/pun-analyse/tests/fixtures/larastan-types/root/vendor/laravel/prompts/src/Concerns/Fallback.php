<?php

namespace Laravel\Prompts\Concerns;

use Closure;
use RuntimeException;

trait Fallback
{
    /**
     * Whether to fallback to a custom implementation
     */
    protected static bool $shouldFallback = false;

    /**
     * The fallback implementations.
     *
     * @var array<class-string, Closure($this): mixed>
     */
    protected static array $fallbacks = [];

    /**
     * Enable the fallback implementation.
     */
    public static function fallbackWhen(bool $condition): void
    {}

    /**
     * Whether the prompt should fallback to a custom implementation.
     */
    public static function shouldFallback(): bool
    {}

    /**
     * Set the fallback implementation.
     *
     * @param  Closure($this): mixed  $fallback
     */
    public static function fallbackUsing(Closure $fallback): void
    {}

    /**
     * Call the registered fallback implementation.
     */
    public function fallback(): mixed
    {}
}
