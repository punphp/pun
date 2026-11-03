<?php declare(strict_types=1);
/*
 * This file is part of PHPUnit.
 *
 * (c) Sebastian Bergmann <sebastian@phpunit.de>
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */
namespace PHPUnit\TextUI\Configuration;

use IteratorAggregate;

/**
 * @no-named-arguments Parameter names are not covered by the backward compatibility promise for PHPUnit
 *
 * @immutable
 *
 * @template-implements IteratorAggregate<non-negative-int, Group>
 */
final readonly class GroupCollection implements IteratorAggregate
{
    /**
     * @var list<Group>
     */
    private array $groups;

    /**
     * @param list<Group> $groups
     */
    public static function fromArray(array $groups): self
    {}

    private function __construct(Group ...$groups)
    {}

    /**
     * @return list<Group>
     */
    public function asArray(): array
    {}

    /**
     * @return list<non-empty-string>
     */
    public function asArrayOfStrings(): array
    {}

    public function isEmpty(): bool
    {}

    public function getIterator(): GroupCollectionIterator
    {}
}
