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

use function array_merge;
use function array_values;
use function assert;
use function count;
use function implode;
use PHPUnit\Event\Code\Test;
use PHPUnit\Event\Code\TestMethod;
use PHPUnit\Event\Facade;
use PHPUnit\Event\Test\AfterLastTestMethodErrored;
use PHPUnit\Event\Test\AfterLastTestMethodFailed;
use PHPUnit\Event\Test\AttemptErrored;
use PHPUnit\Event\Test\AttemptFailed;
use PHPUnit\Event\Test\BeforeFirstTestMethodErrored;
use PHPUnit\Event\Test\BeforeFirstTestMethodFailed;
use PHPUnit\Event\Test\ConsideredRisky;
use PHPUnit\Event\Test\DeprecationTriggered;
use PHPUnit\Event\Test\Errored;
use PHPUnit\Event\Test\ErrorTriggered;
use PHPUnit\Event\Test\Failed;
use PHPUnit\Event\Test\Finished;
use PHPUnit\Event\Test\MarkedIncomplete;
use PHPUnit\Event\Test\NoticeTriggered;
use PHPUnit\Event\Test\PhpDeprecationTriggered;
use PHPUnit\Event\Test\PhpNoticeTriggered;
use PHPUnit\Event\Test\PhpunitDeprecationTriggered;
use PHPUnit\Event\Test\PhpunitErrorTriggered;
use PHPUnit\Event\Test\PhpunitNoticeTriggered;
use PHPUnit\Event\Test\PhpunitWarningTriggered;
use PHPUnit\Event\Test\PhpWarningTriggered;
use PHPUnit\Event\Test\Skipped as TestSkipped;
use PHPUnit\Event\Test\WarningTriggered;
use PHPUnit\Event\TestRunner\ChildProcessErrored;
use PHPUnit\Event\TestRunner\DeprecationTriggered as TestRunnerDeprecationTriggered;
use PHPUnit\Event\TestRunner\ErrorTriggered as TestRunnerIssueErrorTriggered;
use PHPUnit\Event\TestRunner\ExecutionStarted;
use PHPUnit\Event\TestRunner\Issue\DeprecationTriggered as TestRunnerIssueDeprecationTriggered;
use PHPUnit\Event\TestRunner\Issue\NoticeTriggered as TestRunnerIssueNoticeTriggered;
use PHPUnit\Event\TestRunner\Issue\WarningTriggered as TestRunnerIssueWarningTriggered;
use PHPUnit\Event\TestRunner\NoticeTriggered as TestRunnerNoticeTriggered;
use PHPUnit\Event\TestRunner\PhpDeprecationTriggered as TestRunnerIssuePhpDeprecationTriggered;
use PHPUnit\Event\TestRunner\PhpNoticeTriggered as TestRunnerIssuePhpNoticeTriggered;
use PHPUnit\Event\TestRunner\PhpWarningTriggered as TestRunnerIssuePhpWarningTriggered;
use PHPUnit\Event\TestRunner\TimeLimitExceeded;
use PHPUnit\Event\TestRunner\WarningTriggered as TestRunnerWarningTriggered;
use PHPUnit\Event\TestSuite\Finished as TestSuiteFinished;
use PHPUnit\Event\TestSuite\Skipped as TestSuiteSkipped;
use PHPUnit\Event\TestSuite\Started as TestSuiteStarted;
use PHPUnit\Event\TestSuite\TestSuiteForRepeatedTestMethod;
use PHPUnit\Event\TestSuite\TestSuiteForTestClass;
use PHPUnit\Event\TestSuite\TestSuiteForTestMethodWithDataProvider;
use PHPUnit\TestRunner\IssueFilter;
use PHPUnit\TestRunner\TestResult\Issues\Issue;

/**
 * @no-named-arguments Parameter names are not covered by the backward compatibility promise for PHPUnit
 *
 * @internal This class is not covered by the backward compatibility promise for PHPUnit
 */
final class Collector
{
    private readonly IssueFilter $issueFilter;
    private int $numberOfTests        = 0;
    private int $numberOfTestsRun     = 0;
    private int $numberOfAssertions   = 0;
    private bool $prepared            = false;
    private bool $childProcessErrored = false;

    /**
     * When a before-first-test method errors or fails, none of the tests of
     * its test class, the test class that was started last, are run.
     */
    private int $numberOfTestsOfTestClass = 0;

    /**
     * @var non-negative-int
     */
    private int $numberOfIssuesIgnoredByBaseline = 0;

