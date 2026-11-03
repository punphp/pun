<?php declare(strict_types=1);
/*
 * This file is part of PHPUnit.
 *
 * (c) Sebastian Bergmann <sebastian@phpunit.de>
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */
namespace PHPUnit\Framework\Constraint;

use function count;
use function is_countable;
use function iterator_count;
use function sprintf;
use EmptyIterator;
use Generator;
use Iterator;
use IteratorAggregate;
use PHPUnit\Framework\Exception;
use PHPUnit\Framework\GeneratorNotSupportedException;
use SebastianBergmann\RecursionContext\Context;
use Traversable;

/**
 * @no-named-arguments Parameter names are not covered by the backward compatibility promise for PHPUnit
 */
class Count extends Constraint
{
    private readonly int $expectedCount;

    public function __construct(int $expected)
    {}

    public function toString(): string
    {}

    /**
     * Returns the negated string representation of the constraint.
     */
    protected function negatedToString(): string
    {}

    /**
     * Evaluates the constraint for parameter $other. Returns true if the
     * constraint is met, false otherwise.
     *
     * @throws Exception
     */
    protected function matches(mixed $other): bool
    {}

    /**
     * @throws Exception
     */
    protected function getCountOf(mixed $other): ?int
    {}

    /**
     * Returns the description of the failure.
     *
     * The beginning of failure messages is "Failed asserting that" in most
     * cases. This method should return the second part of that sentence.
     *
     * @throws Exception
     */
    protected function failureDescription(mixed $other): string
    {}

    /**
     * @throws Exception
     */
    protected function negatedFailureDescription(mixed $other): string
    {}
}
