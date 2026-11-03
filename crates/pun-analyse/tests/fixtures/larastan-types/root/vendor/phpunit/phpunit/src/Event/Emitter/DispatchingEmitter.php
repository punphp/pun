<?php declare(strict_types=1);
/*
 * This file is part of PHPUnit.
 *
 * (c) Sebastian Bergmann <sebastian@phpunit.de>
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */
namespace PHPUnit\Event;

use function assert;
use function memory_reset_peak_usage;
use function preg_match;
use PHPUnit\Event\Code\ClassMethod;
use PHPUnit\Event\Code\ComparisonFailure;
use PHPUnit\Event\Code\IssueTrigger\IssueTrigger;
use PHPUnit\Event\Code\NoTestCaseObjectOnCallStackException;
use PHPUnit\Event\Code\TestMethod;
use PHPUnit\Event\Code\TestMethodBuilder;
use PHPUnit\Event\Code\Throwable;
use PHPUnit\Event\Test\DataProviderMethodCalled;
use PHPUnit\Event\Test\DataProviderMethodFinished;
use PHPUnit\Event\TestSuite\Filtered as TestSuiteFiltered;
use PHPUnit\Event\TestSuite\Finished as TestSuiteFinished;
use PHPUnit\Event\TestSuite\Loaded as TestSuiteLoaded;
use PHPUnit\Event\TestSuite\Skipped as TestSuiteSkipped;
use PHPUnit\Event\TestSuite\Sorted as TestSuiteSorted;
use PHPUnit\Event\TestSuite\Started as TestSuiteStarted;
use PHPUnit\Event\TestSuite\TestSuite;
use PHPUnit\Framework\TestCase;
use PHPUnit\Metadata\IgnorePhpunitWarnings;
use PHPUnit\Metadata\Parser\Registry;
use PHPUnit\TextUI\Configuration\Configuration;
use SebastianBergmann\Comparator\Comparator;

/**
 * @no-named-arguments Parameter names are not covered by the backward compatibility promise for PHPUnit
 *
 * @internal This class is not covered by the backward compatibility promise for PHPUnit
 */
final class DispatchingEmitter implements Emitter
{
    private readonly Dispatcher $dispatcher;
    private readonly Telemetry\System $system;
    private readonly Telemetry\Snapshot $startSnapshot;
    private Telemetry\Snapshot $previousSnapshot;

    public function __construct(Dispatcher $dispatcher, Telemetry\System $system)
    {}

    /**
     * @throws InvalidArgumentException
     * @throws UnknownEventTypeException
     */
    public function applicationStarted(): void
    {}

    /**
     * @throws InvalidArgumentException
     * @throws UnknownEventTypeException
     */
    public function testRunnerStarted(): void
    {}

    /**
     * @throws InvalidArgumentException
     * @throws UnknownEventTypeException
     */
    public function testRunnerConfigured(Configuration $configuration): void
    {}

    /**
     * @param non-empty-string $filename
     *
     * @throws InvalidArgumentException
     * @throws UnknownEventTypeException
     */
    public function testRunnerBootstrapFinished(string $filename): void
    {}

    /**
     * @param non-empty-string $filename
     * @param non-empty-string $name
     * @param non-empty-string $version
     *
     * @throws InvalidArgumentException
     * @throws UnknownEventTypeException
     */
    public function testRunnerLoadedExtensionFromPhar(string $filename, string $name, string $version): void
    {}

    /**
     * @param class-string          $className
     * @param array<string, string> $parameters
     *
     * @throws InvalidArgumentException
     * @throws UnknownEventTypeException
     */
    public function testRunnerBootstrappedExtension(string $className, array $parameters): void
    {}

    /**
     * @throws InvalidArgumentException
     * @throws UnknownEventTypeException
     */
    public function dataProviderMethodCalled(ClassMethod $testMethod, ClassMethod $dataProviderMethod): void
    {}

    /**
     * @throws InvalidArgumentException
     * @throws UnknownEventTypeException
     */
    public function dataProviderMethodFinished(ClassMethod $testMethod, ClassMethod ...$calledMethods): void
    {}

