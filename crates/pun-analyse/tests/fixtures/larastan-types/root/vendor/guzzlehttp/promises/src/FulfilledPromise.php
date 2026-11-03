<?php

declare(strict_types=1);

namespace GuzzleHttp\Promise;

/**
 * A promise that has been fulfilled.
 *
 * Thenning off of this promise will invoke the onFulfilled callback
 * immediately and ignore other callbacks.
 *
 * @template TValue = mixed
 * @template TReason = never
 *
 * @implements PromiseInterface<TValue, TReason>
 *
 * @final
 */
class FulfilledPromise implements PromiseInterface
{
    /** @var TValue */
    private $value;

    /**
     * @param TValue $value
     */
    public function __construct($value)
    {}

    /**
     * @template TFulfilledValue = never
     * @template TFulfilledReason = never
     * @template TRejectedValue = never
     * @template TRejectedReason = never
     *
     * @param (callable(TValue): (TFulfilledValue|PromiseInterface<TFulfilledValue, TFulfilledReason>))|null $onFulfilled Invoked when the promise fulfills.
     * @param (callable(TReason): (TRejectedValue|PromiseInterface<TRejectedValue, TRejectedReason>))|null   $onRejected  Invoked when the promise is rejected.
     *
     * @return ($onFulfilled is null ? self<TValue, TReason> : PromiseInterface<TFulfilledValue, TFulfilledReason|\Throwable>)
     */
    public function then(
        ?callable $onFulfilled = null,
        ?callable $onRejected = null
    ): PromiseInterface {}

    /**
     * @param callable(TReason): mixed $onRejected Invoked when the promise is rejected.
     *
     * @return self<TValue, TReason>
     */
    public function otherwise(callable $onRejected): PromiseInterface
    {}

    public function wait(bool $unwrap = true)
    {}

    public function getState(): string
    {}

    public function resolve($value = null): void
    {}

    public function reject($reason): void
    {}

    public function cancel(): void
    {}
}
