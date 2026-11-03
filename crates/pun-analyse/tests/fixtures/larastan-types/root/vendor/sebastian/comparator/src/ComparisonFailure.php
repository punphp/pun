<?php declare(strict_types=1);
/*
 * This file is part of sebastian/comparator.
 *
 * (c) Sebastian Bergmann <sebastian@phpunit.de>
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */
namespace SebastianBergmann\Comparator;

use RuntimeException;
use SebastianBergmann\Diff\Differ;
use SebastianBergmann\Diff\Output\StrictUnifiedDiffOutputBuilder;

/**
 * @no-named-arguments Parameter names are not covered by the backward compatibility promise for sebastian/comparator
 */
final class ComparisonFailure extends RuntimeException
{
    private mixed $expected;
    private mixed $actual;
    private string $expectedAsString;
    private string $actualAsString;

    /** @var positive-int */
    private int $contextLines;

    /**
     * @param positive-int $contextLines
     */
    public function __construct(mixed $expected, mixed $actual, string $expectedAsString, string $actualAsString, string $message = '', int $contextLines = 3)
    {}

    /**
     * @return array{expected: mixed, actual: mixed, expectedAsString: string, actualAsString: string, message: string, contextLines: positive-int}
     */
    public function __serialize(): array
    {}

    /**
     * @param array{expected: mixed, actual: mixed, expectedAsString: string, actualAsString: string, message: string, contextLines: positive-int} $data
     */
    public function __unserialize(array $data): void
    {}

    public function getActual(): mixed
    {}

    public function getExpected(): mixed
    {}

    public function getActualAsString(): string
    {}

    public function getExpectedAsString(): string
    {}

    public function getDiff(): string
    {}

    public function toString(): string
    {}
}
