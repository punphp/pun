<?php declare(strict_types=1);
/*
 * This file is part of sebastian/diff.
 *
 * (c) Sebastian Bergmann <sebastian@phpunit.de>
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */
namespace SebastianBergmann\Diff;

use ArrayIterator;
use IteratorAggregate;
use Traversable;

/**
 * @template-implements IteratorAggregate<int, Line>
 */
final class Chunk implements IteratorAggregate
{
    private int $start;
    private int $startRange;
    private int $end;
    private int $endRange;

    /**
     * @var list<Line>
     */
    private array $lines;

    /**
     * @param list<Line> $lines
     */
    public function __construct(int $start = 0, int $startRange = 1, int $end = 0, int $endRange = 1, array $lines = [])
    {}

    public function start(): int
    {}

    public function startRange(): int
    {}

    public function end(): int
    {}

    public function endRange(): int
    {}

    /**
     * @return list<Line>
     */
    public function lines(): array
    {}

    /**
     * @param list<Line> $lines
     */
    public function setLines(array $lines): void
    {}

    public function getIterator(): Traversable
    {}
}
