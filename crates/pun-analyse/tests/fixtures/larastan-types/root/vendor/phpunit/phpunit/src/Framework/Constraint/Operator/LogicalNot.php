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

use const PREG_SPLIT_DELIM_CAPTURE;
use function array_map;
use function assert;
use function preg_quote;
use function preg_replace;
use function preg_split;
use PHPUnit\Framework\ExpectationFailedException;

/**
 * @no-named-arguments Parameter names are not covered by the backward compatibility promise for PHPUnit
 */
final class LogicalNot extends UnaryOperator
{
    /**
     * @deprecated https://github.com/sebastianbergmann/phpunit/issues/6686
     *
     * @return non-empty-string
     */
    public static function negate(string $string): string
    {}

    /**
     * Returns the name of this operator.
     */
    public function operator(): string
    {}

    /**
     * Returns this operator's precedence.
     *
     * @see https://www.php.net/manual/en/language.operators.precedence.php
     */
    public function precedence(): int
    {}

    /**
     * Evaluates the constraint for parameter $other. Returns true if the
     * constraint is met, false otherwise.
     *
     * @throws ExpectationFailedException
     */
    protected function matches(mixed $other): bool
    {}

    /**
     * Returns the string representation of $constraint in context of this
     * operator.
     */
    protected function operandToString(Constraint $constraint): string
    {}

    /**
     * Returns the failure description of $constraint in context of this
     * operator.
     */
    protected function operandFailureDescription(Constraint $constraint, mixed $other): string
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
}
