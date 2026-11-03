<?php

namespace Orchestra\Testbench\Concerns;

use Carbon\Carbon;
use Carbon\CarbonImmutable;
use Closure;
use Illuminate\Database\Eloquent\Model;
use Illuminate\Support\Facades\ParallelTesting;
use Orchestra\Testbench\Foundation\Application as Testbench;
use PHPUnit\Framework\TestCase as PHPUnitTestCase;
use Throwable;

trait ApplicationTestingHooks
{
    use InteractsWithMockery;
    use InteractsWithPest;
    use InteractsWithPHPUnit;
    use InteractsWithTestCase;

    /**
     * The Illuminate application instance.
     *
     * @var \Illuminate\Foundation\Application|null
     */
    protected $app;

    /**
     * The callbacks that should be run after the application is created.
     *
     * @var array<int, callable():void>
     */
    protected array $afterApplicationCreatedCallbacks = [];

    /**
     * The callbacks that should be run after the application is refreshed.
     *
     * @var array<int, callable():void>
     */
    protected array $afterApplicationRefreshedCallbacks = [];

    /**
     * The callbacks that should be run before the application is destroyed.
     *
     * @var array<int, callable():void>
     */
    protected array $beforeApplicationDestroyedCallbacks = [];

    /**
     * The exception thrown while running an application destruction callback.
     *
     * @var \Throwable|null
     */
    protected ?Throwable $callbackException = null;

    /**
     * Indicates if we have made it through the base setUp function.
     *
     * @var bool
     */
    protected bool $setUpHasRun = false;

    /**
     * Setup the testing hooks.
     *
     * @internal
     *
     * @param  (\Closure():(void))|null  $callback
     * @return void
     */
    final protected function setUpTheApplicationTestingHooks(?Closure $callback = null): void
    {}

    /**
     * Teardown the testing hooks.
     *
     * @internal
     *
     * @param  (\Closure():(void))|null  $callback
     * @return void
     *
     * @throws \Throwable
     */
    final protected function tearDownTheApplicationTestingHooks(?Closure $callback = null): void
    {}

    /**
     * Setup parallel testing callback.
     *
     * @internal
     *
     * @return void
     */
    protected function setUpParallelTestingCallbacks(): void
    {}

    /**
     * Teardown parallel testing callback.
     *
     * @internal
     *
     * @return void
     */
    protected function tearDownParallelTestingCallbacks(): void
    {}

    /**
     * Register a callback to be run after the application is refreshed.
     *
     * @api
     *
     * @param  callable():void  $callback
     * @return void
     */
    public function afterApplicationRefreshed(callable $callback): void
    {}

    /**
     * Execute the application's post-refreshed callbacks.
     *
     * @internal
     *
     * @return void
     */
    protected function callAfterApplicationRefreshedCallbacks(): void
    {}

    /**
     * Register a callback to be run after the application is created.
     *
     * @api
     *
     * @param  callable():void  $callback
     * @return void
     */
    public function afterApplicationCreated(callable $callback): void
    {}

    /**
     * Execute the application's post-creation callbacks.
     *
     * @internal
     *
     * @return void
     */
    protected function callAfterApplicationCreatedCallbacks(): void
    {}

    /**
     * Register a callback to be run before the application is destroyed.
     *
     * @api
     *
     * @param  callable():void  $callback
     * @return void
     */
    public function beforeApplicationDestroyed(callable $callback): void
    {}

    /**
     * Execute the application's pre-destruction callbacks.
     *
     * @internal
     *
     * @return void
     */
    protected function callBeforeApplicationDestroyedCallbacks(): void
    {}

    /**
     * Refresh the application instance.
     *
     * @return void
     */
    abstract protected function refreshApplication();
}
