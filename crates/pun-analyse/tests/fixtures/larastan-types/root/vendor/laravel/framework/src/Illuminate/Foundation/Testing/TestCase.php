<?php

namespace Illuminate\Foundation\Testing;

use Illuminate\Contracts\Console\Kernel;
use Illuminate\Foundation\Application;
use Illuminate\Foundation\Testing\Attributes\UnitTest;
use PHPUnit\Framework\TestCase as BaseTestCase;
use ReflectionMethod;
use Throwable;

abstract class TestCase extends BaseTestCase
{
    use Concerns\InteractsWithContainer,
        Concerns\MakesHttpRequests,
        Concerns\InteractsWithAuthentication,
        Concerns\InteractsWithConsole,
        Concerns\InteractsWithDatabase,
        Concerns\InteractsWithDeprecationHandling,
        Concerns\InteractsWithExceptionHandling,
        Concerns\InteractsWithSession,
        Concerns\InteractsWithTime,
        Concerns\InteractsWithTestCaseLifecycle,
        Concerns\InteractsWithViews;

    /**
     * The list of trait that this test uses, fetched recursively.
     *
     * @var array<class-string, int>
     */
    protected array $traitsUsedByTest;

    /**
     * Memoized result of the withoutBootingFramework check.
     *
     * @var bool|null
     */
    protected ?bool $withoutBootingFramework = null;

    /**
     * Creates the application.
     *
     * @return \Illuminate\Foundation\Application
     */
    public function createApplication()
    {}

    /**
     * Setup the test environment.
     *
     * @return void
     */
    protected function setUp(): void
    {}

    /**
     * Refresh the application instance.
     *
     * @return void
     */
    protected function refreshApplication()
    {}

    /**
     * Clean up the testing environment before the next test.
     *
     * @return void
     *
     * @throws \Mockery\Exception\InvalidCountException
     */
    protected function tearDown(): void
    {}

    /**
     * Determine if the test method should boot the framework.
     *
     * @return bool
     *
     * @throws \ReflectionException
     */
    protected function withoutBootingFramework(): bool
    {}

    /**
     * Clean up the testing environment before the next test case.
     *
     * @return void
     */
    public static function tearDownAfterClass(): void
    {}
}
