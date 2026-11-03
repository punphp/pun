<?php

namespace Illuminate\Bus;

use Closure;
use Illuminate\Contracts\Bus\QueueingDispatcher;
use Illuminate\Contracts\Container\Container;
use Illuminate\Contracts\Queue\Queue;
use Illuminate\Contracts\Queue\ShouldQueue;
use Illuminate\Foundation\Bus\PendingChain;
use Illuminate\Pipeline\Pipeline;
use Illuminate\Queue\Attributes\Connection;
use Illuminate\Queue\Attributes\Delay;
use Illuminate\Queue\Attributes\Queue as QueueAttribute;
use Illuminate\Queue\InteractsWithQueue;
use Illuminate\Queue\Jobs\SyncJob;
use Illuminate\Support\Collection;
use Illuminate\Support\Queue\Concerns\ResolvesQueueRoutes;
use Illuminate\Support\Traits\ReadsClassAttributes;
use RuntimeException;

class Dispatcher implements QueueingDispatcher
{
    use ReadsClassAttributes, ResolvesQueueRoutes;

    /**
     * The container implementation.
     *
     * @var \Illuminate\Contracts\Container\Container
     */
    protected $container;

    /**
     * The pipeline instance for the bus.
     *
     * @var \Illuminate\Pipeline\Pipeline
     */
    protected $pipeline;

    /**
     * The pipes to send commands through before dispatching.
     *
     * @var array
     */
    protected $pipes = [];

    /**
     * The command to handler mapping for non-self-handling events.
     *
     * @var array
     */
    protected $handlers = [];

    /**
     * The queue resolver callback.
     *
     * @var \Closure|null
     */
    protected $queueResolver;

    /**
     * Indicates if dispatching after response is disabled.
     *
     * @var bool
     */
    protected $allowsDispatchingAfterResponses = true;

    /**
     * Create a new command dispatcher instance.
     */
    public function __construct(Container $container, ?Closure $queueResolver = null)
    {}

    /**
     * Dispatch a command to its appropriate handler.
     *
     * @param  mixed  $command
     * @return mixed
     */
    public function dispatch($command)
    {}

    /**
     * Dispatch a command to its appropriate handler in the current process.
     *
     * Queueable jobs will be dispatched to the "sync" queue.
     *
     * @param  mixed  $command
     * @param  mixed  $handler
     * @return mixed
     */
    public function dispatchSync($command, $handler = null)
    {}

    /**
     * Dispatch a command to its appropriate handler in the current process without using the synchronous queue.
     *
     * @param  mixed  $command
     * @param  mixed  $handler
     * @return mixed
     */
    public function dispatchNow($command, $handler = null)
    {}

    /**
     * Dispatch multiple commands in bulk to their appropriate handlers on the queue.
     *
     * @param  iterable  $jobs
     * @return void
     */
    public function bulk($jobs)
    {}

    /**
     * Attempt to find the batch with the given ID.
     *
     * @return \Illuminate\Bus\Batch|null
     */
    public function findBatch(string $batchId)
    {}

    /**
     * Create a new batch of queueable jobs.
     *
     * @param  \Illuminate\Support\Collection|mixed  $jobs
     * @return \Illuminate\Bus\PendingBatch
     */
    public function batch($jobs)
    {}

    /**
     * Create a new chain of queueable jobs.
     *
     * @param  \Illuminate\Support\Collection|array|null  $jobs
     * @return \Illuminate\Foundation\Bus\PendingChain
     */
    public function chain($jobs = null)
    {}

    /**
     * Determine if the given command has a handler.
     *
     * @param  mixed  $command
     * @return bool
     */
    public function hasCommandHandler($command)
    {}

    /**
     * Retrieve the handler for a command.
     *
     * @param  mixed  $command
     * @return mixed
     */
    public function getCommandHandler($command)
    {}

    /**
     * Determine if the given command should be queued.
     *
     * @param  mixed  $command
     * @return bool
     */
    protected function commandShouldBeQueued($command)
    {}

    /**
     * Dispatch a command to its appropriate handler behind a queue.
     *
     * @param  mixed  $command
     * @return mixed
     *
     * @throws \RuntimeException
     */
    public function dispatchToQueue($command)
    {}

    /**
     * Push the command onto the given queue instance.
     *
     * @param  \Illuminate\Contracts\Queue\Queue  $queue
     * @param  mixed  $command
     * @return mixed
     */
    protected function pushCommandToQueue($queue, $command)
    {}

    /**
     * Dispatch a command to its appropriate handler after the current process.
     *
     * @param  mixed  $command
     * @param  mixed  $handler
     * @return void
     */
    public function dispatchAfterResponse($command, $handler = null)
    {}

    /**
     * Set the pipes through which commands should be piped before dispatching.
     *
     * @return $this
     */
    public function pipeThrough(array $pipes)
    {}

    /**
     * Map a command to a handler.
     *
     * @return $this
     */
    public function map(array $map)
    {}

    /**
     * Allow dispatching after responses.
     *
     * @return $this
     */
    public function withDispatchingAfterResponses()
    {}

    /**
     * Disable dispatching after responses.
     *
     * @return $this
     */
    public function withoutDispatchingAfterResponses()
    {}
}
