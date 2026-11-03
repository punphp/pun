<?php declare(strict_types=1);
/*
 * This file is part of PHPUnit.
 *
 * (c) Sebastian Bergmann <sebastian@phpunit.de>
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */
namespace PHPUnit\TextUI\Output;

use const PHP_EOL;
use function assert;
use PHPUnit\Event\Facade as EventFacade;
use PHPUnit\Logging\TeamCity\TeamCityLogger;
use PHPUnit\Logging\TestDox\TestResultCollection;
use PHPUnit\Runner\DirectoryDoesNotExistException;
use PHPUnit\Runner\Extension\ExtensionCapabilities;
use PHPUnit\TestRunner\TestResult\TestResult;
use PHPUnit\TextUI\CannotOpenSocketException;
use PHPUnit\TextUI\Configuration\Configuration;
use PHPUnit\TextUI\InvalidSocketException;
use PHPUnit\TextUI\Output\Compact\ProgressPrinter\ProgressPrinter as CompactProgressPrinter;
use PHPUnit\TextUI\Output\Compact\ResultPrinter as CompactResultPrinter;
use PHPUnit\TextUI\Output\Default\ProgressPrinter\ProgressPrinter as DefaultProgressPrinter;
use PHPUnit\TextUI\Output\Default\ResultPrinter as DefaultResultPrinter;
use PHPUnit\TextUI\Output\Default\UnexpectedOutputPrinter;
use PHPUnit\TextUI\Output\TestDox\ResultPrinter as TestDoxResultPrinter;
use SebastianBergmann\Timer\Duration;
use SebastianBergmann\Timer\ResourceUsageFormatter;

/**
 * @no-named-arguments Parameter names are not covered by the backward compatibility promise for PHPUnit
 *
 * @internal This class is not covered by the backward compatibility promise for PHPUnit
 */
final class Facade
{
    private static ?Printer $printer                           = null;
    private static ?CompactResultPrinter $compactResultPrinter = null;
    private static ?DefaultResultPrinter $defaultResultPrinter = null;
    private static ?TestDoxResultPrinter $testDoxResultPrinter = null;
    private static ?SummaryPrinter $summaryPrinter             = null;
    private static bool $defaultProgressPrinter                = false;

    public static function init(Configuration $configuration, ExtensionCapabilities $extensionCapabilities): Printer
    {}

    /**
     * @param ?array<class-string, TestResultCollection> $testDoxResult
     */
    public static function printResult(TestResult $result, ?array $testDoxResult, Duration $duration, bool $stackTraceForDeprecations): void
    {}

    /**
     * @throws CannotOpenSocketException
     * @throws DirectoryDoesNotExistException
     * @throws InvalidSocketException
     */
    public static function printerFor(string $target): Printer
    {}

    private static function createPrinter(Configuration $configuration): void
    {}

    private static function createProgressPrinter(Configuration $configuration): void
    {}

    private static function useDefaultProgressPrinter(Configuration $configuration): bool
    {}

    private static function createResultPrinter(Configuration $configuration): void
    {}

    private static function createSummaryPrinter(Configuration $configuration): void
    {}

    private static function createUnexpectedOutputPrinter(): void
    {}
}