    /**
     * @throws InvalidArgumentException
     * @throws UnknownEventTypeException
     */
    public function testSuiteLoaded(TestSuite $testSuite): void
    {}

    /**
     * @throws InvalidArgumentException
     * @throws UnknownEventTypeException
     */
    public function testSuiteFiltered(TestSuite $testSuite): void
    {}

    /**
     * @param list<non-empty-string> $pipeline
     *
     * @throws InvalidArgumentException
     * @throws UnknownEventTypeException
     */
    public function testSuiteSorted(int $executionOrder, int $executionOrderDefects, bool $resolveDependencies, array $pipeline): void
    {}

    /**
     * @throws InvalidArgumentException
     * @throws UnknownEventTypeException
     */
    public function testRunnerEventFacadeSealed(): void
    {}

    /**
     * @throws InvalidArgumentException
     * @throws UnknownEventTypeException
     */
    public function testRunnerExecutionStarted(TestSuite $testSuite): void
    {}

    /**
     * @throws InvalidArgumentException
     * @throws UnknownEventTypeException
     */
    public function testRunnerDisabledGarbageCollection(): void
    {}

    /**
     * @throws InvalidArgumentException
     * @throws UnknownEventTypeException
     */
    public function testRunnerTriggeredGarbageCollection(): void
    {}

    public function childProcessStarted(TestRunner\ChildProcessReason $reason): void
    {}

    /**
     * @param non-empty-string $message
     */
    public function childProcessErrored(TestRunner\ChildProcessReason $reason, string $message): void
    {}

    public function childProcessFinished(TestRunner\ChildProcessReason $reason, string $stdout, string $stderr): void
    {}

    /**
     * @throws InvalidArgumentException
     * @throws UnknownEventTypeException
     */
    public function testSuiteSkipped(TestSuite $testSuite, string $message): void
    {}

    /**
     * @throws InvalidArgumentException
     * @throws UnknownEventTypeException
     */
    public function testSuiteStarted(TestSuite $testSuite): void
    {}

    /**
     * @throws InvalidArgumentException
     * @throws UnknownEventTypeException
     */
    public function testPreparationStarted(Code\Test $test): void
    {}

    /**
     * @throws InvalidArgumentException
     * @throws UnknownEventTypeException
     */
    public function testPreparationErrored(Code\Test $test, Throwable $throwable): void
    {}

    /**
     * @throws InvalidArgumentException
     * @throws UnknownEventTypeException
     */
    public function testPreparationFailed(Code\Test $test, Throwable $throwable): void
    {}

    /**
     * @param class-string<TestCase> $testClassName
     *
     * @throws InvalidArgumentException
     * @throws UnknownEventTypeException
     */
    public function beforeFirstTestMethodCalled(string $testClassName, ClassMethod $calledMethod): void
    {}

    /**
     * @param class-string<TestCase> $testClassName
     *
     * @throws InvalidArgumentException
     * @throws UnknownEventTypeException
     */
    public function beforeFirstTestMethodErrored(string $testClassName, ClassMethod $calledMethod, Throwable $throwable): void
    {}

    /**
     * @param class-string<TestCase> $testClassName
     *
     * @throws InvalidArgumentException
     * @throws UnknownEventTypeException
     */
    public function beforeFirstTestMethodFailed(string $testClassName, ClassMethod $calledMethod, Throwable $throwable): void
    {}

    /**
     * @param class-string<TestCase> $testClassName
     *
     * @throws InvalidArgumentException
     * @throws UnknownEventTypeException
     */
    public function beforeFirstTestMethodFinished(string $testClassName, ClassMethod ...$calledMethods): void
    {}

    /**
     * @throws InvalidArgumentException
     * @throws UnknownEventTypeException
     */
    public function beforeTestMethodCalled(TestMethod $test, ClassMethod $calledMethod): void
    {}

    /**
     * @throws InvalidArgumentException
     * @throws UnknownEventTypeException
     */
    public function beforeTestMethodErrored(TestMethod $test, ClassMethod $calledMethod, Throwable $throwable): void
    {}

