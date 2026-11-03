<?php declare(strict_types=1);
/*
 * This file is part of PHPUnit.
 *
 * (c) Sebastian Bergmann <sebastian@phpunit.de>
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */
namespace PHPUnit\TextUI\Output\TestDox;

use const PHP_EOL;
use function array_map;
use function assert;
use function explode;
use function implode;
use function preg_match;
use function preg_split;
use function rtrim;
use function sprintf;
use function str_starts_with;
use function trim;
use PHPUnit\Event\Code\Throwable;
use PHPUnit\Event\Test\AfterLastTestMethodErrored;
use PHPUnit\Event\Test\BeforeFirstTestMethodErrored;
use PHPUnit\Framework\TestStatus\TestStatus;
use PHPUnit\Logging\TestDox\TestResult as TestDoxTestResult;
use PHPUnit\Logging\TestDox\TestResultCollection;
use PHPUnit\TestRunner\TestResult\TestResult;
use PHPUnit\TextUI\Output\Printer;
use PHPUnit\Util\Color;
use PHPUnit\Util\Sanitizer;

/**
 * @no-named-arguments Parameter names are not covered by the backward compatibility promise for PHPUnit
 *
 * @internal This class is not covered by the backward compatibility promise for PHPUnit
 */
final readonly class ResultPrinter
{
    private Printer $printer;
    private bool $colors;
    private int $columns;
    private bool $printSummary;

    public function __construct(Printer $printer, bool $colors, int $columns, bool $printSummary)
    {}

    /**
     * @param array<class-string, TestResultCollection> $tests
     */
    public function print(TestResult $result, array $tests): void
    {}

    /**
     * @param array<class-string, TestResultCollection> $tests
     */
    private function doPrint(array $tests, bool $onlySummary): void
    {}

    private function printPrettifiedClassName(string $prettifiedClassName): void
    {}

    private function printTestResult(TestDoxTestResult $test): void
    {}

    private function printTestResultHeader(TestDoxTestResult $test): void
    {}

    private function printTestResultBody(TestDoxTestResult $test): void
    {}

    private function printTestResultBodyStart(TestDoxTestResult $test): void
    {}

    private function printTestResultBodyEnd(TestDoxTestResult $test): void
    {}

    private function printThrowable(TestStatus $status, Throwable $throwable): void
    {}

    /**
     * @return array{message: string, diff: string}
     */
    private function colorizeMessageAndDiff(string $buffer, string $style): array
    {}

    private function formatStackTrace(string $stackTrace): string
    {}

    private function prefixLines(string $prefix, string $message): string
    {}

    /**
     * @param 'default'|'diff'|'last'|'message'|'start'|'trace' $type
     */
    private function prefixFor(string $type, TestStatus $status): string
    {}

    /**
     * @return non-empty-string
     */
    private function colorFor(TestStatus $status): string
    {}

    private function messageColorFor(TestStatus $status): string
    {}

    private function symbolFor(TestStatus $status): string
    {}

    /**
     * @param 'after-last-test'|'before-first-test'                                            $type
     * @param array<non-empty-string, AfterLastTestMethodErrored|BeforeFirstTestMethodErrored> $errors
     */
    private function printBeforeClassOrAfterClassErrors(string $type, array $errors): void
    {}
}
