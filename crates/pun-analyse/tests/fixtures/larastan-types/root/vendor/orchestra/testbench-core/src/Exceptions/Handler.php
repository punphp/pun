<?php

namespace Orchestra\Testbench\Exceptions;

use Illuminate\Foundation\Exceptions\Handler as ExceptionHandler;
use Throwable;

/**
 * @internal
 */
class Handler extends ExceptionHandler
{
    /** {@inheritDoc} */
    #[\Override]
    public function report(Throwable $e)
    {}

    /** {@inheritDoc} */
    #[\Override]
    public function render($request, Throwable $e)
    {}
}