    /**
     * @throws InvalidArgumentException
     * @throws UnknownEventTypeException
     */
    public function beforeTestMethodFailed(TestMethod $test, ClassMethod $calledMethod, Throwable $throwable): void
    {}

    /**
     * @throws InvalidArgumentException
     * @throws UnknownEventTypeException
     */
    public function beforeTestMethodFinished(TestMethod $test, ClassMethod ...$calledMethods): void
    {}

    /**
     * @throws InvalidArgumentException
     * @throws UnknownEventTypeException
     */
    public function preConditionCalled(TestMethod $test, ClassMethod $calledMethod): void
    {}

    /**
     * @throws InvalidArgumentException
     * @throws UnknownEventTypeException
     */
    public function preConditionErrored(TestMethod $test, ClassMethod $calledMethod, Throwable $throwable): void
    {}

    /**
     * @throws InvalidArgumentException
     * @throws UnknownEventTypeException
     */
    public function preConditionFailed(TestMethod $test, ClassMethod $calledMethod, Throwable $throwable): void
    {}

    /**
     * @throws InvalidArgumentException
     * @throws UnknownEventTypeException
     */
    public function preConditionFinished(TestMethod $test, ClassMethod ...$calledMethods): void
    {}

    /**
     * @throws InvalidArgumentException
     * @throws UnknownEventTypeException
     */
    public function testPrepared(Code\Test $test): void
    {}

    /**
     * @param class-string<Comparator> $className
     *
     * @throws InvalidArgumentException
     * @throws UnknownEventTypeException
     */
    public function testRegisteredComparator(string $className): void
    {}

    public function testUsedCustomMethodInvocation(TestMethod $test, ClassMethod $customTestMethodInvocation): void
    {}

    /**
     * @param class-string $className
     *
     * @throws InvalidArgumentException
     * @throws UnknownEventTypeException
     */
    public function testCreatedMockObject(string $className): void
    {}

    /**
     * @param list<class-string> $interfaces
     *
     * @throws InvalidArgumentException
     * @throws UnknownEventTypeException
     */
    public function testCreatedMockObjectForIntersectionOfInterfaces(array $interfaces): void
    {}

    /**
     * @param class-string $className
     *
     * @throws InvalidArgumentException
     * @throws UnknownEventTypeException
     */
    public function testCreatedPartialMockObject(string $className, string ...$methodNames): void
    {}

    /**
     * @param class-string $className
     *
     * @throws InvalidArgumentException
     * @throws UnknownEventTypeException
     */
    public function testCreatedStub(string $className): void
    {}

    /**
     * @param list<class-string> $interfaces
     *
     * @throws InvalidArgumentException
     * @throws UnknownEventTypeException
     */
    public function testCreatedStubForIntersectionOfInterfaces(array $interfaces): void
    {}

    /**
     * @throws InvalidArgumentException
     * @throws UnknownEventTypeException
     */
    public function testErrored(Code\Test $test, Throwable $throwable): void
    {}

    /**
     * @throws InvalidArgumentException
     * @throws UnknownEventTypeException
     */
    public function testFailed(Code\Test $test, Throwable $throwable, ?ComparisonFailure $comparisonFailure): void
    {}

    /**
     * @throws InvalidArgumentException
     * @throws UnknownEventTypeException
     */
    public function testAttemptErrored(Code\Test $test, Throwable $throwable, Telemetry\Duration $duration): void
    {}

    /**
     * @throws InvalidArgumentException
     * @throws UnknownEventTypeException
     */
    public function testAttemptFailed(Code\Test $test, Throwable $throwable, ?ComparisonFailure $comparisonFailure, Telemetry\Duration $duration): void
    {}

    /**
     * @throws InvalidArgumentException
     * @throws UnknownEventTypeException
     */
    public function testPassed(Code\Test $test): void
    {}

    /**
     * @throws InvalidArgumentException
     * @throws UnknownEventTypeException
     */
    public function testConsideredRisky(Code\Test $test, string $message): void
    {}

    /**
     * @throws InvalidArgumentException
     * @throws UnknownEventTypeException
     */
    public function testMarkedAsIncomplete(Code\Test $test, Throwable $throwable): void
    {}

