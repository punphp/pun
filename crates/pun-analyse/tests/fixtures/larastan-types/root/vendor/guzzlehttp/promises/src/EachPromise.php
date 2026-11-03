<?php

declare(strict_types=1);

namespace GuzzleHttp\Promise;

/**
 * Represents a promise that iterates over many promises and invokes
 * side-effect functions in the process.
 *
 * @template TKey of array-key
 * @template TValue
 * @template TReason
 *
 * @implements PromisorInterface<mixed, mixed>
 *
 * @final
 */
class EachPromise implements PromisorInterface
{
    use NonSerializableTrait;

    /** @var array<int, PromiseInterface<mixed, mixed>>|null */
    private ?array $pending = [];

    private int $nextPendingIndex = 0;

    /** @var \Iterator<TKey, TValue|PromiseInterface<TValue, TReason>>|null */
    private ?\Iterator $iterable;

    /** @var (callable(int): int)|int|null */
    private $concurrency;

    /** @var (callable(TValue, TKey, PromiseInterface<mixed, mixed>): mixed)|null */
    private $onFulfilled;

    /** @var (callable(TReason, TKey, PromiseInterface<mixed, mixed>): mixed)|null */
    private $onRejected;

    /** @var Promise<mixed, mixed>|null */
    private ?Promise $aggregate = null;

    private ?bool $mutex = null;

    private bool $stepWhileLocked = false;

    /**
     * Configuration hash can include the following key value pairs:
     *
     * - fulfilled: (callable) Invoked when a promise fulfills. The function
     *   is invoked with three arguments: the fulfillment value, the index
     *   position from the iterable list of the promise, and the aggregate
     *   promise that manages all of the promises. The aggregate promise may
     *   be resolved from within the callback to short-circuit the promise.
     * - rejected: (callable) Invoked when a promise is rejected. The
     *   function is invoked with three arguments: the rejection reason, the
     *   index position from the iterable list of the promise, and the
     *   aggregate promise that manages all of the promises. The aggregate
     *   promise may be resolved from within the callback to short-circuit
     *   the promise.
     * - concurrency: (integer) Pass this configuration option to limit the
     *   allowed number of outstanding concurrently executing promises,
     *   creating a capped pool of promises. There is no limit by default.
     *
     * @param iterable<TKey, TValue|PromiseInterface<TValue, TReason>> $iterable Promises or values to iterate.
     * @param array{
     *     fulfilled?: callable(TValue, TKey, PromiseInterface<mixed, mixed>): mixed,
     *     rejected?: callable(TReason, TKey, PromiseInterface<mixed, mixed>): mixed,
     *     concurrency?: int|(callable(int): int)
     * } $config Configuration options
     */
    public function __construct(iterable $iterable, array $config = [])
    {}

    /**
     * @return PromiseInterface<mixed, mixed>
     */
    public function promise(): PromiseInterface
    {}

    private function createPromise(): void
    {}

    private function refillPending(): void
    {}

    private function addPending(): bool
    {}

    private function advanceIterator(): bool
    {}

    private function step(int $idx): void
    {}

    /** @phpstan-impure */
    private function checkIfFinished(): bool
    {}
}
