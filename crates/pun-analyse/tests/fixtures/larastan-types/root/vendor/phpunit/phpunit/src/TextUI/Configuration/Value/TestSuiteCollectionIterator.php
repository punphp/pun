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

use function assert;
use Iterator;

/**
 * @no-named-arguments Parameter names are not covered by the backward compatibility promise for PHPUnit
 *
 * @template-implements Iterator<non-negative-int, TestSuite>
 */
final class TestSuiteCollectionIterator implements Iterator
{
    /**
     * @var list<TestSuite>
     */
    private readonly array $testSuites;

    /**
     * @var non-negative-int
     */
    private int $position = 0;

    public function __construct(TestSuiteCollection $testSuites)
    {}

    public function rewind(): void
    {}

    public function valid(): bool
    {}

    /**
     * @return non-negative-int
     */
    public function key(): int
    {}

    public function current(): TestSuite
    {}

    public function next(): void
    {}
}
