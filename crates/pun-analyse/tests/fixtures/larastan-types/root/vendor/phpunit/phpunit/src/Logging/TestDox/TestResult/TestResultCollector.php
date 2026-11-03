<?php declare(strict_types=1);
/*
 * This file is part of PHPUnit.
 *
 * (c) Sebastian Bergmann <sebastian@phpunit.de>
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */
namespace PHPUnit\Logging\TestDox;

use function array_keys;
use function array_merge;
use function assert;
use function is_subclass_of;
use function strnatcasecmp;
use function uasort;
use function uksort;
use function usort;
use PHPUnit\Event\Code\TestMethod;
use PHPUnit\Event\Code\Throwable;
use PHPUnit\Event\Facade;
use PHPUnit\Event\InvalidArgumentException;
use PHPUnit\Event\Test\ConsideredRisky;
use PHPUnit\Event\Test\DeprecationTriggered;
use PHPUnit\Event\Test\Errored;
use PHPUnit\Event\Test\Failed;
use PHPUnit\Event\Test\Finished;
use PHPUnit\Event\Test\MarkedIncomplete;
use PHPUnit\Event\Test\NoticeTriggered;
use PHPUnit\Event\Test\Passed;
use PHPUnit\Event\Test\PhpDeprecationTriggered;
use PHPUnit\Event\Test\PhpNoticeTriggered;
use PHPUnit\Event\Test\PhpunitDeprecationTriggered;
use PHPUnit\Event\Test\PhpunitErrorTriggered;
use PHPUnit\Event\Test\PhpunitWarningTriggered;
use PHPUnit\Event\Test\PhpWarningTriggered;
use PHPUnit\Event\Test\Prepared;
use PHPUnit\Event\Test\Skipped;
use PHPUnit\Event\Test\WarningTriggered;
use PHPUnit\Event\TestSuite\Skipped as TestSuiteSkipped;
use PHPUnit\Framework\TestStatus\TestStatus;
use PHPUnit\Logging\TestDox\TestResult as TestDoxTestMethod;
use PHPUnit\TestRunner\IssueFilter;
use ReflectionMethod;

/**
 * @no-named-arguments Parameter names are not covered by the backward compatibility promise for PHPUnit
 *
 * @internal This class is not covered by the backward compatibility promise for PHPUnit
 */
final class TestResultCollector
{
    private readonly IssueFilter $issueFilter;

    /**
     * @var array<class-string, list<TestDoxTestMethod>>
     */
    private array $tests          = [];
    private ?TestStatus $status   = null;
    private ?Throwable $throwable = null;
    private bool $prepared        = false;

    public function __construct(Facade $facade, IssueFilter $issueFilter)
    {}

    /**
     * @return array<class-string, TestResultCollection>
     */
    public function testMethodsGroupedByClass(): array
    {}

    public function testPrepared(Prepared $event): void
    {}

    public function testErrored(Errored $event): void
    {}

    public function testFailed(Failed $event): void
    {}

    public function testPassed(Passed $event): void
    {}

    public function testSkipped(Skipped $event): void
    {}

    public function testMarkedIncomplete(MarkedIncomplete $event): void
    {}

    public function testConsideredRisky(ConsideredRisky $event): void
    {}

    public function testTriggeredDeprecation(DeprecationTriggered $event): void
    {}

    public function testTriggeredNotice(NoticeTriggered $event): void
    {}

    public function testTriggeredWarning(WarningTriggered $event): void
    {}

    public function testTriggeredPhpDeprecation(PhpDeprecationTriggered $event): void
    {}

    public function testTriggeredPhpNotice(PhpNoticeTriggered $event): void
    {}

    public function testTriggeredPhpWarning(PhpWarningTriggered $event): void
    {}

    public function testTriggeredPhpunitDeprecation(PhpunitDeprecationTriggered $event): void
    {}

    public function testTriggeredPhpunitError(PhpunitErrorTriggered $event): void
    {}

    public function testTriggeredPhpunitWarning(PhpunitWarningTriggered $event): void
    {}

    /**
     * @throws InvalidArgumentException
     */
    public function testFinished(Finished $event): void
    {}

    /**
     * A test class that is skipped as a whole is reported as a skipped test
     * suite, and its tests never start: none of them emits the event that
     * ends a test, which is what every other test is recorded on. They are
     * recorded here so that they are not missing from the output, which
     * would otherwise show fewer tests than the test run counted.
     */
    public function testSuiteSkipped(TestSuiteSkipped $event): void
    {}

    private function registerSubscribers(Facade $facade): void
    {}

    private function updateTestStatus(TestStatus $status, ?Throwable $throwable = null): void
    {}

    private function process(TestMethod $test): void
    {}

    /**
     * A test that is skipped, marked incomplete, or errored before it starts
     * - because a test it depends on did not pass, because a requirement it
     * declares is not met, or because setUp() decided so - never emits the
     * event that ends a test, which is what every other test is recorded on.
     * It is recorded here instead.
     *
     * What it was recorded with must not be carried over to the next test
     * that never starts: nothing resets it in between, and a status that is
     * kept would be the more important one of two unrelated tests.
     */
    private function recordTestThatNeverStarted(TestMethod $test): void
    {}

    private function record(TestMethod $test, TestStatus $status, ?Throwable $throwable): void
    {}
}
