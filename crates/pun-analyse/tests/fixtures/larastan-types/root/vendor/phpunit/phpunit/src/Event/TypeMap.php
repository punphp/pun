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

use function array_any;
use function array_key_exists;
use function class_exists;
use function class_implements;
use function in_array;
use function interface_exists;
use function sprintf;

/**
 * @no-named-arguments Parameter names are not covered by the backward compatibility promise for PHPUnit
 *
 * @internal This class is not covered by the backward compatibility promise for PHPUnit
 */
final class TypeMap
{
    /**
     * @var array<class-string, class-string>
     */
    private array $mapping = [];

    /**
     * @param class-string $subscriberInterface
     * @param class-string $eventClass
     *
     * @throws EventAlreadyAssignedException
     * @throws InvalidEventException
     * @throws InvalidSubscriberException
     * @throws SubscriberTypeAlreadyRegisteredException
     * @throws UnknownEventException
     * @throws UnknownSubscriberException
     */
    public function addMapping(string $subscriberInterface, string $eventClass): void
    {}

    public function isKnownSubscriberType(Subscriber $subscriber): bool
    {}

    public function isKnownEventType(Event $event): bool
    {}

    /**
     * @throws MapError
     *
     * @return class-string
     */
    public function map(Subscriber $subscriber): string
    {}

    /**
     * @param class-string $subscriberInterface
     *
     * @throws UnknownSubscriberException
     */
    private function ensureSubscriberInterfaceExists(string $subscriberInterface): void
    {}

    /**
     * @param class-string $eventClass
     *
     * @throws UnknownEventException
     */
    private function ensureEventClassExists(string $eventClass): void
    {}

    /**
     * @param class-string $subscriberInterface
     *
     * @throws InvalidSubscriberException
     */
    private function ensureSubscriberInterfaceExtendsInterface(string $subscriberInterface): void
    {}

    /**
     * @param class-string $eventClass
     *
     * @throws InvalidEventException
     */
    private function ensureEventClassImplementsEventInterface(string $eventClass): void
    {}

    /**
     * @param class-string $subscriberInterface
     *
     * @throws SubscriberTypeAlreadyRegisteredException
     */
    private function ensureSubscriberWasNotAlreadyRegistered(string $subscriberInterface): void
    {}

    /**
     * @param class-string $eventClass
     *
     * @throws EventAlreadyAssignedException
     */
    private function ensureEventWasNotAlreadyAssigned(string $eventClass): void
    {}
}
