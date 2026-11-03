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

use const DIRECTORY_SEPARATOR;
use const PHP_EOL;
use function array_slice;
use function assert;
use function explode;
use function implode;
use function is_string;
use function preg_last_error_msg;
use function preg_match;
use function preg_quote;
use function preg_replace;
use function sprintf;
use function str_replace;
use function strlen;
use function strpos;
use function strtr;
use function substr;
use PHPUnit\Framework\Exception as FrameworkException;
use PHPUnit\Util\DifferBuilder;
use SebastianBergmann\Diff\Differ;

/**
 * @no-named-arguments Parameter names are not covered by the backward compatibility promise for PHPUnit
 */
final class StringMatchesFormatDescription extends Constraint
{
    private readonly string $formatDescription;

    public function __construct(string $formatDescription)
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
     * @throws FrameworkException
     */
    protected function matches(mixed $other): bool
    {}

    protected function failureDescription(mixed $other): string
    {}

    protected function negatedFailureDescription(mixed $other): string
    {}

    /**
     * Returns a cleaned up diff.
     *
     * The expected string can contain placeholders like %s and %d.
     * By using 'diff' such placeholders compared to the real output will
     * always be different, although we don't want to show them as different.
     * This method removes the expected differences by figuring out if a difference
     * is allowed by the use of a placeholder.
     *
     * For %A and %a multiline placeholders that can match across multiple lines,
     * we use anchor lookahead: find the next non-multiline expected line and search
     * for it in the actual output to determine how many actual lines the placeholder
     * consumed, keeping expected and actual in sync.
     */
    protected function additionalFailureDescription(mixed $other): string
    {}

    private function regularExpressionForFormatDescription(string $string): string
    {}

    private function isMultilineMatch(string $line): bool
    {}

    /**
     * @param list<string> $expected
     *
     * @return null|array{int, string}
     */
    private function findNextAnchor(array $expected, int $startIdx): ?array
    {}

    /**
     * @param list<string> $actual
     */
    private function findAnchorInActual(string $anchorLine, array $actual, int $startIdx): ?int
    {}

    private function convertNewlines(string $text): string
    {}

    private function differ(): Differ
    {}
}