    /**
     * @var list<AfterLastTestMethodErrored|BeforeFirstTestMethodErrored|Errored>
     */
    private array $testErroredEvents = [];

    /**
     * @var list<AfterLastTestMethodFailed|BeforeFirstTestMethodFailed|Failed>
     */
    private array $testFailedEvents = [];

    /**
     * @var list<MarkedIncomplete>
     */
    private array $testMarkedIncompleteEvents = [];

    /**
     * @var list<TestSuiteSkipped>
     */
    private array $testSuiteSkippedEvents = [];

    /**
     * @var list<TestSkipped>
     */
    private array $testSkippedEvents = [];

    /**
     * @var array<string,list<ConsideredRisky>>
     */
    private array $testConsideredRiskyEvents = [];

    /**
     * @var array<string,list<PhpunitDeprecationTriggered>>
     */
    private array $testTriggeredPhpunitDeprecationEvents = [];

    /**
     * @var array<string,list<PhpunitErrorTriggered>>
     */
    private array $testTriggeredPhpunitErrorEvents = [];

    /**
     * @var array<string,list<PhpunitNoticeTriggered>>
     */
    private array $testTriggeredPhpunitNoticeEvents = [];

    /**
     * @var array<string,list<PhpunitWarningTriggered>>
     */
    private array $testTriggeredPhpunitWarningEvents = [];

    /**
     * @var list<TestRunnerDeprecationTriggered>
     */
    private array $testRunnerTriggeredDeprecationEvents = [];

    /**
     * @var list<TestRunnerNoticeTriggered>
     */
    private array $testRunnerTriggeredNoticeEvents = [];

    /**
     * @var list<TestRunnerWarningTriggered>
     */
    private array $testRunnerTriggeredWarningEvents = [];

    /**
     * @var list<TestRunnerIssueDeprecationTriggered>
     */
    private array $testRunnerTriggeredIssueDeprecationEvents = [];

    /**
     * @var list<TestRunnerIssueErrorTriggered>
     */
    private array $testRunnerTriggeredIssueErrorEvents = [];

    /**
     * @var list<TestRunnerIssueNoticeTriggered>
     */
    private array $testRunnerTriggeredIssueNoticeEvents = [];

    /**
     * @var list<TestRunnerIssuePhpDeprecationTriggered>
     */
    private array $testRunnerTriggeredIssuePhpDeprecationEvents = [];

    /**
     * @var list<TestRunnerIssuePhpNoticeTriggered>
     */
    private array $testRunnerTriggeredIssuePhpNoticeEvents = [];

    /**
     * @var list<TestRunnerIssuePhpWarningTriggered>
     */
    private array $testRunnerTriggeredIssuePhpWarningEvents = [];

    /**
     * @var list<TestRunnerIssueWarningTriggered>
     */
    private array $testRunnerTriggeredIssueWarningEvents = [];

    /**
     * @var array<non-empty-string, Issue>
     */
    private array $errors = [];

    /**
     * @var array<non-empty-string, Issue>
     */
    private array $deprecations = [];

    /**
     * @var array<non-empty-string, Issue>
     */
    private array $notices = [];

    /**
     * @var array<non-empty-string, Issue>
     */
    private array $warnings = [];

    /**
     * @var array<non-empty-string, Issue>
     */
    private array $phpDeprecations = [];

    /**
     * @var array<non-empty-string, Issue>
     */
    private array $phpNotices = [];

    /**
     * @var array<non-empty-string, Issue>
     */
    private array $phpWarnings = [];

    /**
     * @var array{self: array<non-empty-string, true>, direct: array<non-empty-string, true>, indirect: array<non-empty-string, true>, unknown: array<non-empty-string, true>}
     */
    private array $deprecationIdsByTrigger = [
        'self'     => [],
        'direct'   => [],
        'indirect' => [],
        'unknown'  => [],
    ];

    /**
     * @var array<non-empty-string, positive-int>
     */
    private array $retriedTests                        = [];
    private ?TimeLimitExceeded $timeLimitExceededEvent = null;

    public function __construct(Facade $facade, IssueFilter $issueFilter)
    {}

    public function result(): TestResult
    {}

    public function executionStarted(ExecutionStarted $event): void
    {}

    public function testSuiteSkipped(TestSuiteSkipped $event): void
    {}

    public function testSuiteStarted(TestSuiteStarted $event): void
    {}

    public function testSuiteFinished(TestSuiteFinished $event): void
    {}

    public function testPrepared(): void
    {}

    public function testFinished(Finished $event): void
    {}

    public function beforeTestClassMethodErrored(BeforeFirstTestMethodErrored $event): void
    {}

