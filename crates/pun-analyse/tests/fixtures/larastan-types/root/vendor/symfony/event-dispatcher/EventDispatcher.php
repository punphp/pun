<?php

/*
 * This file is part of the Symfony package.
 *
 * (c) Fabien Potencier <fabien@symfony.com>
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */

namespace Symfony\Component\EventDispatcher;

use Psr\EventDispatcher\StoppableEventInterface;
use Symfony\Component\EventDispatcher\Debug\WrappedListener;

/**
 * The EventDispatcherInterface is the central point of Symfony's event listener system.
 *
 * Listeners are registered on the manager and events are dispatched through the
 * manager.
 *
 * @author Guilherme Blanco <guilhermeblanco@hotmail.com>
 * @author Jonathan Wage <jonwage@gmail.com>
 * @author Roman Borschel <roman@code-factory.org>
 * @author Bernhard Schussek <bschussek@gmail.com>
 * @author Fabien Potencier <fabien@symfony.com>
 * @author Jordi Boggiano <j.boggiano@seld.be>
 * @author Jordan Alliot <jordan.alliot@gmail.com>
 * @author Nicolas Grekas <p@tchwork.com>
 */
class EventDispatcher implements EventDispatcherInterface
{
    private array $listeners = [];
    private array $sorted = [];
    private array $optimized;

    public function __construct()
    {}

    public function dispatch(object $event, ?string $eventName = null): object
    {}

    public function getListeners(?string $eventName = null): array
    {}

    public function getListenerPriority(string $eventName, callable|array $listener): ?int
    {}

    public function hasListeners(?string $eventName = null): bool
    {}

    public function addListener(string $eventName, callable|array $listener, int $priority = 0): void
    {}

    public function removeListener(string $eventName, callable|array $listener): void
    {}

    public function addSubscriber(EventSubscriberInterface $subscriber): void
    {}

    public function removeSubscriber(EventSubscriberInterface $subscriber): void
    {}

    /**
     * Triggers the listeners of an event.
     *
     * This method can be overridden to add functionality that is executed
     * for each listener.
     *
     * @param callable[] $listeners The event listeners
     * @param string     $eventName The name of the event to dispatch
     * @param object     $event     The event object to pass to the event handlers/listeners
     */
    protected function callListeners(iterable $listeners, string $eventName, object $event): void
    {}

    /**
     * Sorts the internal list of listeners for the given event by priority.
     */
    private function sortListeners(string $eventName): void
    {}

    /**
     * Optimizes the internal list of listeners for the given event by priority.
     */
    private function optimizeListeners(string $eventName): array
    {}
}