    /**
     * @throws InvalidArgumentException
     * @throws UnknownEventTypeException
     */
    public function testSkipped(Code\Test $test, string $message): void
    {}

    /**
     * @param non-empty-string $message
     *
     * @throws InvalidArgumentException
     * @throws NoTestCaseObjectOnCallStackException
     * @throws UnknownEventTypeException
     */
    public function testTriggeredPhpunitDeprecation(?Code\Test $test, string $message): void
    {}

    /**
     * @param non-empty-string $message
     *
     * @throws InvalidArgumentException
     * @throws NoTestCaseObjectOnCallStackException
     * @throws UnknownEventTypeException
     */
    public function testTriggeredPhpunitNotice(Code\Test $test, string $message): void
    {}

    /**
     * @param non-empty-string $message
     * @param non-empty-string $file
     * @param positive-int     $line
     *
     * @throws InvalidArgumentException
     * @throws UnknownEventTypeException
     */
    public function testTriggeredPhpDeprecation(Code\Test $test, string $message, string $file, int $line, bool $suppressed, bool $ignoredByBaseline, bool $ignoredByTest, bool $ignoredByFilter, IssueTrigger $trigger): void
    {}

    /**
     * @param non-empty-string $message
     * @param non-empty-string $file
     * @param positive-int     $line
     * @param non-empty-string $stackTrace
     *
     * @throws InvalidArgumentException
     * @throws UnknownEventTypeException
     */
    public function testTriggeredDeprecation(Code\Test $test, string $message, string $file, int $line, bool $suppressed, bool $ignoredByBaseline, bool $ignoredByTest, bool $ignoredByFilter, IssueTrigger $trigger, string $stackTrace): void
    {}

    /**
     * @param non-empty-string $message
     * @param non-empty-string $file
     * @param positive-int     $line
     *
     * @throws InvalidArgumentException
     * @throws UnknownEventTypeException
     */
    public function testTriggeredError(Code\Test $test, string $message, string $file, int $line, bool $suppressed): void
    {}

    /**
     * @param non-empty-string $message
     * @param non-empty-string $file
     * @param positive-int     $line
     *
     * @throws InvalidArgumentException
     * @throws UnknownEventTypeException
     */
    public function testTriggeredNotice(Code\Test $test, string $message, string $file, int $line, bool $suppressed, bool $ignoredByBaseline): void
    {}

    /**
     * @param non-empty-string $message
     * @param non-empty-string $file
     * @param positive-int     $line
     *
     * @throws InvalidArgumentException
     * @throws UnknownEventTypeException
     */
    public function testTriggeredPhpNotice(Code\Test $test, string $message, string $file, int $line, bool $suppressed, bool $ignoredByBaseline): void
    {}

    /**
     * @param non-empty-string $message
     * @param non-empty-string $file
     * @param positive-int     $line
     *
     * @throws InvalidArgumentException
     * @throws UnknownEventTypeException
     */
    public function testTriggeredWarning(Code\Test $test, string $message, string $file, int $line, bool $suppressed, bool $ignoredByBaseline): void
    {}

    /**
     * @param non-empty-string $message
     * @param non-empty-string $file
     * @param positive-int     $line
     *
     * @throws InvalidArgumentException
     * @throws UnknownEventTypeException
     */
    public function testTriggeredPhpWarning(Code\Test $test, string $message, string $file, int $line, bool $suppressed, bool $ignoredByBaseline): void
    {}

    /**
     * @param non-empty-string $message
     *
     * @throws InvalidArgumentException
     * @throws UnknownEventTypeException
     */
    public function testTriggeredPhpunitError(Code\Test $test, string $message): void
    {}

    /**
     * @param non-empty-string $message
     *
     * @throws InvalidArgumentException
     * @throws UnknownEventTypeException
     */
    public function testTriggeredPhpunitWarning(Code\Test $test, string $message): void
    {}

    /**
     * @param non-empty-string $output
     *
     * @throws InvalidArgumentException
     * @throws UnknownEventTypeException
     */
    public function testPrintedUnexpectedOutput(string $output): void
    {}

