<?php

namespace Illuminate\Bus;

use Closure;
use Illuminate\Bus\Events\BatchDispatched;
use Illuminate\Contracts\Container\Container;
use Illuminate\Contracts\Events\Dispatcher as EventDispatcher;
use Illuminate\Support\Arr;
use Illuminate\Support\Collection;
use Illuminate\Support\Traits\Conditionable;
use Laravel\SerializableClosure\SerializableClosure;
use RuntimeException;
use Throwable;
use UnitEnum;

use function Illuminate\Support\enum_value;

/**
 * @phpstan-type TParam (Closure(\Illuminate\Bus\Batch, \Throwable|null): mixed)|(callable(\Illuminate\Bus\Batch, \Throwable|null): mixed)
 */
class PendingBatch
{
    use Conditionable;

    /**
     * The IoC container instance.
     *
     * @var \Illuminate\Contracts\Container\Container
     */
    protected $container;

    /**
     * The batch name.
     *
     * @var string
     */
    public $name = '';

    /**
     * The jobs that belong to the batch.
     *
     * @var \Illuminate\Support\Collection
     */
    public $jobs;

    /**
     * The batch options.
     *
     * @var array
     */
    public $options = [];

    /**
     * Jobs that have been verified to contain the Batchable trait.
     *
     * @var array<class-string, bool>
     */
    protected static $batchableClasses = [];

    /**
     * Create a new pending batch instance.
     *
     * @param  \Illuminate\Contracts\Container\Container  $container
     * @param  \Illuminate\Support\Collection  $jobs
     */
    public function __construct(Container $container, Collection $jobs)
    {}

    /**
     * Add jobs to the batch.
     *
     * @param  iterable|object|array  $jobs
     * @return $this
     */
    public function add($jobs)
    {}

    /**
     * Ensure the given job is batchable.
     *
     * @param  object|array  $job
     * @return void
     *
     * @throws \RuntimeException
     */
    protected function ensureJobIsBatchable(object|array $job): void
    {}

    /**
     * Add a callback to be executed when the batch is stored.
     *
     * @param  callable  $callback
     * @return $this
     */
    public function before($callback)
    {}

    /**
     * Get the "before" callbacks that have been registered with the pending batch.
     *
     * @return array
     */
    public function beforeCallbacks()
    {}

    /**
     * Add a callback to be executed after a job in the batch have executed successfully.
     *
     * @param  callable  $callback
     * @return $this
     */
    public function progress($callback)
    {}

    /**
     * Get the "progress" callbacks that have been registered with the pending batch.
     *
     * @return array
     */
    public function progressCallbacks()
    {}

    /**
     * Add a callback to be executed after all jobs in the batch have executed successfully.
     *
     * @param  callable  $callback
     * @return $this
     */
    public function then($callback)
    {}

    /**
     * Get the "then" callbacks that have been registered with the pending batch.
     *
     * @return array
     */
    public function thenCallbacks()
    {}

    /**
     * Add a callback to be executed after the first failing job in the batch.
     *
     * @param  callable  $callback
     * @return $this
     */
    public function catch($callback)
    {}

    /**
     * Get the "catch" callbacks that have been registered with the pending batch.
     *
     * @return array
     */
    public function catchCallbacks()
    {}

    /**
     * Add a callback to be executed after the batch has finished executing.
     *
     * @param  callable  $callback
     * @return $this
     */
    public function finally($callback)
    {}

    /**
     * Get the "finally" callbacks that have been registered with the pending batch.
     *
     * @return array
     */
    public function finallyCallbacks()
    {}

    /**
     * Indicate that the batch should not be canceled when a job within the batch fails.
     *
     * Optionally, add callbacks to be executed upon each job failure.
     *
     * @param  bool|TParam|array<array-key, TParam>  $param
     * @return $this
     */
    public function allowFailures($param = true)
    {}

    /**
     * Determine if the pending batch allows jobs to fail without cancelling the batch.
     *
     * @return bool
     */
    public function allowsFailures()
    {}

    /**
     * Get the "failure" callbacks that have been registered with the pending batch.
     *
     * @return array<array-key, Closure|callable>
     */
    public function failureCallbacks(): array
    {}

    /**
     * Register a callback with proper serialization.
     */
    private function registerCallback(string $type, Closure|callable $callback): void
    {}

    /**
     * Set the name for the batch.
     *
     * @param  string  $name
     * @return $this
     */
    public function name(string $name)
    {}

    /**
     * Specify the queue connection that the batched jobs should run on.
     *
     * @param  \UnitEnum|string  $connection
     * @return $this
     */
    public function onConnection(UnitEnum|string $connection)
    {}

    /**
     * Get the connection used by the pending batch.
     *
     * @return string|null
     */
    public function connection()
    {}

    /**
     * Specify the queue that the batched jobs should run on.
     *
     * @param  \UnitEnum|string|null  $queue
     * @return $this
     */
    public function onQueue($queue)
    {}

    /**
     * Get the queue used by the pending batch.
     *
     * @return string|null
     */
    public function queue()
    {}

    /**
     * Add additional data into the batch's options array.
     *
     * @param  string  $key
     * @param  mixed  $value
     * @return $this
     */
    public function withOption(string $key, $value)
    {}

    /**
     * Dispatch the batch.
     *
     * @return \Illuminate\Bus\Batch
     *
     * @throws \Throwable
     */
    public function dispatch()
    {}

    /**
     * Dispatch the batch after the response is sent to the browser.
     *
     * @return \Illuminate\Bus\Batch
     */
    public function dispatchAfterResponse()
    {}

    /**
     * Dispatch an existing batch.
     *
     * @param  \Illuminate\Bus\Batch  $batch
     * @return void
     *
     * @throws \Throwable
     */
    protected function dispatchExistingBatch($batch)
    {}

    /**
     * Dispatch the batch if the given truth test passes.
     *
     * @param  bool|\Closure  $boolean
     * @return \Illuminate\Bus\Batch|null
     */
    public function dispatchIf($boolean)
    {}

    /**
     * Dispatch the batch unless the given truth test passes.
     *
     * @param  bool|\Closure  $boolean
     * @return \Illuminate\Bus\Batch|null
     */
    public function dispatchUnless($boolean)
    {}

    /**
     * Store the batch using the given repository.
     *
     * @param  \Illuminate\Bus\BatchRepository  $repository
     * @return \Illuminate\Bus\Batch
     */
    protected function store($repository)
    {}
}
