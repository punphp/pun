<?php

namespace Orchestra\Testbench\Concerns;

use Exception;
use Illuminate\Database\Eloquent\Factory as ModelFactory;

use function Orchestra\Testbench\laravel_or_fail;

/**
 * @api
 *
 * @deprecated
 *
 * @codeCoverageIgnore
 */
trait WithFactories
{
    /**
     * Load model factories from path.
     *
     * @internal
     *
     * @param  string  $path
     * @return $this
     *
     * @throws \Exception
     */
    protected function withFactories(string $path)
    {}

    /**
     * Load model factories from path using Application.
     *
     * @param  \Illuminate\Contracts\Foundation\Application  $app
     * @param  string  $path
     * @return $this
     *
     * @throws \Exception
     */
    protected function loadFactoriesUsing($app, string $path)
    {}
}
