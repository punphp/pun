<?php

declare(strict_types=1);

namespace GuzzleHttp\Promise;

/**
 * A promise that has been rejected.
 *
 * Thenning off of this promise will invoke the onRejected callback
 * immediately and ignore other callbacks.
 *
 * @template TValue = never
 * @template TReason = mixed
 *
 * @implements PromiseInterface<TValue, TReason>
 *
 * @final
 */
class RejectedPromise implements PromiseInterface
{
    /** @var TReason */
    private $reason;

    /**
     * @param TReason $reason
     */
    public function __construct($reason)
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
     * @return ($onRejected is null ? self<TValue, TReason> : PromiseInterface<TRejectedValue, TRejectedReason|\Throwable>)
     */
    public function then(
        ?callable $onFulfilled = null,
        ?callable $onRejected = null
    ): PromiseInterface {}

    /**
     * @template TRejectedValue = never
     * @template TRejectedReason = never
     *
     * @param callable(TReason): (TRejectedValue|PromiseInterface<TRejectedValue, TRejectedReason>) $onRejected Invoked when the promise is rejected.
     *
     * @return PromiseInterface<TRejectedValue, TRejectedReason|\Throwable>
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
