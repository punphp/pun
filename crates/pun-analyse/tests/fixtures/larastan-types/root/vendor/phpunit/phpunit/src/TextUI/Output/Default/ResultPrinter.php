<?php declare(strict_types=1);
/*
 * This file is part of PHPUnit.
 *
 * (c) Sebastian Bergmann <sebastian@phpunit.de>
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */
namespace PHPUnit\TextUI\Output\Default;

use const PHP_EOL;
use function array_keys;
use function array_merge;
use function array_reverse;
use function array_unique;
use function assert;
use function count;
use function explode;
use function ksort;
use function range;
use function sprintf;
use function str_starts_with;
use function strlen;
use function substr;
use function trim;
use PHPUnit\Event\Code\Test;
use PHPUnit\Event\Code\TestMethod;
use PHPUnit\Event\Test\AfterLastTestMethodErrored;
use PHPUnit\Event\Test\AfterLastTestMethodFailed;
use PHPUnit\Event\Test\BeforeFirstTestMethodErrored;
use PHPUnit\Event\Test\BeforeFirstTestMethodFailed;
use PHPUnit\Event\Test\ConsideredRisky;
use PHPUnit\Event\Test\DeprecationTriggered;
use PHPUnit\Event\Test\ErrorTriggered;
use PHPUnit\Event\Test\NoticeTriggered;
use PHPUnit\Event\Test\PhpDeprecationTriggered;
use PHPUnit\Event\Test\PhpNoticeTriggered;
use PHPUnit\Event\Test\PhpunitDeprecationTriggered;
use PHPUnit\Event\Test\PhpunitErrorTriggered;
use PHPUnit\Event\Test\PhpunitNoticeTriggered;
use PHPUnit\Event\Test\PhpunitWarningTriggered;
use PHPUnit\Event\Test\PhpWarningTriggered;
use PHPUnit\Event\Test\WarningTriggered;
use PHPUnit\Event\TestRunner\Issue as TestRunnerIssue;
use PHPUnit\TestRunner\TestResult\Issues\Issue;
use PHPUnit\TestRunner\TestResult\TestResult;
use PHPUnit\TextUI\Output\Printer;
use PHPUnit\Util\Sanitizer;

/**
 * @no-named-arguments Parameter names are not covered by the backward compatibility promise for PHPUnit
 *
 * @internal This class is not covered by the backward compatibility promise for PHPUnit
 */
final class ResultPrinter
{
    private readonly Printer $printer;
    private readonly bool $displayPhpunitDeprecations;
    private readonly bool $displayPhpunitErrors;
    private readonly bool $displayPhpunitNotices;
    private readonly bool $displayPhpunitWarnings;
    private readonly bool $displayTestsWithErrors;
    private readonly bool $displayTestsWithFailedAssertions;
    private readonly bool $displayRiskyTests;
    private readonly bool $displayDetailsOnIncompleteTests;
    private readonly bool $displayDetailsOnSkippedTests;
    private readonly bool $displayDetailsOnTestsThatTriggerDeprecations;
    private readonly bool $displayDetailsOnTestsThatTriggerErrors;
    private readonly bool $displayDetailsOnTestsThatTriggerNotices;
    private readonly bool $displayDetailsOnTestsThatTriggerWarnings;
    private readonly bool $displayDefectsInReverseOrder;
    private bool $listPrinted = false;

    public function __construct(Printer $printer, bool $displayPhpunitDeprecations, bool $displayPhpunitErrors, bool $displayPhpunitNotices, bool $displayPhpunitWarnings, bool $displayTestsWithErrors, bool $displayTestsWithFailedAssertions, bool $displayRiskyTests, bool $displayDetailsOnIncompleteTests, bool $displayDetailsOnSkippedTests, bool $displayDetailsOnTestsThatTriggerDeprecations, bool $displayDetailsOnTestsThatTriggerErrors, bool $displayDetailsOnTestsThatTriggerNotices, bool $displayDetailsOnTestsThatTriggerWarnings, bool $displayDefectsInReverseOrder)
    {}

