<?php declare(strict_types=1);
/*
 * This file is part of PHPUnit.
 *
 * (c) Sebastian Bergmann <sebastian@phpunit.de>
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */
namespace PHPUnit\Runner\TestRunHistory;

use function round;
use PHPUnit\Event\Event;
use PHPUnit\Event\Facade;
use PHPUnit\Event\Telemetry\HRTime;
use PHPUnit\Event\Test\ConsideredRisky;
use PHPUnit\Event\Test\Errored;
use PHPUnit\Event\Test\Failed;
use PHPUnit\Event\Test\Finished;
use PHPUnit\Event\Test\MarkedIncomplete;
use PHPUnit\Event\Test\Passed;
use PHPUnit\Event\Test\Prepared;
use PHPUnit\Event\Test\Skipped;
use PHPUnit\Framework\InvalidArgumentException;
use PHPUnit\Framework\TestStatus\TestStatus;

/**
 * @no-named-arguments Parameter names are not covered by the backward compatibility promise for PHPUnit
 *
 * @internal This class is not covered by the backward compatibility promise for PHPUnit
 */
final class TestRunHistoryHandler
{
    private readonly TestRunHistory $testRunHistory;
    private bool $pruneOnPersist;
    private ?HRTime $time        = null;
    private bool $testWasSkipped = false;

    /**
     * A status loaded from a previous run must always be replaced by the
     * current run's first result for a test; after that, only a more
     * important status may replace it, and a pass must not clear a defect
     * recorded by an earlier repetition of the same test.
     *
     * @var array<string, TestStatus>
     */
    private array $statusesRecordedInThisRun = [];

    public function __construct(TestRunHistory $testRunHistory, Facade $facade, bool $pruneOnPersist)
    {}

    /**
     * The test run history is persisted when the test runner has finished
     * executing the test suite, and not when the outermost test suite
     * finishes: a test suite that is skipped by a method that is invoked
     * before the first test of a test class does not emit the event that ends
     * it, so counting the test suites that started and finished can never
     * reach the outermost one again once such a test suite was run.
     */
    public function testRunnerExecutionFinished(): void
    {}

    public function testRunnerExecutionAborted(): void
    {}

    public function testPrepared(Prepared $event): void
    {}

    public function testMarkedIncomplete(MarkedIncomplete $event): void
    {}

    public function testConsideredRisky(ConsideredRisky $event): void
    {}

    public function testErrored(Errored $event): void
    {}

    public function testFailed(Failed $event): void
    {}

    public function testPassed(Passed $event): void
    {}

    public function testSkipped(Skipped $event): void
    {}

    /**
     * @throws \PHPUnit\Event\InvalidArgumentException
     * @throws InvalidArgumentException
     */
    public function testFinished(Finished $event): void
    {}

    private function recordStatus(TestRunHistoryId $id, TestStatus $status): void
    {}

    /**
     * @throws \PHPUnit\Event\InvalidArgumentException
     * @throws InvalidArgumentException
     */
    private function duration(Event $event): float
    {}

    private function registerSubscribers(Facade $facade): void
    {}
}