    public function beforeTestClassMethodFailed(BeforeFirstTestMethodFailed $event): void
    {}

    public function afterTestClassMethodErrored(AfterLastTestMethodErrored $event): void
    {}

    public function afterTestClassMethodFailed(AfterLastTestMethodFailed $event): void
    {}

    public function testErrored(Errored $event): void
    {}

    public function testFailed(Failed $event): void
    {}

    public function testAttemptErrored(AttemptErrored $event): void
    {}

    public function testAttemptFailed(AttemptFailed $event): void
    {}

    public function testMarkedIncomplete(MarkedIncomplete $event): void
    {}

    public function testSkipped(TestSkipped $event): void
    {}

    public function testConsideredRisky(ConsideredRisky $event): void
    {}

    public function testTriggeredDeprecation(DeprecationTriggered $event): void
    {}

    public function testTriggeredPhpDeprecation(PhpDeprecationTriggered $event): void
    {}

    public function testTriggeredPhpunitDeprecation(PhpunitDeprecationTriggered $event): void
    {}

    public function testTriggeredPhpunitNotice(PhpunitNoticeTriggered $event): void
    {}

    public function testTriggeredError(ErrorTriggered $event): void
    {}

    public function testTriggeredNotice(NoticeTriggered $event): void
    {}

    public function testTriggeredPhpNotice(PhpNoticeTriggered $event): void
    {}

    public function testTriggeredWarning(WarningTriggered $event): void
    {}

    public function testTriggeredPhpWarning(PhpWarningTriggered $event): void
    {}

    public function testTriggeredPhpunitError(PhpunitErrorTriggered $event): void
    {}

    public function testTriggeredPhpunitWarning(PhpunitWarningTriggered $event): void
    {}

    public function testRunnerTriggeredPhpunitDeprecation(TestRunnerDeprecationTriggered $event): void
    {}

    public function testRunnerTriggeredPhpunitNotice(TestRunnerNoticeTriggered $event): void
    {}

    public function testRunnerTriggeredPhpunitWarning(TestRunnerWarningTriggered $event): void
    {}

    public function testRunnerTriggeredIssueDeprecation(TestRunnerIssueDeprecationTriggered $event): void
    {}

    public function testRunnerTriggeredIssueError(TestRunnerIssueErrorTriggered $event): void
    {}

    public function testRunnerTriggeredIssueNotice(TestRunnerIssueNoticeTriggered $event): void
    {}

    public function testRunnerTriggeredIssuePhpDeprecation(TestRunnerIssuePhpDeprecationTriggered $event): void
    {}

    public function testRunnerTriggeredIssuePhpNotice(TestRunnerIssuePhpNoticeTriggered $event): void
    {}

    public function testRunnerTriggeredIssuePhpWarning(TestRunnerIssuePhpWarningTriggered $event): void
    {}

    public function testRunnerTriggeredIssueWarning(TestRunnerIssueWarningTriggered $event): void
    {}

    public function testRunnerTimeLimitExceeded(TimeLimitExceeded $event): void
    {}

    public function wasTimeLimitExceeded(): bool
    {}

    public function childProcessErrored(ChildProcessErrored $event): void
    {}

    public function numberOfErroredTests(): int
    {}

    public function numberOfFailedTests(): int
    {}

    public function numberOfRiskyTests(): int
    {}

    public function numberOfSkippedTests(): int
    {}

    public function numberOfIncompleteTests(): int
    {}

    public function numberOfNotices(): int
    {}

    public function numberOfWarnings(): int
    {}

    private function rememberRetriedTest(Test $test): void
    {}

    private function forgetRetriedTest(Test $test): void
    {}

    /**
     * @return ?non-empty-string
     */
    private function retriedTestId(Test $test): ?string
    {}

    /**
     * @return non-empty-string
     */
    private function logicalTestId(TestMethod $test): string
    {}

    private function registerTestMethodAsPassedIfNoRunFailedOrErrored(TestSuiteForRepeatedTestMethod|TestSuiteForTestMethodWithDataProvider $testSuite): void
    {}

    /**
     * @return non-empty-string
     */
    private function issueId(DeprecationTriggered|ErrorTriggered|NoticeTriggered|PhpDeprecationTriggered|PhpNoticeTriggered|PhpWarningTriggered|WarningTriggered $event): string
    {}

    private function registerDeprecationByTrigger(DeprecationTriggered|PhpDeprecationTriggered|TestRunnerIssueDeprecationTriggered|TestRunnerIssuePhpDeprecationTriggered $event): void
    {}
}
