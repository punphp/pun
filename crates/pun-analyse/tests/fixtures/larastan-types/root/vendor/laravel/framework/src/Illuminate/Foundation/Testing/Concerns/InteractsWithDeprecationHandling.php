<?php

namespace Illuminate\Foundation\Testing\Concerns;

use ErrorException;

trait InteractsWithDeprecationHandling
{
    /**
     * The original deprecation handler.
     *
     * @var callable|null
     */
    protected $originalDeprecationHandler;

    /**
     * Restore deprecation handling.
     *
     * @return $this
     */
    protected function withDeprecationHandling()
    {}

    /**
     * Disable deprecation handling for the test.
     *
     * @return $this
     *
     * @throws \ErrorException
     */
    protected function withoutDeprecationHandling()
    {}
}
