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

use function array_map;
use function count;

/**
 * @no-named-arguments Parameter names are not covered by the backward compatibility promise for PHPUnit
 */
abstract class BinaryOperator extends Operator
{
    /**
     * @var list<Constraint>
     */
    private readonly array $constraints;

    protected function __construct(mixed ...$constraints)
    {}

    /**
     * Returns the number of operands (constraints).
     */
    final public function arity(): int
    {}

    /**
     * Returns a string representation of the constraint.
     */
    public function toString(): string
    {}

    /**
     * Counts the number of constraint elements.
     */
    public function count(): int
    {}

    /**
     * @return list<Constraint>
     */
    final protected function constraints(): array
    {}

    /**
     * Returns true if the $constraint needs to be wrapped with braces.
     */
    final protected function constraintNeedsParentheses(Constraint $constraint): bool
    {}

    /**
     * Reduces the sub-expression starting at $this by skipping degenerate
     * sub-expression and returns first descendant constraint that starts
     * a non-reducible sub-expression.
     *
     * See Constraint::reduce() for more.
     */
    protected function reduce(): Constraint
    {}

    /**
     * Returns string representation of given operand in context of this operator.
     */
    private function constraintToString(Constraint $constraint, int $position): string
    {}
}
