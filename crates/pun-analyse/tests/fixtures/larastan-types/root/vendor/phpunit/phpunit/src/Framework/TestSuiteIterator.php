<?php declare(strict_types=1);
/*
 * This file is part of PHPUnit.
 *
 * (c) Sebastian Bergmann <sebastian@phpunit.de>
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */
namespace PHPUnit\Framework;

use function assert;
use RecursiveIterator;

/**
 * @template-implements RecursiveIterator<non-negative-int, Test>
 *
 * @no-named-arguments Parameter names are not covered by the backward compatibility promise for PHPUnit
 *
 * @internal This class is not covered by the backward compatibility promise for PHPUnit
 */
final class TestSuiteIterator implements RecursiveIterator
{
    /**
     * @var list<Test>
     */
    private readonly array $tests;

    /**
     * @var non-negative-int
     */
    private int $position = 0;

    public function __construct(TestSuite $testSuite)
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

    public function current(): Test
    {}

    public function next(): void
    {}

    /**
     * @throws NoChildTestSuiteException
     */
    public function getChildren(): self
    {}

    public function hasChildren(): bool
    {}
}