    /**
     * @param non-empty-string $additionalInformation
     *
     * @throws InvalidArgumentException
     * @throws UnknownEventTypeException
     */
    public function testProvidedAdditionalInformation(TestMethod $test, string $additionalInformation): void
    {}

    /**
     * @param non-negative-int $numberOfAssertionsPerformed
     *
     * @throws InvalidArgumentException
     * @throws UnknownEventTypeException
     */
    public function testFinished(Code\Test $test, int $numberOfAssertionsPerformed): void
    {}

    /**
     * @throws InvalidArgumentException
     * @throws UnknownEventTypeException
     */
    public function postConditionCalled(TestMethod $test, ClassMethod $calledMethod): void
    {}

    /**
     * @throws InvalidArgumentException
     * @throws UnknownEventTypeException
     */
    public function postConditionErrored(TestMethod $test, ClassMethod $calledMethod, Throwable $throwable): void
    {}

    /**
     * @throws InvalidArgumentException
     * @throws UnknownEventTypeException
     */
    public function postConditionFailed(TestMethod $test, ClassMethod $calledMethod, Throwable $throwable): void
    {}

    /**
     * @throws InvalidArgumentException
     * @throws UnknownEventTypeException
     */
    public function postConditionFinished(TestMethod $test, ClassMethod ...$calledMethods): void
    {}

    /**
     * @throws InvalidArgumentException
     * @throws UnknownEventTypeException
     */
    public function afterTestMethodCalled(TestMethod $test, ClassMethod $calledMethod): void
    {}

    /**
     * @throws InvalidArgumentException
     * @throws UnknownEventTypeException
     */
    public function afterTestMethodErrored(TestMethod $test, ClassMethod $calledMethod, Throwable $throwable): void
    {}

    /**
     * @throws InvalidArgumentException
     * @throws UnknownEventTypeException
     */
    public function afterTestMethodFailed(TestMethod $test, ClassMethod $calledMethod, Throwable $throwable): void
    {}

    /**
     * @throws InvalidArgumentException
     * @throws UnknownEventTypeException
     */
    public function afterTestMethodFinished(TestMethod $test, ClassMethod ...$calledMethods): void
    {}

    /**
     * @param class-string<TestCase> $testClassName
     *
     * @throws InvalidArgumentException
     * @throws UnknownEventTypeException
     */
    public function afterLastTestMethodCalled(string $testClassName, ClassMethod $calledMethod): void
    {}

    /**
     * @param class-string<TestCase> $testClassName
     *
     * @throws InvalidArgumentException
     * @throws UnknownEventTypeException
     */
    public function afterLastTestMethodErrored(string $testClassName, ClassMethod $calledMethod, Throwable $throwable): void
    {}

    /**
     * @param class-string<TestCase> $testClassName
     *
     * @throws InvalidArgumentException
     * @throws UnknownEventTypeException
     */
    public function afterLastTestMethodFailed(string $testClassName, ClassMethod $calledMethod, Throwable $throwable): void
    {}

    /**
     * @param class-string<TestCase> $testClassName
     *
     * @throws InvalidArgumentException
     * @throws UnknownEventTypeException
     */
    public function afterLastTestMethodFinished(string $testClassName, ClassMethod ...$calledMethods): void
    {}

    /**
     * @throws InvalidArgumentException
     * @throws UnknownEventTypeException
     */
    public function testSuiteFinished(TestSuite $testSuite): void
    {}

    /**
     * @throws InvalidArgumentException
     * @throws UnknownEventTypeException
     */
    public function testRunnerStartedStaticAnalysisForCodeCoverage(): void
    {}

    /**
     * @param non-negative-int $cacheHits
     * @param non-negative-int $cacheMisses
     *
     * @throws InvalidArgumentException
     * @throws UnknownEventTypeException
     */
    public function testRunnerFinishedStaticAnalysisForCodeCoverage(int $cacheHits, int $cacheMisses): void
    {}

    /**
     * @param non-empty-string $message
     *
     * @throws InvalidArgumentException
     * @throws UnknownEventTypeException
     */
    public function testRunnerTriggeredPhpunitDeprecation(string $message): void
    {}

