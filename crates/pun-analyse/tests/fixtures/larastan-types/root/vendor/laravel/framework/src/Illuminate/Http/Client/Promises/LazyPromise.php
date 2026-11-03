<?php

namespace Illuminate\Http\Client\Promises;

use Closure;
use GuzzleHttp\Promise\PromiseInterface;
use RuntimeException;

class LazyPromise implements PromiseInterface
{
    /**
     * The callbacks to execute after the Guzzle Promise has been built.
     *
     * @var list<callable>
     */
    protected array $pending = [];

    /**
     * The promise built by the creator.
     *
     * @var \GuzzleHttp\Promise\PromiseInterface
     */
    protected PromiseInterface $guzzlePromise;

    /**
     * Create a new lazy promise instance.
     *
     * @param  (\Closure(): \GuzzleHttp\Promise\PromiseInterface)  $promiseBuilder  The callback to build a new PromiseInterface.
     */
    public function __construct(protected Closure $promiseBuilder)
    {}

    /**
     * Build the promise from the promise builder.
     *
     * @return \GuzzleHttp\Promise\PromiseInterface
     *
     * @throws \RuntimeException If the promise has already been built
     */
    public function buildPromise(): PromiseInterface
    {}

    #[\Override]
    public function then(?callable $onFulfilled = null, ?callable $onRejected = null): PromiseInterface
    {}

    #[\Override]
    public function otherwise(callable $onRejected): PromiseInterface
    {}

    #[\Override]
    public function getState(): string
    {}

    #[\Override]
    public function resolve($value = null): void
    {}

    #[\Override]
    public function reject($reason): void
    {}

    #[\Override]
    public function cancel(): void
    {}

    #[\Override]
    public function wait(bool $unwrap = true)
    {}

    /**
     * Determine if the promise has been created from the promise builder.
     *
     * @return bool
     */
    public function promiseNeedsBuilt(): bool
    {}
}
