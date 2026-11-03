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

use function array_filter;
use function array_merge;
use function array_reverse;
use function array_values;
use function count;
use function usort;
use Countable;
use IteratorAggregate;

/**
 * @template-implements IteratorAggregate<int, Complexity>
 *
 * @psalm-immutable
 */
final readonly class ComplexityCollection implements Countable, IteratorAggregate
{
    /**
     * @var list<Complexity>
     */
    private array $items;

    public static function fromList(Complexity ...$items): self
    {}

    /**
     * @param list<Complexity> $items
     */
    private function __construct(array $items)
    {}

    /**
     * @return list<Complexity>
     */
    public function asArray(): array
    {}

    public function getIterator(): ComplexityCollectionIterator
    {}

    /**
     * @return non-negative-int
     */
    public function count(): int
    {}

    public function isEmpty(): bool
    {}

    /**
     * @return non-negative-int
     */
    public function cyclomaticComplexity(): int
    {}

    public function isFunction(): self
    {}

    public function isMethod(): self
    {}

    public function mergeWith(self $other): self
    {}

    public function sortByDescendingCyclomaticComplexity(): self
    {}
}
