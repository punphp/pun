<?php declare(strict_types=1);
/*
 * This file is part of PHPUnit.
 *
 * (c) Sebastian Bergmann <sebastian@phpunit.de>
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */
namespace PHPUnit\TestRunner\TestResult;

use function array_map;
use function array_sum;
use function assert;
use function count;
use PHPUnit\Event\Test\AfterLastTestMethodErrored;
use PHPUnit\Event\Test\AfterLastTestMethodFailed;
use PHPUnit\Event\Test\BeforeFirstTestMethodErrored;
use PHPUnit\Event\Test\BeforeFirstTestMethodFailed;
use PHPUnit\Event\Test\ConsideredRisky;
use PHPUnit\Event\Test\Errored;
use PHPUnit\Event\Test\Failed;
use PHPUnit\Event\Test\MarkedIncomplete;
use PHPUnit\Event\Test\PhpunitDeprecationTriggered;
use PHPUnit\Event\Test\PhpunitErrorTriggered;
use PHPUnit\Event\Test\PhpunitNoticeTriggered;
use PHPUnit\Event\Test\PhpunitWarningTriggered;
use PHPUnit\Event\Test\Skipped as TestSkipped;
use PHPUnit\Event\TestRunner\DeprecationTriggered as TestRunnerDeprecationTriggered;
use PHPUnit\Event\TestRunner\ErrorTriggered as TestRunnerIssueErrorTriggered;
use PHPUnit\Event\TestRunner\Issue\DeprecationTriggered as TestRunnerIssueDeprecationTriggered;
use PHPUnit\Event\TestRunner\Issue\NoticeTriggered as TestRunnerIssueNoticeTriggered;
use PHPUnit\Event\TestRunner\Issue\WarningTriggered as TestRunnerIssueWarningTriggered;
use PHPUnit\Event\TestRunner\NoticeTriggered as TestRunnerNoticeTriggered;
use PHPUnit\Event\TestRunner\PhpDeprecationTriggered as TestRunnerIssuePhpDeprecationTriggered;
use PHPUnit\Event\TestRunner\PhpNoticeTriggered as TestRunnerIssuePhpNoticeTriggered;
use PHPUnit\Event\TestRunner\PhpWarningTriggered as TestRunnerIssuePhpWarningTriggered;
use PHPUnit\Event\TestRunner\TimeLimitExceeded;
use PHPUnit\Event\TestRunner\WarningTriggered as TestRunnerWarningTriggered;
use PHPUnit\Event\TestSuite\Skipped as TestSuiteSkipped;
use PHPUnit\TestRunner\TestResult\Issues\Issue;

/**
 * @no-named-arguments Parameter names are not covered by the backward compatibility promise for PHPUnit
 *
 * @internal This class is not covered by the backward compatibility promise for PHPUnit
 */
