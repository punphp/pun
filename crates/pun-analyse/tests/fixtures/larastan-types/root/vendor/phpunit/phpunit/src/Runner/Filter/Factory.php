<?php declare(strict_types=1);
/*
 * This file is part of PHPUnit.
 *
 * (c) Sebastian Bergmann <sebastian@phpunit.de>
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */
namespace PHPUnit\Runner\Filter;

use function assert;
use FilterIterator;
use Iterator;
use PHPUnit\Framework\Test;
use PHPUnit\Framework\TestSuite;
use RecursiveFilterIterator;
use RecursiveIterator;

/**
 * @no-named-arguments Parameter names are not covered by the backward compatibility promise for PHPUnit
 *
 * @internal This class is not covered by the backward compatibility promise for PHPUnit
 */
final class Factory
{
    /**
     * @var list<array{className: class-string<RecursiveFilterIterator<int, Test, RecursiveIterator<int, Test>>>, argument: list<non-empty-string>|non-empty-string}>
     */
    private array $filters = [];

    /**
     * @param list<non-empty-string> $testIds
     */
    public function addTestIdFilter(array $testIds): void
    {}

    /**
     * @param list<non-empty-string> $groups
     */
    public function addIncludeGroupFilter(array $groups): void
    {}

    /**
     * @param list<non-empty-string> $groups
     */
    public function addExcludeGroupFilter(array $groups): void
    {}

    /**
     * @param non-empty-string $name
     */
    public function addIncludeNameFilter(string $name): void
    {}

    /**
     * @param non-empty-string $name
     */
    public function addExcludeNameFilter(string $name): void
    {}

    /**
     * @param Iterator<int, Test> $iterator
     *
     * @return FilterIterator<int, Test, Iterator<int, Test>>
     */
    public function factory(Iterator $iterator, TestSuite $suite): FilterIterator
    {}
}
