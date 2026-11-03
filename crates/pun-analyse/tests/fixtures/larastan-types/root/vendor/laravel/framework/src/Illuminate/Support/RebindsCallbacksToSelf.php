<?php

declare(strict_types=1);

namespace Illuminate\Support;

use Closure;
use ReflectionFunction;

trait RebindsCallbacksToSelf
{
    /**
     * Binds the provided callback to the class instance.
     *
     * @throws \ReflectionException
     */
    protected function bindCallbackToSelf(Closure $callback): ?Closure
    {}
}
