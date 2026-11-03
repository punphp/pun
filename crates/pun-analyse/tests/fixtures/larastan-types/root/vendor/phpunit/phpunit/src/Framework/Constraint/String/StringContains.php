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

use function is_string;
use function mb_detect_encoding;
use function mb_stripos;
use function mb_strtolower;
use function sprintf;
use function str_contains;
use function strlen;
use function strtr;
use PHPUnit\Util\Exporter;

/**
 * @no-named-arguments Parameter names are not covered by the backward compatibility promise for PHPUnit
 */
final class StringContains extends Constraint
{
    private readonly string $needle;
    private readonly bool $ignoreCase;
    private readonly bool $ignoreLineEndings;

    public function __construct(string $needle, bool $ignoreCase = false, bool $ignoreLineEndings = false)
    {}

    /**
     * Returns a string representation of the constraint.
     */
    public function toString(): string
    {}

    public function failureDescription(mixed $other): string
    {}

    /**
     * Returns the negated string representation of the constraint.
     *
     * Authoring the negation here keeps the needle and the haystack out of
     * the negation entirely.
     */
    protected function negatedToString(): string
    {}

    protected function negatedFailureDescription(mixed $other): string
    {}

    /**
     * Evaluates the constraint for parameter $other. Returns true if the
     * constraint is met, false otherwise.
     */
    protected function matches(mixed $other): bool
    {}

    private function needleAsString(): string
    {}

    private function haystackAsString(mixed $other): string
    {}

    private function detectedEncoding(mixed $other): string
    {}

    private function haystackLength(mixed $haystack): int
    {}

    private function normalizeLineEndings(string $string): string
    {}
}
