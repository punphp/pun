<?php

namespace Orchestra\Testbench\Bootstrap;

use Illuminate\Log\LogManager;
use Orchestra\Testbench\Exceptions\DeprecatedException;
use Orchestra\Testbench\Foundation\Env;

use function Orchestra\Sidekick\Filesystem\join_paths;

/**
 * @internal
 */
final class HandleExceptions extends \Illuminate\Foundation\Bootstrap\HandleExceptions
{
    /**
     * {@inheritDoc}
     *
     * @throws \Orchestra\Testbench\Exceptions\DeprecatedException
     */
    #[\Override]
    public function handleDeprecationError($message, $file, $line, $level = E_DEPRECATED)
    {}

    /** {@inheritDoc} */
    #[\Override]
    protected function ensureDeprecationLoggerIsConfigured()
    {}

    /** {@inheritDoc} */
    #[\Override]
    protected function shouldIgnoreDeprecationErrors()
    {}
}
