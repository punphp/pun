<?php declare(strict_types=1);
/*
 * This file is part of PHPUnit.
 *
 * (c) Sebastian Bergmann <sebastian@phpunit.de>
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */
namespace PHPUnit\Framework\TestRunner;

use function assert;
use function hash_equals;
use function is_int;
use function property_exists;
use function sprintf;
use function strlen;
use function substr;
use function trim;
use function unserialize;
use PHPUnit\Event\Code\TestMethodBuilder;
use PHPUnit\Event\Code\ThrowableBuilder;
use PHPUnit\Event\Emitter;
use PHPUnit\Event\EventCollection;
use PHPUnit\Event\Facade;
use PHPUnit\Event\TestRunner\ChildProcessReason;
use PHPUnit\Framework\AssertionFailedError;
use PHPUnit\Framework\Exception;
use PHPUnit\Framework\Test;
use PHPUnit\Framework\TestCase;
use PHPUnit\Framework\TestStatus\TestStatus;
use PHPUnit\Runner\CodeCoverage;
use PHPUnit\TestRunner\TestResult\Facade as TestResultFacade;
use PHPUnit\TestRunner\TestResult\PassedTests;
use stdClass;
use Throwable;

/**
 * @no-named-arguments Parameter names are not covered by the backward compatibility promise for PHPUnit
 */
final readonly class ChildProcessResultProcessor
{
    private Facade $eventFacade;
    private Emitter $emitter;
    private PassedTests $passedTests;
    private CodeCoverage $codeCoverage;

    public function __construct(Facade $eventFacade, Emitter $emitter, PassedTests $passedTests, CodeCoverage $codeCoverage)
    {}

    /**
     * @param ?non-empty-string $processResultNonce
     */
    public function process(Test $test, string $serializedProcessResult, string $stderr, ?string $processResultNonce = null): void
    {}

    /**
     * When the test run was interrupted by a signal, the child process was
     * most likely terminated by the same signal before the test was finished,
     * so that it did not produce a usable result. The test is then reported
     * as skipped rather than as errored.
     */
    private function testWasInterrupted(Test $test): void
    {}
}
