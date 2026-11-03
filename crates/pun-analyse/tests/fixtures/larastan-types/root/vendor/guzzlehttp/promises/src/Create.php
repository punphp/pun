<?php

declare(strict_types=1);

namespace GuzzleHttp\Promise;

final class Create
{
    private function __construct()
    {}

    /**
     * Returns `$value` when it is already a Guzzle promise, wraps foreign
     * thenables in a Guzzle promise, or returns a fulfilled promise for plain
     * values.
     *
     * @template TValue
     * @template TPromise of PromiseInterface<mixed, mixed> = PromiseInterface<mixed, mixed>
     *
     * @param TValue|TPromise $value Promise or value.
     *
     * @return ($value is PromiseInterface ? TPromise : FulfilledPromise<TValue, never>)
     */
    public static function promiseFor($value): PromiseInterface
    {}

    /**
     * Returns `$reason` when it is already a promise, or returns a rejected
     * promise for plain reasons.
     *
     * @template TReason
     * @template TValue = never
     * @template TPromise of PromiseInterface<mixed, mixed> = PromiseInterface<mixed, mixed>
     *
     * @param TReason|TPromise $reason Promise or reason.
     *
     * @return ($reason is PromiseInterface ? TPromise : RejectedPromise<TValue, TReason>)
     */
    public static function rejectionFor($reason): PromiseInterface
    {}

    /**
     * Returns throwable reasons as-is, or wraps non-throwable reasons in
     * `RejectionException`.
     *
     * @template TReason
     *
     * @param TReason $reason
     */
    public static function exceptionFor($reason): \Throwable
    {}

    /**
     * Returns an iterator for arrays, iterators, iterator aggregates, and
     * traversables.
     *
     * @template TKey of array-key
     * @template TValue
     *
     * @param iterable<TKey, TValue> $value
     *
     * @return \Iterator<TKey, TValue>
     */
    public static function iterFor(iterable $value): \Iterator
    {}
}
