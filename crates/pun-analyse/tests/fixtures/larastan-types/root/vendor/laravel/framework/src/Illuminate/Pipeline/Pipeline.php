<?php

namespace Illuminate\Pipeline;

use Closure;
use Illuminate\Contracts\Container\Container;
use Illuminate\Contracts\Pipeline\Pipeline as PipelineContract;
use Illuminate\Support\Traits\Conditionable;
use Illuminate\Support\Traits\Macroable;
use RuntimeException;
use Throwable;

class Pipeline implements PipelineContract
{
    use Conditionable;
    use Macroable;

    /**
     * The container implementation.
     *
     * @var \Illuminate\Contracts\Container\Container|null
     */
    protected $container;

    /**
     * The object being passed through the pipeline.
     *
     * @var mixed
     */
    protected $passable;

    /**
     * The array of class pipes.
     *
     * @var array
     */
    protected $pipes = [];

    /**
     * The method to call on each pipe.
     *
     * @var string
     */
    protected $method = 'handle';

    /**
     * The final callback to be executed after the pipeline ends regardless of the outcome.
     *
     * @var \Closure|null
     */
    protected $finally;

    /**
     * Indicates whether to wrap the pipeline in a database transaction.
     *
     * @var string|null|\UnitEnum|false
     */
    protected $withinTransaction = false;

    /**
     * Create a new class instance.
     *
     * @param  \Illuminate\Contracts\Container\Container|null  $container
     */
    public function __construct(?Container $container = null)
    {}

    /**
     * Set the object being sent through the pipeline.
     *
     * @param  mixed  $passable
     * @return $this
     */
    public function send($passable)
    {}

    /**
     * Set the array of pipes.
     *
     * @param  mixed  $pipes
     * @return $this
     */
    public function through($pipes)
    { func_get_args(); }

    /**
     * Push additional pipes onto the pipeline.
     *
     * @param  mixed  $pipes
     * @return $this
     */
    public function pipe($pipes)
    { func_get_args(); }

    /**
     * Set the method to call on the pipes.
     *
     * @param  string  $method
     * @return $this
     */
    public function via($method)
    {}

    /**
     * Run the pipeline with a final destination callback.
     *
     * @param  \Closure  $destination
     * @return mixed
     */
    public function then(Closure $destination)
    {}

    /**
     * Run the pipeline and return the result.
     *
     * @return mixed
     */
    public function thenReturn()
    {}

    /**
     * Set a final callback to be executed after the pipeline ends regardless of the outcome.
     *
     * @param  \Closure  $callback
     * @return $this
     */
    public function finally(Closure $callback)
    {}

    /**
     * Get the final piece of the Closure onion.
     *
     * @param  \Closure  $destination
     * @return \Closure
     */
    protected function prepareDestination(Closure $destination)
    {}

    /**
     * Get a Closure that represents a slice of the application onion.
     *
     * @return \Closure
     */
    protected function carry()
    {}

    /**
     * Parse full pipe string to get name and parameters.
     *
     * @param  string  $pipe
     * @return array
     */
    protected function parsePipeString($pipe)
    {}

    /**
     * Get the array of configured pipes.
     *
     * @return array
     */
    protected function pipes()
    {}

    /**
     * Execute each pipeline step within a database transaction.
     *
     * @param  string|null|\UnitEnum|false  $withinTransaction
     * @return $this
     */
    public function withinTransaction($withinTransaction = null)
    {}

    /**
     * Get the container instance.
     *
     * @return \Illuminate\Contracts\Container\Container
     *
     * @throws \RuntimeException
     */
    protected function getContainer()
    {}

    /**
     * Set the container instance.
     *
     * @param  \Illuminate\Contracts\Container\Container  $container
     * @return $this
     */
    public function setContainer(Container $container)
    {}

    /**
     * Handle the value returned from each pipe before passing it to the next.
     *
     * @param  mixed  $carry
     * @return mixed
     */
    protected function handleCarry($carry)
    {}

    /**
     * Handle the given exception.
     *
     * @param  mixed  $passable
     * @param  \Throwable  $e
     * @return mixed
     *
     * @throws \Throwable
     */
    protected function handleException($passable, Throwable $e)
    {}
}
