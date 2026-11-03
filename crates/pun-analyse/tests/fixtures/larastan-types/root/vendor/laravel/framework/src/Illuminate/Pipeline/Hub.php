<?php

namespace Illuminate\Pipeline;

use Closure;
use Illuminate\Contracts\Container\Container;
use Illuminate\Contracts\Pipeline\Hub as HubContract;
use InvalidArgumentException;

class Hub implements HubContract
{
    /**
     * The container implementation.
     *
     * @var \Illuminate\Contracts\Container\Container|null
     */
    protected $container;

    /**
     * All of the available pipelines.
     *
     * @var array
     */
    protected $pipelines = [];

    /**
     * Create a new Hub instance.
     *
     * @param  \Illuminate\Contracts\Container\Container|null  $container
     */
    public function __construct(?Container $container = null)
    {}

    /**
     * Define the default named pipeline.
     *
     * @param  \Closure  $callback
     * @return void
     */
    public function defaults(Closure $callback)
    {}

    /**
     * Define a new named pipeline.
     *
     * @param  string  $name
     * @param  \Closure  $callback
     * @return void
     */
    public function pipeline($name, Closure $callback)
    {}

    /**
     * Send an object through one of the available pipelines.
     *
     * @param  mixed  $object
     * @param  string|null  $pipeline
     * @return mixed
     */
    public function pipe($object, $pipeline = null)
    {}

    /**
     * Get the container instance used by the hub.
     *
     * @return \Illuminate\Contracts\Container\Container|null
     */
    public function getContainer()
    {}

    /**
     * Set the container instance used by the hub.
     *
     * @param  \Illuminate\Contracts\Container\Container  $container
     * @return $this
     */
    public function setContainer(Container $container)
    {}
}
