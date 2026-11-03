<?php

namespace Orchestra\Testbench\Concerns;

use Closure;
use Illuminate\Foundation\Testing\DatabaseMigrations;
use Illuminate\Foundation\Testing\DatabaseTransactions;
use Illuminate\Foundation\Testing\DatabaseTruncation;
use Illuminate\Foundation\Testing\RefreshDatabase;
use Illuminate\Foundation\Testing\WithFaker;
use Illuminate\Foundation\Testing\WithoutMiddleware;
use Illuminate\Support\LazyCollection;
use Orchestra\Testbench\Pest\WithPest;
use PHPUnit\Framework\TestCase as PHPUnitTestCase;

use function Orchestra\Sidekick\once;

/**
 * @api
 */
trait Testing
{
    use ApplicationTestingHooks;
    use CreatesApplication;
    use HandlesAssertions;
    use HandlesAttributes;
    use HandlesDatabases;
    use HandlesRoutes;
    use InteractsWithMigrations;
    use WithFactories;

    /**
     * Setup the test environment.
     *
     * @internal
     *
     * @return void
     */
    final protected function setUpTheTestEnvironment(): void
    {}

    /**
     * Clean up the testing environment before the next test.
     *
     * @internal
     *
     * @return void
     */
    final protected function tearDownTheTestEnvironment(): void
    {}

    /**
     * Boot the testing helper traits.
     *
     * @internal
     *
     * @param  array<class-string, class-string>  $uses
     * @return array<class-string, class-string>
     */
    final protected function setUpTheTestEnvironmentTraits(array $uses): array
    {}

    /**
     * Reload the application instance with cached routes.
     *
     * @api
     *
     * @return void
     */
    protected function reloadApplication(): void
    {}

    /**
     * Determine trait should be ignored from being autoloaded.
     *
     * @param  class-string  $use
     * @return bool
     */
    abstract protected function setUpTheTestEnvironmentTraitToBeIgnored(string $use): bool;

    /**
     * Boot the testing helper traits.
     *
     * @return array<class-string, class-string>
     */
    abstract protected function setUpTraits();
}