    /**
     * @param non-empty-string $message
     *
     * @throws InvalidArgumentException
     * @throws UnknownEventTypeException
     */
    public function testRunnerTriggeredPhpunitNotice(string $message): void
    {}

    /**
     * @param non-empty-string $message
     *
     * @throws InvalidArgumentException
     * @throws UnknownEventTypeException
     */
    public function testRunnerTriggeredPhpunitWarning(string $message): void
    {}

    /**
     * @param non-empty-string $message
     * @param non-empty-string $file
     * @param positive-int     $line
     *
     * @throws InvalidArgumentException
     * @throws UnknownEventTypeException
     */
    public function testRunnerTriggeredPhpNotice(string $message, string $file, int $line, bool $suppressed, bool $ignoredByBaseline): void
    {}

    /**
     * @param non-empty-string $message
     * @param non-empty-string $file
     * @param positive-int     $line
     *
     * @throws InvalidArgumentException
     * @throws UnknownEventTypeException
     */
    public function testRunnerTriggeredNotice(string $message, string $file, int $line, bool $suppressed, bool $ignoredByBaseline): void
    {}

    /**
     * @param non-empty-string $message
     * @param non-empty-string $file
     * @param positive-int     $line
     *
     * @throws InvalidArgumentException
     * @throws UnknownEventTypeException
     */
    public function testRunnerTriggeredPhpWarning(string $message, string $file, int $line, bool $suppressed, bool $ignoredByBaseline): void
    {}

    /**
     * @param non-empty-string $message
     * @param non-empty-string $file
     * @param positive-int     $line
     *
     * @throws InvalidArgumentException
     * @throws UnknownEventTypeException
     */
    public function testRunnerTriggeredWarning(string $message, string $file, int $line, bool $suppressed, bool $ignoredByBaseline): void
    {}

    /**
     * @param non-empty-string $message
     * @param non-empty-string $file
     * @param positive-int     $line
     *
     * @throws InvalidArgumentException
     * @throws UnknownEventTypeException
     */
    public function testRunnerTriggeredPhpDeprecation(string $message, string $file, int $line, bool $suppressed, bool $ignoredByBaseline, bool $ignoredByFilter, IssueTrigger $trigger): void
    {}

    /**
     * @param non-empty-string $message
     * @param non-empty-string $file
     * @param positive-int     $line
     * @param non-empty-string $stackTrace
     *
     * @throws InvalidArgumentException
     * @throws UnknownEventTypeException
     */
    public function testRunnerTriggeredDeprecation(string $message, string $file, int $line, bool $suppressed, bool $ignoredByBaseline, bool $ignoredByFilter, IssueTrigger $trigger, string $stackTrace): void
    {}

    /**
     * @param non-empty-string $message
     * @param non-empty-string $file
     * @param positive-int     $line
     *
     * @throws InvalidArgumentException
     * @throws UnknownEventTypeException
     */
    public function testRunnerTriggeredError(string $message, string $file, int $line, bool $suppressed): void
    {}

    /**
     * @throws InvalidArgumentException
     * @throws UnknownEventTypeException
     */
    public function testRunnerEnabledGarbageCollection(): void
    {}

    /**
     * @throws InvalidArgumentException
     * @throws UnknownEventTypeException
     */
    public function testRunnerExecutionAborted(): void
    {}

    /**
     * @throws InvalidArgumentException
     * @throws UnknownEventTypeException
     */
    public function testRunnerExecutionFinished(): void
    {}

    /**
     * @throws InvalidArgumentException
     * @throws UnknownEventTypeException
     */
    public function testRunnerFinished(): void
    {}

    /**
     * @param positive-int $timeLimit
     *
     * @throws InvalidArgumentException
     * @throws UnknownEventTypeException
     */
    public function testRunnerTimeLimitExceeded(int $timeLimit): void
    {}

    /**
     * @throws InvalidArgumentException
     * @throws UnknownEventTypeException
     */
    public function applicationFinished(int $shellExitCode): void
    {}

    /**
     * @throws InvalidArgumentException
     */
    private function telemetryInfo(): Telemetry\Info
    {}
}
