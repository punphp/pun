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

use function count;
use Countable;
use IteratorAggregate;

/**
 * @template-implements IteratorAggregate<non-negative-int, Event>
 *
 * @no-named-arguments Parameter names are not covered by the backward compatibility promise for PHPUnit
 */
final class EventCollection implements Countable, IteratorAggregate
{
    /**
     * @var list<Event>
     */
    private array $events = [];

    public function add(Event ...$events): void
    {}

    /**
     * @return list<Event>
     */
    public function asArray(): array
    {}

    public function count(): int
    {}

    public function isEmpty(): bool
    {}

    public function isNotEmpty(): bool
    {}

    public function getIterator(): EventCollectionIterator
    {}
}
