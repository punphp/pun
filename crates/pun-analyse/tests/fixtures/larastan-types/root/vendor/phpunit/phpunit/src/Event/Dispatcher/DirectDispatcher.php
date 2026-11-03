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

use const PHP_EOL;
use function array_key_exists;
use function dirname;
use function sprintf;
use function str_starts_with;
use Throwable;

/**
 * @no-named-arguments Parameter names are not covered by the backward compatibility promise for PHPUnit
 *
 * @internal This class is not covered by the backward compatibility promise for PHPUnit
 */
final class DirectDispatcher implements SubscribableDispatcher
{
    private readonly TypeMap $typeMap;

    /**
     * @var array<class-string, list<Subscriber>>
     */
    private array $subscribers = [];

    /**
     * @var list<Tracer\Tracer>
     */
    private array $tracers = [];

    public function __construct(TypeMap $map)
    {}

    public function registerTracer(Tracer\Tracer $tracer): void
    {}

    /**
     * @throws MapError
     * @throws UnknownSubscriberTypeException
     */
    public function registerSubscriber(Subscriber $subscriber): void
    {}

    /**
     * @throws Throwable
     * @throws UnknownEventTypeException
     */
    public function dispatch(Event $event): void
    {}

    /**
     * @throws Throwable
     */
    public function handleThrowable(Throwable $t): void
    {}

    private function isThrowableFromThirdPartySubscriber(Throwable $t): bool
    {}
}
