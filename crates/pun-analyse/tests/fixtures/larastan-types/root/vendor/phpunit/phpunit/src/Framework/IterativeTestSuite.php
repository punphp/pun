<?php declare(strict_types=1);
/*
 * This file is part of PHPUnit.
 *
 * (c) Sebastian Bergmann <sebastian@phpunit.de>
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */
namespace PHPUnit\Framework;

use function assert;
use function sprintf;
use PHPUnit\Event;
use PHPUnit\Event\EventCollection;
use PHPUnit\Event\NoPreviousThrowableException;
use SebastianBergmann\CodeCoverage\InvalidArgumentException;
use SebastianBergmann\CodeCoverage\UnintentionallyCoveredCodeException;
use Throwable;

/**
 * A test suite that aggregates the executions of a single test method, such as
 * the repetitions of a repeated test or the attempts of a retried test.
 *
 * @no-named-arguments Parameter names are not covered by the backward compatibility promise for PHPUnit
 *
 * @internal This class is not covered by the backward compatibility promise for PHPUnit
 */
abstract class IterativeTestSuite extends TestSuite
{
    /**
     * @var list<ExecutionOrderDependency>
     */
    private array $dependencies = [];

    /**
     * @param list<ExecutionOrderDependency> $dependencies
     */
    public function setDependencies(array $dependencies): void
    {}

    /**
     * @return list<ExecutionOrderDependency>
     */
    public function provides(): array
    {}

    /**
     * @return list<ExecutionOrderDependency>
     */
    public function requires(): array
    {}

    public function sortId(): string
    {}

    /**
     * @param list<Test> $tests
     *
     * @throws Event\RuntimeException
     * @throws Exception
     * @throws InvalidArgumentException
     * @throws NoPreviousThrowableException
     * @throws UnintentionallyCoveredCodeException
     */
    final protected function runTests(array $tests, Event\Emitter $emitter): void
    {}

    /**
     * @return list<ExecutionOrderDependency>
     */
    final protected function dependencies(): array
    {}

    /**
     * Run a single repetition or attempt with its events collected instead of
     * dispatched.
     *
     * The collection is stopped even when running the test throws, because a
     * dispatcher that is left collecting events swallows every event that is
     * emitted afterwards. The events collected before the throwable was raised
     * are forwarded rather than discarded.
     *
     * @throws Throwable
     */
    final protected function runCollectingEvents(Test $test): EventCollection
    {}

    /**
     * @param list<Test> $tests
     *
     * @throws Event\RuntimeException
     * @throws Exception
     * @throws InvalidArgumentException
     * @throws NoPreviousThrowableException
     * @throws UnintentionallyCoveredCodeException
     */
    abstract protected function execute(array $tests, Event\Emitter $emitter): void;

    /**
     * Emits the event for a repetition that is skipped because an earlier
     * repetition failed and returns the message the repetition is skipped with.
     *
     * @return non-empty-string
     */
    final protected function skipRemainingRepetition(Event\Code\Test $test, int $failedRepetition, Event\Emitter $emitter): string
    {}

    final protected function emitAttemptEvent(EventCollection $events, Event\Emitter $emitter): bool
    {}

    /**
     * Determine the wall-clock time spent on an attempt from its collected
     * events, namely from its Prepared event to its Finished event, so that
     * the durations of retried attempts can be accounted for even though the
     * attempt's events themselves are not forwarded. This is how the Open
     * Test Reporting logger and the test run history measure the time of a
     * test. The JUnit XML logger and the TeamCity logger measure the time of
     * a test from its PreparationStarted event instead.
     */
    final protected function durationOf(EventCollection $events): Event\Telemetry\Duration
    {}
}
