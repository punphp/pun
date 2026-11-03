<?php declare(strict_types=1);
/*
 * This file is part of PHPUnit.
 *
 * (c) Sebastian Bergmann <sebastian@phpunit.de>
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */
namespace PHPUnit\Event;

use function assert;
use function interface_exists;
use PHPUnit\Event\Telemetry\HRTime;
use PHPUnit\Event\Telemetry\SystemGarbageCollectorStatusProvider;
use PHPUnit\Runner\DeprecationCollector\Facade as DeprecationCollector;

/**
 * @no-named-arguments Parameter names are not covered by the backward compatibility promise for PHPUnit
 *
 * @internal This class is not covered by the backward compatibility promise for PHPUnit
 */
final class Facade
{
    private static ?self $instance = null;
    private Emitter $emitter;
    private ?TypeMap $typeMap                         = null;
    private ?DeferringDispatcher $deferringDispatcher = null;
    private bool $sealed                              = false;

    public static function instance(): self
    {}

    public static function emitter(): Emitter
    {}

    public function __construct()
    {}

    /**
     * @throws EventFacadeIsSealedException
     * @throws UnknownSubscriberTypeException
     */
    public function registerSubscribers(Subscriber ...$subscribers): void
    {}

    /**
     * @throws EventFacadeIsSealedException
     * @throws UnknownSubscriberTypeException
     */
    public function registerSubscriber(Subscriber $subscriber): void
    {}

    /**
     * Registers subscribers that are notified of events while they are being
     * collected instead of dispatched, for instance for an attempt of a
     * retried test.
     *
     * @throws EventFacadeIsSealedException
     * @throws UnknownSubscriberTypeException
     */
    public function registerSubscribersForCollectedEvents(Subscriber ...$subscribers): void
    {}

    /**
     * @throws EventFacadeIsSealedException
     */
    public function registerTracer(Tracer\Tracer $tracer): void
    {}

    /**
     * @codeCoverageIgnore
     *
     * @noinspection PhpUnused
     */
    public function initForIsolation(HRTime $offset): CollectingDispatcher
    {}

    public function forward(EventCollection $events): void
    {}

    /**
     * @throws EventsAreAlreadyBeingCollectedException
     */
    public function startCollectingEvents(): void
    {}

    /**
     * @throws EventsAreNotBeingCollectedException
     */
    public function stopCollectingEvents(): EventCollection
    {}

    public function seal(): void
    {}

    private function createDispatchingEmitter(): DispatchingEmitter
    {}

    private function createTelemetrySystem(): Telemetry\System
    {}

    private function deferredDispatcher(): DeferringDispatcher
    {}

    private function typeMap(): TypeMap
    {}

    private function registerDefaultTypes(TypeMap $typeMap): void
    {}
}
