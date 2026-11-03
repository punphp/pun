<?php declare(strict_types=1);
/*
 * This file is part of PHPUnit.
 *
 * (c) Sebastian Bergmann <sebastian@phpunit.de>
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */
namespace PHPUnit\TextUI\Output\Compact;

use const PHP_EOL;
use function assert;
use function implode;
use function ksort;
use function sprintf;
use function trim;
use PHPUnit\Event\Test\PhpunitDeprecationTriggered;
use PHPUnit\Event\Test\PhpunitErrorTriggered;
use PHPUnit\Event\Test\PhpunitNoticeTriggered;
use PHPUnit\Event\Test\PhpunitWarningTriggered;
use PHPUnit\Event\TestRunner\ErrorTriggered;
use PHPUnit\Event\TestRunner\Issue as TestRunnerIssue;
use PHPUnit\Event\TestRunner\PhpDeprecationTriggered;
use PHPUnit\Event\TestRunner\PhpNoticeTriggered;
use PHPUnit\Event\TestRunner\PhpWarningTriggered;
use PHPUnit\TestRunner\TestResult\Issues\Issue;
use PHPUnit\TestRunner\TestResult\TestResult;
use PHPUnit\TextUI\Output\Printer;

/**
 * @no-named-arguments Parameter names are not covered by the backward compatibility promise for PHPUnit
 *
 * @internal This class is not covered by the backward compatibility promise for PHPUnit
 */
final readonly class ResultPrinter
{
    private Printer $printer;
    private Renderer $renderer;
    private bool $displayDetailsOnIncompleteTests;
    private bool $displayDetailsOnSkippedTests;
    private bool $displayDetailsOnTestsThatTriggerDeprecations;
    private bool $displayDetailsOnTestsThatTriggerErrors;
    private bool $displayDetailsOnTestsThatTriggerNotices;
    private bool $displayDetailsOnTestsThatTriggerWarnings;
    private bool $displayDetailsOnPhpunitDeprecations;
    private bool $displayDetailsOnPhpunitNotices;

    public function __construct(Printer $printer, bool $displayDetailsOnIncompleteTests, bool $displayDetailsOnSkippedTests, bool $displayDetailsOnTestsThatTriggerDeprecations, bool $displayDetailsOnTestsThatTriggerErrors, bool $displayDetailsOnTestsThatTriggerNotices, bool $displayDetailsOnTestsThatTriggerWarnings, bool $displayDetailsOnPhpunitDeprecations, bool $displayDetailsOnPhpunitNotices)
    {}

    public function print(TestResult $result): void
    {}

    private function printSummaryLine(TestResult $result): void
    {}

    private function printDeprecations(TestResult $result): void
    {}

    private function printWarnings(TestResult $result): void
    {}

    private function printNotices(TestResult $result): void
    {}

    private function printTestTriggeredErrors(TestResult $result): void
    {}

    private function printPhpunitErrors(TestResult $result): void
    {}

    private function printPhpunitWarnings(TestResult $result): void
    {}

    private function printPhpunitDeprecations(TestResult $result): void
    {}

    private function printPhpunitNotices(TestResult $result): void
    {}

    private function printTimeLimitExceeded(TestResult $result): void
    {}

    private function printTestRunnerWarnings(TestResult $result): void
    {}

    private function printTestRunnerDeprecations(TestResult $result): void
    {}

    private function printTestRunnerNotices(TestResult $result): void
    {}

    /**
     * @param list<ErrorTriggered|PhpDeprecationTriggered|PhpNoticeTriggered|PhpWarningTriggered|TestRunnerIssue\DeprecationTriggered|TestRunnerIssue\NoticeTriggered|TestRunnerIssue\WarningTriggered> $events
     */
    private function printIssuesTriggeredOutsideOfTests(array $events, string $type): void
    {}

    private function printRiskyTests(TestResult $result): void
    {}

    private function printIncompleteTests(TestResult $result): void
    {}

    private function printSkippedTests(TestResult $result): void
    {}

    /**
     * @param list<Issue> $issues
     */
    private function printIssueList(string $type, array $issues): void
    {}
}
