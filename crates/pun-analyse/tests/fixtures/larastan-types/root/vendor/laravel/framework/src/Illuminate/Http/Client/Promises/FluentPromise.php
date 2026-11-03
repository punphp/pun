<?php

namespace Illuminate\Http\Client\Promises;

use GuzzleHttp\Promise\PromiseInterface;
use Illuminate\Support\Traits\ForwardsCalls;

/**
 * A decorated Promise which allows for chaining callbacks.
 */
class FluentPromise implements PromiseInterface
{
    use ForwardsCalls;

    /**
     * Create a new fluent promise instance.
     *
     * @param  \GuzzleHttp\Promise\PromiseInterface  $guzzlePromise
     */
    public function __construct(protected PromiseInterface $guzzlePromise)
    {}

    #[\Override]
    public function then(?callable $onFulfilled = null, ?callable $onRejected = null): PromiseInterface
    {}

    #[\Override]
    public function otherwise(callable $onRejected): PromiseInterface
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

    #[\Override]
    public function getState(): string
    {}

    /**
     * Get the underlying Guzzle promise.
     *
     * @return \GuzzleHttp\Promise\PromiseInterface
     */
    public function getGuzzlePromise(): PromiseInterface
    {}

    /**
     * Proxy requests to the underlying promise interface and update the local promise.
     *
     * @param  string  $method
     * @param  array  $parameters
     * @return mixed
     */
    public function __call($method, $parameters)
    {}
}