    public function print(TestResult $result, bool $stackTraceForDeprecations = false): void
    {}

    private function printPhpunitErrors(TestResult $result): void
    {}

    private function printDetailsOnTestsThatTriggeredPhpunitDeprecations(TestResult $result): void
    {}

    private function printDetailsOnTestsThatTriggeredPhpunitNotices(TestResult $result): void
    {}

    private function printTestRunnerNotices(TestResult $result): void
    {}

    private function printTestRunnerWarnings(TestResult $result): void
    {}

    private function printTestRunnerDeprecations(TestResult $result): void
    {}

    /**
     * @param list<\PHPUnit\Event\TestRunner\ErrorTriggered|\PHPUnit\Event\TestRunner\PhpDeprecationTriggered|\PHPUnit\Event\TestRunner\PhpNoticeTriggered|\PHPUnit\Event\TestRunner\PhpWarningTriggered|TestRunnerIssue\DeprecationTriggered|TestRunnerIssue\NoticeTriggered|TestRunnerIssue\WarningTriggered> $events
     * @param non-empty-string                                                                                                                                                                                                                                                                                  $type
     */
    private function printIssuesTriggeredOutsideOfTests(array $events, string $type): void
    {}

    private function printDetailsOnTestsThatTriggeredPhpunitWarnings(TestResult $result): void
    {}

    private function printTestsWithErrors(TestResult $result): void
    {}

    private function printTestsWithFailedAssertions(TestResult $result): void
    {}

    private function printRiskyTests(TestResult $result): void
    {}

    private function printRetriedTests(TestResult $result): void
    {}

    private function printIncompleteTests(TestResult $result): void
    {}

    private function printSkippedTestSuites(TestResult $result): void
    {}

    private function printSkippedTests(TestResult $result): void
    {}

    /**
     * @param non-empty-string $type
     * @param list<Issue>      $issues
     */
    private function printIssueList(string $type, array $issues, bool $stackTrace = false): void
    {}

    private function printListHeaderWithNumberOfTestsAndNumberOfIssues(int $numberOfTestsWithIssues, int $numberOfIssues, string $type): void
    {}

    private function printIssueTriggeredOutsideOfTestListHeader(int $number, string $type): void
    {}

    private function printListHeaderWithNumber(int $number, string $type): void
    {}

    private function printListHeader(string $header): void
    {}

    /**
     * @param list<array{title: string, body: string}> $elements
     */
    private function printList(array $elements): void
    {}

    private function printListElement(int $number, string $title, string $body): void
    {}

    private function printIssueListElement(int $number, string $title, string $body): void
    {}

    private function name(Test $test): string
    {}

    /**
     * @param array<string,list<ConsideredRisky|DeprecationTriggered|ErrorTriggered|NoticeTriggered|PhpDeprecationTriggered|PhpNoticeTriggered|PhpunitDeprecationTriggered|PhpunitErrorTriggered|PhpunitNoticeTriggered|PhpunitWarningTriggered|PhpWarningTriggered|WarningTriggered>> $events
     *
     * @return array{numberOfTestsWithIssues: int, numberOfIssues: int, elements: list<array{title: string, body: string}>}
     */
    private function mapTestsWithIssuesEventsToElements(array $events): array
    {}

    private function testLocation(Test $test): string
    {}

    private function reasonMessage(ConsideredRisky|DeprecationTriggered|ErrorTriggered|NoticeTriggered|PhpDeprecationTriggered|PhpNoticeTriggered|PhpunitDeprecationTriggered|PhpunitErrorTriggered|PhpunitNoticeTriggered|PhpunitWarningTriggered|PhpWarningTriggered|WarningTriggered $reason, bool $single): string
    {}

    private function reasonLocation(ConsideredRisky|DeprecationTriggered|ErrorTriggered|NoticeTriggered|PhpDeprecationTriggered|PhpNoticeTriggered|PhpunitDeprecationTriggered|PhpunitErrorTriggered|PhpunitNoticeTriggered|PhpunitWarningTriggered|PhpWarningTriggered|WarningTriggered $reason, bool $single): string
    {}
}
