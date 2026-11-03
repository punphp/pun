<?php declare(strict_types=1);
/*
 * This file is part of PHPUnit.
 *
 * (c) Sebastian Bergmann <sebastian@phpunit.de>
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */
namespace PHPUnit\TextUI\Output\Default\ProgressPrinter;

use function floor;
use function sprintf;
use function str_repeat;
use function strlen;
use PHPUnit\Event\Facade;
use PHPUnit\Event\Test\DeprecationTriggered;
use PHPUnit\Event\Test\Errored;
use PHPUnit\Event\Test\ErrorTriggered;
use PHPUnit\Event\Test\NoticeTriggered;
use PHPUnit\Event\Test\PhpDeprecationTriggered;
use PHPUnit\Event\Test\PhpNoticeTriggered;
use PHPUnit\Event\Test\PhpunitWarningTriggered;
use PHPUnit\Event\Test\PhpWarningTriggered;
use PHPUnit\Event\Test\WarningTriggered;
use PHPUnit\Event\TestRunner\ChildProcessErrored;
use PHPUnit\Event\TestRunner\ExecutionStarted;
use PHPUnit\Event\TestSuite\Started as TestSuiteStarted;
use PHPUnit\Framework\TestStatus\TestStatus;
use PHPUnit\TextUI\Configuration\Source;
use PHPUnit\TextUI\Configuration\SourceFilter;
use PHPUnit\TextUI\Output\Printer;
use PHPUnit\Util\Color;

/**
 * @no-named-arguments Parameter names are not covered by the backward compatibility promise for PHPUnit
 *
 * @internal This class is not covered by the backward compatibility promise for PHPUnit
 */
final class ProgressPrinter
{
    private readonly Printer $printer;
    private readonly bool $colors;
    private readonly int $numberOfColumns;
    private readonly Source $source;
    private int $column               = 0;
    private int $numberOfTests        = 0;
    private int $numberOfTestsWidth   = 0;
    private int $maxColumn            = 0;
    private int $numberOfTestsRun     = 0;
    private ?TestStatus $status       = null;
    private bool $prepared            = false;
    private bool $childProcessErrored = false;

    /**
     * When a before-first-test method errors or fails, none of the tests of
     * its test class, the test class that was started last, are run.
     */
    private int $numberOfTestsOfTestClass = 0;

    public function __construct(Printer $printer, Facade $facade, bool $colors, int $numberOfColumns, Source $source)
    {}

    public function testRunnerExecutionStarted(ExecutionStarted $event): void
    {}

    public function testSuiteStarted(TestSuiteStarted $event): void
    {}

    public function beforeTestClassMethodErrored(): void
    {}

    public function beforeTestClassMethodFailed(): void
    {}

    public function testPrepared(): void
    {}

    public function testSkipped(): void
    {}

    public function testSuiteSkipped(int $countTests): void
    {}

    public function testMarkedIncomplete(): void
    {}

    public function testTriggeredNotice(NoticeTriggered $event): void
    {}

    public function testTriggeredPhpNotice(PhpNoticeTriggered $event): void
    {}

    public function testTriggeredDeprecation(DeprecationTriggered $event): void
    {}

    public function testTriggeredPhpDeprecation(PhpDeprecationTriggered $event): void
    {}

    public function testTriggeredPhpunitDeprecation(): void
    {}

    public function testTriggeredPhpunitNotice(): void
    {}

    public function testConsideredRisky(): void
    {}

    public function testTriggeredWarning(WarningTriggered $event): void
    {}

    public function testTriggeredPhpWarning(PhpWarningTriggered $event): void
    {}

    public function testTriggeredPhpunitWarning(PhpunitWarningTriggered $event): void
    {}

    public function testTriggeredError(ErrorTriggered $event): void
    {}

    public function testFailed(): void
    {}

    public function testErrored(Errored $event): void
    {}

    public function testFinished(): void
    {}

    public function childProcessErrored(ChildProcessErrored $event): void
    {}

    private function registerSubscribers(Facade $facade): void
    {}

    private function updateTestStatus(TestStatus $status): void
    {}

    private function printProgressForSuccess(): void
    {}

    private function printProgressForSkipped(): void
    {}

    private function printProgressForIncomplete(): void
    {}

    private function printProgressForNotice(): void
    {}

    private function printProgressForDeprecation(): void
    {}

    private function printProgressForRisky(): void
    {}

    private function printProgressForWarning(): void
    {}

    private function printProgressForFailure(): void
    {}

    private function printProgressForError(): void
    {}

    /**
     * @param non-empty-string $color
     * @param non-empty-string $progress
     */
    private function printProgressWithColor(string $color, string $progress): void
    {}

    /**
     * @param non-empty-string $progress
     */
    private function printProgress(string $progress): void
    {}
}
