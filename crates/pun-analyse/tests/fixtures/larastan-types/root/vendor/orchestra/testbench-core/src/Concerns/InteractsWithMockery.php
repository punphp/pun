<?php

namespace Orchestra\Testbench\Concerns;

use Mockery;
use PHPUnit\Framework\TestCase as PHPUnitTestCase;

trait InteractsWithMockery
{
    /**
     * Teardown the testing environment.
     *
     * @return void
     */
    protected function tearDownTheTestEnvironmentUsingMockery(): void
    {}
}
