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

/**
 * @no-named-arguments Parameter names are not covered by the backward compatibility promise for PHPUnit
 *
 * @internal This class is not covered by the backward compatibility promise for PHPUnit
 */
final class DeferringDispatcher implements SubscribableDispatcher
{
    private readonly SubscribableDispatcher $dispatcher;
    private readonly SubscribableDispatcher $collectedEventsDispatcher;
    private EventCollection $events;
    private bool $recording                   = true;
    private ?EventCollection $collectedEvents = null;

    public function __construct(SubscribableDispatcher $dispatcher, SubscribableDispatcher $collectedEventsDispatcher)
    {}

    public function registerTracer(Tracer\Tracer $tracer): void
    {}

    public function registerSubscriber(Subscriber $subscriber): void
    {}

    /**
     * Registers a subscriber that is notified of events as they are collected,
     * in addition to any subscriber that is notified when they are forwarded.
     *
     * This is for state that is consulted while a test is running, for
     * instance the deprecations that the expectations of a test are verified
     * against, which cannot wait until the events of an attempt of a retried
     * test have been collected and forwarded.
     */
    public function registerSubscriberForCollectedEvents(Subscriber $subscriber): void
    {}

    public function dispatch(Event $event): void
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

    public function flush(): void
    {}
}
