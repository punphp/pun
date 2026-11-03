<?php

namespace Orchestra\Testbench\Concerns;

use Illuminate\Support\Str;

use function Orchestra\Sidekick\Filesystem\filename_from_classname;
use function Orchestra\Testbench\in_parallel_testing;

/**
 * @api
 *
 * @codeCoverageIgnore
 */
trait WithFixtures
{
    use InteractsWithPest;

    /**
     * Setup test case to include fixture file using ".fixtures.php" suffix if it's available.
     *
     * @return void
     */
    protected static function setupWithFixturesForTestingEnvironment(): void
    {}
}