final readonly class TestResult
{
    private int $numberOfTests;
    private int $numberOfTestsRun;
    private int $numberOfAssertions;

    /**
     * @var list<AfterLastTestMethodErrored|BeforeFirstTestMethodErrored|Errored>
     */
    private array $testErroredEvents;

    /**
     * @var list<AfterLastTestMethodFailed|BeforeFirstTestMethodFailed|Failed>
     */
    private array $testFailedEvents;

    /**
     * @var list<MarkedIncomplete>
     */
    private array $testMarkedIncompleteEvents;

    /**
     * @var list<TestSuiteSkipped>
     */
    private array $testSuiteSkippedEvents;

    /**
     * @var list<TestSkipped>
     */
    private array $testSkippedEvents;

    /**
     * @var array<string,list<ConsideredRisky>>
     */
    private array $testConsideredRiskyEvents;

    /**
     * @var array<string,list<PhpunitDeprecationTriggered>>
     */
    private array $testTriggeredPhpunitDeprecationEvents;

    /**
     * @var array<string,list<PhpunitErrorTriggered>>
     */
    private array $testTriggeredPhpunitErrorEvents;

    /**
     * @var array<string,list<PhpunitNoticeTriggered>>
     */
    private array $testTriggeredPhpunitNoticeEvents;

    /**
     * @var array<string,list<PhpunitWarningTriggered>>
     */
    private array $testTriggeredPhpunitWarningEvents;

    /**
     * @var list<TestRunnerDeprecationTriggered>
     */
    private array $testRunnerTriggeredDeprecationEvents;

    /**
     * @var list<TestRunnerNoticeTriggered>
     */
    private array $testRunnerTriggeredNoticeEvents;

    /**
     * @var list<TestRunnerWarningTriggered>
     */
    private array $testRunnerTriggeredWarningEvents;

    /**
     * @var list<TestRunnerIssueDeprecationTriggered>
     */
    private array $testRunnerTriggeredIssueDeprecationEvents;

    /**
     * @var list<TestRunnerIssueErrorTriggered>
     */
    private array $testRunnerTriggeredIssueErrorEvents;

    /**
     * @var list<TestRunnerIssueNoticeTriggered>
     */
    private array $testRunnerTriggeredIssueNoticeEvents;

    /**
     * @var list<TestRunnerIssuePhpDeprecationTriggered>
     */
    private array $testRunnerTriggeredIssuePhpDeprecationEvents;

    /**
     * @var list<TestRunnerIssuePhpNoticeTriggered>
     */
    private array $testRunnerTriggeredIssuePhpNoticeEvents;

    /**
     * @var list<TestRunnerIssuePhpWarningTriggered>
     */
    private array $testRunnerTriggeredIssuePhpWarningEvents;

    /**
     * @var list<TestRunnerIssueWarningTriggered>
     */
    private array $testRunnerTriggeredIssueWarningEvents;

    /**
     * @var list<Issue>
     */
    private array $errors;

    /**
     * @var list<Issue>
     */
    private array $deprecations;

    /**
     * @var list<Issue>
     */
    private array $notices;

    /**
     * @var list<Issue>
     */
    private array $warnings;

    /**
     * @var list<Issue>
     */
    private array $phpDeprecations;

    /**
     * @var list<Issue>
     */
    private array $phpNotices;

    /**
     * @var list<Issue>
     */
    private array $phpWarnings;

    /**
     * @var non-negative-int
     */
    private int $numberOfIssuesIgnoredByBaseline;

    /**
     * @var array{self: non-negative-int, direct: non-negative-int, indirect: non-negative-int, unknown: non-negative-int}
     */
    private array $numberOfDeprecationsByTrigger;

    /**
     * @var array<non-empty-string, positive-int>
     */
    private array $retriedTests;
    private ?TimeLimitExceeded $timeLimitExceededEvent;

    /**
     * @param list<AfterLastTestMethodErrored|BeforeFirstTestMethodErrored|Errored>                                          $testErroredEvents
     * @param list<AfterLastTestMethodFailed|BeforeFirstTestMethodFailed|Failed>                                             $testFailedEvents
     * @param array<string,list<ConsideredRisky>>                                                                            $testConsideredRiskyEvents
     * @param list<TestSuiteSkipped>                                                                                         $testSuiteSkippedEvents
     * @param list<TestSkipped>                                                                                              $testSkippedEvents
     * @param list<MarkedIncomplete>                                                                                         $testMarkedIncompleteEvents
     * @param array<string,list<PhpunitDeprecationTriggered>>                                                                $testTriggeredPhpunitDeprecationEvents
     * @param array<string,list<PhpunitErrorTriggered>>                                                                      $testTriggeredPhpunitErrorEvents
     * @param array<string,list<PhpunitNoticeTriggered>>                                                                     $testTriggeredPhpunitNoticeEvents
     * @param array<string,list<PhpunitWarningTriggered>>                                                                    $testTriggeredPhpunitWarningEvents
     * @param list<TestRunnerDeprecationTriggered>                                                                           $testRunnerTriggeredDeprecationEvents
     * @param list<TestRunnerNoticeTriggered>                                                                                $testRunnerTriggeredNoticeEvents
     * @param list<TestRunnerWarningTriggered>                                                                               $testRunnerTriggeredWarningEvents
     * @param list<TestRunnerIssueDeprecationTriggered>                                                                      $testRunnerTriggeredIssueDeprecationEvents
     * @param list<TestRunnerIssueErrorTriggered>                                                                            $testRunnerTriggeredIssueErrorEvents
     * @param list<TestRunnerIssueNoticeTriggered>                                                                           $testRunnerTriggeredIssueNoticeEvents
     * @param list<TestRunnerIssuePhpDeprecationTriggered>                                                                   $testRunnerTriggeredIssuePhpDeprecationEvents
     * @param list<TestRunnerIssuePhpNoticeTriggered>                                                                        $testRunnerTriggeredIssuePhpNoticeEvents
     * @param list<TestRunnerIssuePhpWarningTriggered>                                                                       $testRunnerTriggeredIssuePhpWarningEvents
     * @param list<TestRunnerIssueWarningTriggered>                                                                          $testRunnerTriggeredIssueWarningEvents
     * @param list<Issue>                                                                                                    $errors
     * @param list<Issue>                                                                                                    $deprecations
     * @param list<Issue>                                                                                                    $notices
     * @param list<Issue>                                                                                                    $warnings
     * @param list<Issue>                                                                                                    $phpDeprecations
     * @param list<Issue>                                                                                                    $phpNotices
     * @param list<Issue>                                                                                                    $phpWarnings
     * @param non-negative-int                                                                                               $numberOfIssuesIgnoredByBaseline
     * @param array{self: non-negative-int, direct: non-negative-int, indirect: non-negative-int, unknown: non-negative-int} $numberOfDeprecationsByTrigger
     * @param array<non-empty-string, positive-int>                                                                          $retriedTests
     */
    public function __construct(int $numberOfTests, int $numberOfTestsRun, int $numberOfAssertions, array $testErroredEvents, array $testFailedEvents, array $testConsideredRiskyEvents, array $testSuiteSkippedEvents, array $testSkippedEvents, array $testMarkedIncompleteEvents, array $testTriggeredPhpunitDeprecationEvents, array $testTriggeredPhpunitErrorEvents, array $testTriggeredPhpunitNoticeEvents, array $testTriggeredPhpunitWarningEvents, array $testRunnerTriggeredDeprecationEvents, array $testRunnerTriggeredNoticeEvents, array $testRunnerTriggeredWarningEvents, array $testRunnerTriggeredIssueDeprecationEvents, array $testRunnerTriggeredIssueErrorEvents, array $testRunnerTriggeredIssueNoticeEvents, array $testRunnerTriggeredIssuePhpDeprecationEvents, array $testRunnerTriggeredIssuePhpNoticeEvents, array $testRunnerTriggeredIssuePhpWarningEvents, array $testRunnerTriggeredIssueWarningEvents, array $errors, array $deprecations, array $notices, array $warnings, array $phpDeprecations, array $phpNotices, array $phpWarnings, int $numberOfIssuesIgnoredByBaseline, array $numberOfDeprecationsByTrigger, array $retriedTests = [], ?TimeLimitExceeded $timeLimitExceededEvent = null)
    {}

    /**
     * @phpstan-assert-if-true !array{} $this->retriedTests
     */
    public function hasRetriedTests(): bool
    {}

    /**
     * @return array<non-empty-string, positive-int>
     */
    public function retriedTests(): array
    {}

    /**
     * @phpstan-assert-if-true !null $this->timeLimitExceededEvent
     */
    public function wasTimeLimitExceeded(): bool
    {}

    public function timeLimitExceededEvent(): TimeLimitExceeded
    {}

    public function numberOfTestsRun(): int
    {}

    public function numberOfAssertions(): int
    {}

    /**
     * @return list<AfterLastTestMethodErrored|BeforeFirstTestMethodErrored|Errored>
     */
    public function testErroredEvents(): array
    {}

    public function numberOfTestErroredEvents(): int
    {}

    public function hasTestErroredEvents(): bool
    {}

    /**
     * @return list<AfterLastTestMethodFailed|BeforeFirstTestMethodFailed|Failed>
     */
    public function testFailedEvents(): array
    {}

    public function numberOfTestFailedEvents(): int
    {}

    public function hasTestFailedEvents(): bool
    {}

    /**
     * @return array<string,list<ConsideredRisky>>
     */
    public function testConsideredRiskyEvents(): array
    {}

    public function numberOfTestsWithTestConsideredRiskyEvents(): int
    {}

    public function hasTestConsideredRiskyEvents(): bool
    {}

    /**
     * @return list<TestSuiteSkipped>
     */
    public function testSuiteSkippedEvents(): array
    {}

    public function numberOfTestSkippedByTestSuiteSkippedEvents(): int
    {}

    public function hasTestSuiteSkippedEvents(): bool
    {}

    /**
     * @return list<TestSkipped>
     */
    public function testSkippedEvents(): array
    {}

    public function numberOfTestSkippedEvents(): int
    {}

    public function hasTestSkippedEvents(): bool
    {}

    /**
     * @return list<MarkedIncomplete>
     */
    public function testMarkedIncompleteEvents(): array
    {}

    public function numberOfTestMarkedIncompleteEvents(): int
    {}

    public function hasTestMarkedIncompleteEvents(): bool
    {}

    /**
     * @return array<string,list<PhpunitDeprecationTriggered>>
     */
    public function testTriggeredPhpunitDeprecationEvents(): array
    {}

    public function numberOfTestsWithTestTriggeredPhpunitDeprecationEvents(): int
    {}

    public function hasTestTriggeredPhpunitDeprecationEvents(): bool
    {}

    /**
     * @return array<string,list<PhpunitErrorTriggered>>
     */
    public function testTriggeredPhpunitErrorEvents(): array
    {}

    public function numberOfTestsWithTestTriggeredPhpunitErrorEvents(): int
    {}

    public function hasTestTriggeredPhpunitErrorEvents(): bool
    {}

    /**
     * @return array<string,list<PhpunitNoticeTriggered>>
     */
    public function testTriggeredPhpunitNoticeEvents(): array
    {}

    public function numberOfTestsWithTestTriggeredPhpunitNoticeEvents(): int
    {}

    public function hasTestTriggeredPhpunitNoticeEvents(): bool
    {}

    /**
     * @return array<string,list<PhpunitWarningTriggered>>
     */
    public function testTriggeredPhpunitWarningEvents(): array
    {}

    public function numberOfTestsWithTestTriggeredPhpunitWarningEvents(): int
    {}

    public function hasTestTriggeredPhpunitWarningEvents(): bool
    {}

    /**
     * @return list<TestRunnerDeprecationTriggered>
     */
    public function testRunnerTriggeredDeprecationEvents(): array
    {}

    public function numberOfTestRunnerTriggeredDeprecationEvents(): int
    {}

    public function hasTestRunnerTriggeredDeprecationEvents(): bool
    {}

    /**
     * @return list<TestRunnerNoticeTriggered>
     */
    public function testRunnerTriggeredNoticeEvents(): array
    {}

    public function numberOfTestRunnerTriggeredNoticeEvents(): int
    {}

    public function hasTestRunnerTriggeredNoticeEvents(): bool
    {}

    /**
     * @return list<TestRunnerWarningTriggered>
     */
    public function testRunnerTriggeredWarningEvents(): array
    {}

    public function numberOfTestRunnerTriggeredWarningEvents(): int
    {}

    public function hasTestRunnerTriggeredWarningEvents(): bool
    {}

    /**
     * @return list<TestRunnerIssueDeprecationTriggered>
     */
    public function testRunnerTriggeredIssueDeprecationEvents(): array
    {}

    public function hasTestRunnerTriggeredIssueDeprecationEvents(): bool
    {}

    /**
     * @return list<TestRunnerIssueErrorTriggered>
     */
    public function testRunnerTriggeredIssueErrorEvents(): array
    {}

    public function hasTestRunnerTriggeredIssueErrorEvents(): bool
    {}

    /**
     * @return list<TestRunnerIssueNoticeTriggered>
     */
    public function testRunnerTriggeredIssueNoticeEvents(): array
    {}

    public function hasTestRunnerTriggeredIssueNoticeEvents(): bool
    {}

    /**
     * @return list<TestRunnerIssuePhpDeprecationTriggered>
     */
    public function testRunnerTriggeredIssuePhpDeprecationEvents(): array
    {}

    public function hasTestRunnerTriggeredIssuePhpDeprecationEvents(): bool
    {}

    /**
     * @return list<TestRunnerIssuePhpNoticeTriggered>
     */
    public function testRunnerTriggeredIssuePhpNoticeEvents(): array
    {}

    public function hasTestRunnerTriggeredIssuePhpNoticeEvents(): bool
    {}

    /**
     * @return list<TestRunnerIssuePhpWarningTriggered>
     */
    public function testRunnerTriggeredIssuePhpWarningEvents(): array
    {}

    public function hasTestRunnerTriggeredIssuePhpWarningEvents(): bool
    {}

    /**
     * @return list<TestRunnerIssueWarningTriggered>
     */
    public function testRunnerTriggeredIssueWarningEvents(): array
    {}

    public function hasTestRunnerTriggeredIssueWarningEvents(): bool
    {}

    public function wasSuccessful(): bool
    {}

    public function hasIssues(): bool
    {}

    public function hasIssuesTriggeredOutsideOfTests(): bool
    {}

    public function hasTestsWithIssues(): bool
    {}

    /**
     * @return list<Issue>
     */
    public function errors(): array
    {}

    /**
     * @return list<Issue>
     */
    public function deprecations(): array
    {}

    /**
     * @return list<Issue>
     */
    public function notices(): array
    {}

    /**
     * @return list<Issue>
     */
    public function warnings(): array
    {}

    /**
     * @return list<Issue>
     */
    public function phpDeprecations(): array
    {}

    /**
     * @return list<Issue>
     */
    public function phpNotices(): array
    {}

    /**
     * @return list<Issue>
     */
    public function phpWarnings(): array
    {}

    public function hasTests(): bool
    {}

    public function hasErrors(): bool
    {}

    public function numberOfErrors(): int
    {}

    public function hasDeprecations(): bool
    {}

    public function hasPhpOrUserDeprecations(): bool
    {}

    public function numberOfPhpOrUserDeprecations(): int
    {}

    public function hasSelfDeprecations(): bool
    {}

    public function numberOfSelfDeprecations(): int
    {}

    public function hasDirectDeprecations(): bool
    {}

    public function numberOfDirectDeprecations(): int
    {}

    public function hasIndirectDeprecations(): bool
    {}

    public function numberOfIndirectDeprecations(): int
    {}

    public function hasDeprecationsWithUnknownTrigger(): bool
    {}

    public function numberOfDeprecationsWithUnknownTrigger(): int
    {}

    public function hasPhpunitDeprecations(): bool
    {}

    public function numberOfPhpunitDeprecations(): int
    {}

    public function hasPhpunitWarnings(): bool
    {}

    public function numberOfPhpunitWarnings(): int
    {}

    public function numberOfDeprecations(): int
    {}

    public function hasNotices(): bool
    {}

    public function numberOfNotices(): int
    {}

    public function hasWarnings(): bool
    {}

    public function numberOfWarnings(): int
    {}

    public function hasIncompleteTests(): bool
    {}

    public function hasRiskyTests(): bool
    {}

    public function hasSkippedTests(): bool
    {}

    public function hasIssuesIgnoredByBaseline(): bool
    {}

    /**
     * @return non-negative-int
     */
    public function numberOfIssuesIgnoredByBaseline(): int
    {}

    public function hasPhpunitNotices(): bool
    {}

    public function numberOfPhpunitNotices(): int
    {}
}
