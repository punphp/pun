<?php

namespace Illuminate\Queue;

use Closure;
use Illuminate\Bus\Batchable;
use Illuminate\Bus\Queueable;
use Illuminate\Contracts\Container\Container;
use Illuminate\Contracts\Queue\ShouldQueue;
use Illuminate\Foundation\Bus\Dispatchable;
use Laravel\SerializableClosure\SerializableClosure;
use ReflectionFunction;

class CallQueuedClosure implements ShouldQueue
{
    use Batchable, Dispatchable, InteractsWithQueue, Queueable, SerializesModels;

    /**
     * The serializable Closure instance.
     *
     * @var \Laravel\SerializableClosure\SerializableClosure
     */
    public $closure;

    /**
     * The name assigned to the job.
     *
     * @var string|null
     */
    public $name = null;

    /**
     * The callbacks that should be executed on failure.
     *
     * @var array
     */
    public $failureCallbacks = [];

    /**
     * Indicate if the job should be deleted when models are missing.
     *
     * @var bool
     */
    public $deleteWhenMissingModels = true;

    /**
     * Create a new job instance.
     *
     * @param  \Laravel\SerializableClosure\SerializableClosure  $closure
     */
    public function __construct($closure)
    {}

    /**
     * Create a new job instance.
     *
     * @param  \Closure  $job
     * @return self
     */
    public static function create(Closure $job)
    {}

    /**
     * Execute the job.
     *
     * @param  \Illuminate\Contracts\Container\Container  $container
     * @return void
     */
    public function handle(Container $container)
    {}

    /**
     * Add a callback to be executed if the job fails.
     *
     * @param  callable  $callback
     * @return $this
     */
    public function onFailure($callback)
    {}

    /**
     * Handle a job failure.
     *
     * @param  \Throwable  $e
     * @return void
     */
    public function failed($e)
    {}

    /**
     * Get the display name for the queued job.
     *
     * @return string
     */
    public function displayName()
    {}

    /**
     * Assign a name to the job.
     *
     * @param  string  $name
     * @return $this
     */
    public function name($name)
    {}
}
