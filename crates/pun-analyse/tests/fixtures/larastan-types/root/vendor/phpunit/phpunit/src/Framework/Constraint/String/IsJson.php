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

use const JSON_ERROR_CTRL_CHAR;
use const JSON_ERROR_DEPTH;
use const JSON_ERROR_NONE;
use const JSON_ERROR_STATE_MISMATCH;
use const JSON_ERROR_SYNTAX;
use const JSON_ERROR_UTF8;
use function is_string;
use function json_decode;
use function json_last_error;
use function json_validate;
use function sprintf;

/**
 * @no-named-arguments Parameter names are not covered by the backward compatibility promise for PHPUnit
 */
final class IsJson extends Constraint
{
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
     */
    protected function matches(mixed $other): bool
    {}

    /**
     * Returns the description of the failure.
     *
     * The beginning of failure messages is "Failed asserting that" in most
     * cases. This method should return the second part of that sentence.
     */
    protected function failureDescription(mixed $other): string
    {}

    /**
     * When this constraint is negated, the failure only occurs for a value
     * that *is* valid JSON, so the parse-error detail that the affirmative
     * description carries is not applicable here.
     */
    protected function negatedFailureDescription(mixed $other): string
    {}

    private function determineJsonError(string $json): string
    {}
}
