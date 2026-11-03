<?php declare(strict_types=1);
/*
 * This file is part of PHPUnit.
 *
 * (c) Sebastian Bergmann <sebastian@phpunit.de>
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */
namespace PHPUnit\Event\TestData;

use function assert;
use Iterator;

/**
 * @template-implements Iterator<non-negative-int, TestData>
 *
 * @no-named-arguments Parameter names are not covered by the backward compatibility promise for PHPUnit
 */
final class TestDataCollectionIterator implements Iterator
{
    /**
     * @var list<TestData>
     */
    private readonly array $data;

    /**
     * @var non-negative-int
     */
    private int $position = 0;

    /**
     * @internal This method is not covered by the backward compatibility promise for PHPUnit
     */
    public function __construct(TestDataCollection $data)
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

    public function current(): TestData
    {}

    public function next(): void
    {}
}
