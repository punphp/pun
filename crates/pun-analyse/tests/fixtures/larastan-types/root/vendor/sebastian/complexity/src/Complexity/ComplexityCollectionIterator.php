<?php declare(strict_types=1);
/*
 * This file is part of sebastian/complexity.
 *
 * (c) Sebastian Bergmann <sebastian@phpunit.de>
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */
namespace SebastianBergmann\Complexity;

use Iterator;

/**
 * @template-implements Iterator<int, Complexity>
 */
final class ComplexityCollectionIterator implements Iterator
{
    /**
     * @var list<Complexity>
     */
    private readonly array $items;
    private int $position = 0;

    public function __construct(ComplexityCollection $items)
    {}

    public function rewind(): void
    {}

    public function valid(): bool
    {}

    public function key(): int
    {}

    public function current(): Complexity
    {}

    public function next(): void
    {}
}
