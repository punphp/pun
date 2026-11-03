<?php

namespace Illuminate\View\Engines;

use Closure;
use InvalidArgumentException;

class EngineResolver
{
    /**
     * The array of engine resolvers.
     *
     * @var array
     */
    protected $resolvers = [];

    /**
     * The resolved engine instances.
     *
     * @var array
     */
    protected $resolved = [];

    /**
     * Register a new engine resolver.
     *
     * The engine string typically corresponds to a file extension.
     *
     * @param  string  $engine
     * @param  \Closure  $resolver
     * @return void
     */
    public function register($engine, Closure $resolver)
    {}

    /**
     * Resolve an engine instance by name.
     *
     * @param  string  $engine
     * @return \Illuminate\Contracts\View\Engine
     *
     * @throws \InvalidArgumentException
     */
    public function resolve($engine)
    {}

    /**
     * Remove a resolved engine.
     *
     * @param  string  $engine
     * @return void
     */
    public function forget($engine)
    {}
}
