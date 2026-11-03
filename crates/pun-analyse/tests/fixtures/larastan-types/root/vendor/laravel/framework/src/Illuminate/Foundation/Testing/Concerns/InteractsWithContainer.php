<?php

namespace Illuminate\Foundation\Testing\Concerns;

use Closure;
use Illuminate\Foundation\Mix;
use Illuminate\Foundation\Vite;
use Illuminate\Support\Defer\DeferredCallbackCollection;
use Illuminate\Support\Facades\Vite as ViteFacade;
use Illuminate\Support\HtmlString;
use Mockery;

trait InteractsWithContainer
{
    /**
     * The original Vite handler.
     *
     * @var \Illuminate\Foundation\Vite|null
     */
    protected $originalVite;

    /**
     * The original Laravel Mix handler.
     *
     * @var \Illuminate\Foundation\Mix|null
     */
    protected $originalMix;

    /**
     * The original deferred callbacks collection.
     *
     * @var \Illuminate\Support\Defer\DeferredCallbackCollection|null
     */
    protected $originalDeferredCallbacksCollection;

    /**
     * Register an instance of an object in the container.
     *
     * @template TSwap of object
     *
     * @param  string  $abstract
     * @param  TSwap  $instance
     * @return TSwap
     */
    protected function swap($abstract, $instance)
    {}

    /**
     * Register an instance of an object in the container.
     *
     * @template TInstance of object
     *
     * @param  string  $abstract
     * @param  TInstance  $instance
     * @return TInstance
     */
    protected function instance($abstract, $instance)
    {}

    /**
     * Mock an instance of an object in the container.
     *
     * @template TInstance of object
     *
     * @param  string|class-string<TInstance>  $abstract
     * @param  \Closure|null  $mock
     * @return ($abstract is class-string<TInstance> ? TInstance&\Mockery\MockInterface : \Mockery\MockInterface)
     */
    protected function mock($abstract, ?Closure $mock = null)
    {}

    /**
     * Mock a partial instance of an object in the container.
     *
     * @template TInstance of object
     *
     * @param  string|class-string<TInstance>  $abstract
     * @param  \Closure|null  $mock
     * @return ($abstract is class-string<TInstance> ? TInstance&\Mockery\MockInterface : \Mockery\MockInterface)
     */
    protected function partialMock($abstract, ?Closure $mock = null)
    {}

    /**
     * Spy an instance of an object in the container.
     *
     * @template TInstance of object
     *
     * @param  string|class-string<TInstance>  $abstract
     * @param  \Closure|null  $mock
     * @return ($abstract is class-string<TInstance> ? TInstance&\Mockery\MockInterface : \Mockery\MockInterface)
     */
    protected function spy($abstract, ?Closure $mock = null)
    {}

    /**
     * Instruct the container to forget a previously mocked / spied instance of an object.
     *
     * @param  string  $abstract
     * @return $this
     */
    protected function forgetMock($abstract)
    {}

    /**
     * Register an empty handler for Vite in the container.
     *
     * @return $this
     */
    protected function withoutVite()
    {}

    /**
     * Restore Vite in the container.
     *
     * @return $this
     */
    protected function withVite()
    {}

    /**
     * Register an empty handler for Laravel Mix in the container.
     *
     * @return $this
     */
    protected function withoutMix()
    {}

    /**
     * Restore Laravel Mix in the container.
     *
     * @return $this
     */
    protected function withMix()
    {}

    /**
     * Execute deferred functions immediately.
     *
     * @return $this
     */
    protected function withoutDefer()
    {}

    /**
     * Restore deferred functions.
     *
     * @return $this
     */
    protected function withDefer()
    {}
}
