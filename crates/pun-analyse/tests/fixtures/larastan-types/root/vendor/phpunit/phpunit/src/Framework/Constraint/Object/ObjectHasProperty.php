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

use function gettype;
use function is_object;
use function is_scalar;
use function sprintf;
use ReflectionObject;

/**
 * @no-named-arguments Parameter names are not covered by the backward compatibility promise for PHPUnit
 */
final class ObjectHasProperty extends Constraint
{
    private readonly string $propertyName;

    public function __construct(string $propertyName)
    {}

    /**
     * Returns a string representation of the constraint.
     */
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
     * @param mixed $other value or object to evaluate
     */
    protected function matches(mixed $other): bool
    {}

    /**
     * Returns the description of the failure.
     *
     * The beginning of failure messages is "Failed asserting that" in most
     * cases. This method should return the second part of that sentence.
     *
     * @param mixed $other evaluated value or object
     */
    protected function failureDescription(mixed $other): string
    {}

    protected function negatedFailureDescription(mixed $other): string
    {}

    private function describe(mixed $other, string $propertyDescription): string
    {}
}
