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

use const PHP_EOL;
use function array_any;
use function array_keys;
use function array_merge;
use function array_values;
use function assert;
use function chdir;
use function clearstatcache;
use function error_clear_last;
use function getcwd;
use function implode;
use function is_callable;
use function libxml_clear_errors;
use function method_exists;
use function sprintf;
use function str_contains;
use AssertionError;
use PHPUnit\Event;
use PHPUnit\Event\NoPreviousThrowableException;
use PHPUnit\Framework\MockObject\Exception as MockObjectException;
use PHPUnit\Framework\MockObject\InvocationJournal;
use PHPUnit\Framework\MockObject\InvocationJournalImplementation;
use PHPUnit\Framework\MockObject\MockBuilder;
use PHPUnit\Framework\MockObject\MockObject;
use PHPUnit\Framework\MockObject\Rule\AnyInvokedCount as AnyInvokedCountMatcher;
use PHPUnit\Framework\MockObject\Rule\InvokedAtLeastCount as InvokedAtLeastCountMatcher;
use PHPUnit\Framework\MockObject\Rule\InvokedAtLeastOnce as InvokedAtLeastOnceMatcher;
use PHPUnit\Framework\MockObject\Rule\InvokedAtMostCount as InvokedAtMostCountMatcher;
use PHPUnit\Framework\MockObject\Rule\InvokedCount as InvokedCountMatcher;
use PHPUnit\Framework\MockObject\Stub;
use PHPUnit\Framework\MockObject\Stub\Exception as ExceptionStub;
use PHPUnit\Framework\MockObject\TestStubBuilder;
use PHPUnit\Framework\TestCase\CustomRegistrations;
use PHPUnit\Framework\TestCase\DataSet;
use PHPUnit\Framework\TestCase\DependencyResolver;
use PHPUnit\Framework\TestCase\DeprecationExpectation;
use PHPUnit\Framework\TestCase\EnvironmentVariables;
use PHPUnit\Framework\TestCase\ErrorLogCapture;
use PHPUnit\Framework\TestCase\ExceptionExpectation;
use PHPUnit\Framework\TestCase\GlobalStateCapture;
use PHPUnit\Framework\TestCase\HookMethodInvoker;
use PHPUnit\Framework\TestCase\MockObjectRegistry;
use PHPUnit\Framework\TestCase\OutputBuffer;
use PHPUnit\Framework\TestCase\OutputBufferStopResult;
use PHPUnit\Framework\TestCase\TestDoubleFactory;
use PHPUnit\Framework\TestRunner\TestRunner;
use PHPUnit\Framework\TestSize\TestSize;
use PHPUnit\Framework\TestStatus\TestStatus;
use PHPUnit\Metadata\Api\Groups;
use PHPUnit\Metadata\Api\HookMethods;
use PHPUnit\Metadata\Api\Requirements;
use PHPUnit\Metadata\Parser\Registry as MetadataRegistry;
use PHPUnit\TestRunner\TestResult\PassedTests;
use ReflectionMethod;
use SebastianBergmann\CodeCoverage\UnintentionallyCoveredCodeException;
use SebastianBergmann\Comparator\Comparator;
use SebastianBergmann\Exporter\ObjectExporter;
use SebastianBergmann\Invoker\TimeoutException;
use Throwable;

/**
 * @no-named-arguments Parameter names are not covered by the backward compatibility promise for PHPUnit
 *
 * @phpstan-import-type HookMethodsByType from HookMethods
 */
abstract class TestCase extends Assert implements Reorderable, SelfDescribing, Test
{
    /**
     * @var non-empty-string
     */
    private string $methodName;
    private DataSet $dataSet;

    /**
     * @var list<string>
     */
    private array $groups = [];

    /**
     * @var list<ExecutionOrderDependency>
     */
    private array $providedTests = [];

    /**
     * @var list<ExecutionOrderDependency>
     */
    private array $dependencies = [];

    /**
     * @var array<string, mixed>
     */
    private array $dependencyInput                 = [];
    private ?bool $runTestInSeparateProcess        = null;
    private bool $preserveGlobalState              = false;
    private bool $inIsolation                      = false;
    private ?string $emptyDataProviderSkipMessage  = null;
    private ?Throwable $throwableFromDeferredIssue = null;

    /**
     * @var positive-int
     */
    private int $repetition = 1;

    /**
     * @var positive-int
     */
    private int $totalRepetitions = 1;

    /**
     * @var positive-int
     */
    private int $attempt = 1;

    /**
     * @var positive-int
     */
    private int $maxAttempts = 1;
    private ExceptionExpectation $exceptionExpectation;
    private DeprecationExpectation $deprecationExpectation;
    private OutputBuffer $outputBuffer;
    private ErrorLogCapture $errorLogCapture;
    private GlobalStateCapture $globalStateCapture;
    private EnvironmentVariables $environmentVariables;
    private MockObjectRegistry $mockObjectRegistry;
    private CustomRegistrations $customRegistrations;

    /**
     * @var array<class-string, true>
     */
    private array $failureTypes = [];
    private TestStatus $status;

    /**
     * @var non-negative-int
     */
    private int $numberOfAssertionsPerformed                 = 0;
    private mixed $testResult                                = null;
    private bool $doesNotPerformAssertions                   = false;
    private bool $wasPrepared                                = false;
    private ?Event\Code\TestMethod $testValueObjectForEvents = null;
    private Event\Emitter $emitter;

    /**
     * @param non-empty-string $name
     *
     * @internal This method is not covered by the backward compatibility promise for PHPUnit
     */
    final public function __construct(string $name)
    {}

    /**
     * This method is called before the first test of this test class is run.
     *
     * @codeCoverageIgnore
     */
    public static function setUpBeforeClass(): void
    {}

    /**
     * This method is called after the last test of this test class is run.
     *
     * @codeCoverageIgnore
     */
    public static function tearDownAfterClass(): void
    {}

    /**
     * This method is called before each test.
     *
     * @codeCoverageIgnore
     */
    protected function setUp(): void
    {}

    /**
     * Performs assertions shared by all tests of a test case.
     *
     * This method is called between setUp() and test.
     *
     * @codeCoverageIgnore
     */
    protected function assertPreConditions(): void
    {}

    /**
     * Performs assertions shared by all tests of a test case.
     *
     * This method is called between test and tearDown().
     *
     * @codeCoverageIgnore
     */
    protected function assertPostConditions(): void
    {}

    /**
     * This method is called after each test.
     *
     * @codeCoverageIgnore
     */
    protected function tearDown(): void
    {}

    /**
     * Returns a string representation of the test case.
     *
     * @throws Exception
     *
     * @internal This method is not covered by the backward compatibility promise for PHPUnit
     */
    public function toString(): string
    {}

    /**
     * @internal This method is not covered by the backward compatibility promise for PHPUnit
     */
    final public function count(): int
    {}

    /**
     * @internal This method is not covered by the backward compatibility promise for PHPUnit
     */
    final public function status(): TestStatus
    {}

    /**
     * @throws \PHPUnit\Runner\Exception
     * @throws \PHPUnit\Util\Exception
     * @throws \SebastianBergmann\CodeCoverage\InvalidArgumentException
     * @throws \SebastianBergmann\Template\InvalidArgumentException
     * @throws Exception
     * @throws NoPreviousThrowableException
     * @throws ProcessIsolationException
     * @throws UnintentionallyCoveredCodeException
     *
     * @internal This method is not covered by the backward compatibility promise for PHPUnit
     */
    final public function run(): void
    {}

    /**
     * @return list<string>
     *
     * @internal This method is not covered by the backward compatibility promise for PHPUnit
     */
    final public function groups(): array
    {}

    /**
     * @param list<string> $groups
     *
     * @internal This method is not covered by the backward compatibility promise for PHPUnit
     */
    final public function setGroups(array $groups): void
    {}

    /**
     * @internal This method is not covered by the backward compatibility promise for PHPUnit
     */
    final public function nameWithDataSet(): string
    {}

    /**
     * @return non-empty-string
     *
     * @internal This method is not covered by the backward compatibility promise for PHPUnit
     */
    final public function name(): string
    {}

    /**
     * @internal This method is not covered by the backward compatibility promise for PHPUnit
     */
    final public function size(): TestSize
    {}

    /**
     * @internal This method is not covered by the backward compatibility promise for PHPUnit
     *
     * @phpstan-assert-if-true non-empty-string $this->output()
     */
    final public function hasUnexpectedOutput(): bool
    {}

    /**
     * @internal This method is not covered by the backward compatibility promise for PHPUnit
     */
    final public function output(): string
    {}

    /**
     * @internal This method is not covered by the backward compatibility promise for PHPUnit
     */
    final public function doesNotPerformAssertions(): bool
    {}

    /**
     * @internal This method is not covered by the backward compatibility promise for PHPUnit
     */
    final public function expectsOutput(): bool
    {}

    /**
     * @throws Throwable
     *
     * @internal This method is not covered by the backward compatibility promise for PHPUnit
     */
    final public function runLifecycle(): void
    {}

    /**
     * @param list<ExecutionOrderDependency> $dependencies
     *
     * @internal This method is not covered by the backward compatibility promise for PHPUnit
     */
    final public function setDependencies(array $dependencies): void
    {}

    /**
     * @param array<string, mixed> $dependencyInput
     *
     * @internal This method is not covered by the backward compatibility promise for PHPUnit
     */
    final public function setDependencyInput(array $dependencyInput): void
    {}

    /**
     * @return array<string, mixed>
     *
     * @internal This method is not covered by the backward compatibility promise for PHPUnit
     */
    final public function dependencyInput(): array
    {}

    /**
     * @internal This method is not covered by the backward compatibility promise for PHPUnit
     */
    final public function hasDependencyInput(): bool
    {}

    /**
     * @internal This method is not covered by the backward compatibility promise for PHPUnit
     */
    final public function setBackupGlobals(bool $backupGlobals): void
    {}

    /**
     * @param list<string> $backupGlobalsExcludeList
     *
     * @internal This method is not covered by the backward compatibility promise for PHPUnit
     */
    final public function setBackupGlobalsExcludeList(array $backupGlobalsExcludeList): void
    {}

    /**
     * @internal This method is not covered by the backward compatibility promise for PHPUnit
     */
    final public function setBackupStaticProperties(bool $backupStaticProperties): void
    {}

    /**
     * @param array<class-string, list<non-empty-string>> $backupStaticPropertiesExcludeList
     *
     * @internal This method is not covered by the backward compatibility promise for PHPUnit
     */
    final public function setBackupStaticPropertiesExcludeList(array $backupStaticPropertiesExcludeList): void
    {}

    /**
     * @internal This method is not covered by the backward compatibility promise for PHPUnit
     */
    final public function runsTestInSeparateProcess(): bool
    {}

    /**
     * @internal This method is not covered by the backward compatibility promise for PHPUnit
     */
    final public function setRunTestInSeparateProcess(bool $runTestInSeparateProcess): void
    {}

    /**
     * @internal This method is not covered by the backward compatibility promise for PHPUnit
     */
    final public function preservesGlobalState(): bool
    {}

    /**
     * @internal This method is not covered by the backward compatibility promise for PHPUnit
     */
    final public function setPreserveGlobalState(bool $preserveGlobalState): void
    {}

    /**
     * @internal This method is not covered by the backward compatibility promise for PHPUnit
     */
    final public function isInIsolation(): bool
    {}

    /**
     * @internal This method is not covered by the backward compatibility promise for PHPUnit
     *
     * @codeCoverageIgnore
     */
    final public function setInIsolation(bool $inIsolation): void
    {}

    /**
     * @internal This method is not covered by the backward compatibility promise for PHPUnit
     */
    final public function setEmptyDataProviderSkipMessage(string $message): void
    {}

    /**
     * @internal This method is not covered by the backward compatibility promise for PHPUnit
     */
    final public function setThrowableFromDeferredIssue(Throwable $throwable): void
    {}

    /**
     * @internal This method is not covered by the backward compatibility promise for PHPUnit
     */
    final public function result(): mixed
    {}

    /**
     * @internal This method is not covered by the backward compatibility promise for PHPUnit
     */
    final public function setResult(mixed $result): void
    {}

    /**
     * @param non-empty-string $type
     *
     * @internal This method is not covered by the backward compatibility promise for PHPUnit
     */
    final public function registerMockObject(string $type, MockObject $mockObject): void
    {}

    /**
     * @param non-negative-int $count
     *
     * @internal This method is not covered by the backward compatibility promise for PHPUnit
     */
    final public function addToAssertionCount(int $count): void
    {}

    /**
     * @return non-negative-int
     *
     * @internal This method is not covered by the backward compatibility promise for PHPUnit
     */
    final public function numberOfAssertionsPerformed(): int
    {}

    /**
     * @internal This method is not covered by the backward compatibility promise for PHPUnit
     */
    final public function usesDataProvider(): bool
    {}

    /**
     * @internal This method is not covered by the backward compatibility promise for PHPUnit
     */
    final public function dataName(): int|string
    {}

    /**
     * @internal This method is not covered by the backward compatibility promise for PHPUnit
     */
    final public function dataSetAsString(): string
    {}

    /**
     * @internal This method is not covered by the backward compatibility promise for PHPUnit
     */
    final public function dataSetAsStringWithData(): string
    {}

    /**
     * @return array<mixed>
     *
     * @internal This method is not covered by the backward compatibility promise for PHPUnit
     */
    final public function providedData(): array
    {}

    /**
     * @internal This method is not covered by the backward compatibility promise for PHPUnit
     */
    final public function sortId(): string
    {}

    /**
     * @return list<ExecutionOrderDependency>
     *
     * @internal This method is not covered by the backward compatibility promise for PHPUnit
     */
    final public function provides(): array
    {}

    /**
     * @return list<ExecutionOrderDependency>
     *
     * @internal This method is not covered by the backward compatibility promise for PHPUnit
     */
    final public function requires(): array
    {}

    /**
     * @param array<mixed> $data
     *
     * @internal This method is not covered by the backward compatibility promise for PHPUnit
     */
    final public function setData(int|string $dataName, array $data): void
    {}

    /**
     * @internal This method is not covered by the backward compatibility promise for PHPUnit
     */
    final public function valueObjectForEvents(): Event\Code\TestMethod
    {}

    /**
     * @internal This method is not covered by the backward compatibility promise for PHPUnit
     */
    final public function wasPrepared(): bool
    {}

    /**
     * @return positive-int
     *
     * @internal This method is not covered by the backward compatibility promise for PHPUnit
     */
    final public function repetition(): int
    {}

    /**
     * @return positive-int
     *
     * @internal This method is not covered by the backward compatibility promise for PHPUnit
     */
    final public function totalRepetitions(): int
    {}

    /**
     * @param positive-int $repetition
     * @param positive-int $totalRepetitions
     *
     * @internal This method is not covered by the backward compatibility promise for PHPUnit
     */
    final public function setRepetition(int $repetition, int $totalRepetitions): void
    {}

    /**
     * @return positive-int
     *
     * @internal This method is not covered by the backward compatibility promise for PHPUnit
     */
    final public function attempt(): int
    {}

    /**
     * @return positive-int
     *
     * @internal This method is not covered by the backward compatibility promise for PHPUnit
     */
    final public function maxAttempts(): int
    {}

    /**
     * @param positive-int $attempt
     * @param positive-int $maxAttempts
     *
     * @internal This method is not covered by the backward compatibility promise for PHPUnit
     */
    final public function setAttempt(int $attempt, int $maxAttempts): void
    {}

    /**
     * @internal This method is not covered by the backward compatibility promise for PHPUnit
     */
    final public function setStatus(TestStatus $status): void
    {}

    /**
     * Returns a matcher that matches when the method is executed
     * zero or more times.
     *
     * @deprecated https://github.com/sebastianbergmann/phpunit/issues/6461
     */
    final protected function any(): AnyInvokedCountMatcher
    {}

    /**
     * Returns a matcher that matches when the method is never executed.
     */
    final protected function never(): InvokedCountMatcher
    {}

    /**
     * Returns a matcher that matches when the method is executed
     * at least N times.
     */
    final protected function atLeast(int $requiredInvocations): InvokedAtLeastCountMatcher
    {}

    /**
     * Returns a matcher that matches when the method is executed at least once.
     */
    final protected function atLeastOnce(): InvokedAtLeastOnceMatcher
    {}

    /**
     * Returns a matcher that matches when the method is executed exactly once.
     */
    final protected function once(): InvokedCountMatcher
    {}

    /**
     * Returns a matcher that matches when the method is executed
     * exactly $count times.
     */
    final protected function exactly(int $count): InvokedCountMatcher
    {}

    /**
     * Returns a matcher that matches when the method is executed
     * at most N times.
     */
    final protected function atMost(int $allowedInvocations): InvokedAtMostCountMatcher
    {}

    final protected function throwException(Throwable $exception): ExceptionStub
    {}

    final protected function getActualOutputForAssertion(): string
    {}

    final protected function expectOutputRegex(string $expectedRegex): void
    {}

    final protected function expectOutputString(string $expectedString): void
    {}

    final protected function expectErrorLog(): void
    {}

    /**
     * @param class-string<Throwable> $exception
     */
    final protected function expectException(string $exception): void
    {}

    final protected function expectExceptionCode(int|string $code): void
    {}

    /**
     * @deprecated https://github.com/sebastianbergmann/phpunit/issues/6560
     */
    final protected function expectExceptionMessage(string $message): void
    {}

    final protected function expectExceptionMessageIs(string $message): void
    {}

    final protected function expectExceptionMessageIsOrContains(string $message): void
    {}

    final protected function expectExceptionMessageMatches(string $regularExpression): void
    {}

    /**
     * Sets up an expectation for an exception to be raised by the code under test.
     * Information for expected exception class, expected exception message, and
     * expected exception code are retrieved from a given Exception object.
     */
    final protected function expectExceptionObject(Throwable $exception): void
    {}

    final protected function expectNotToPerformAssertions(): void
    {}

    /**
     * @param non-empty-string $expectedUserDeprecationMessage
     */
    final protected function expectUserDeprecationMessage(string $expectedUserDeprecationMessage): void
    {}

    /**
     * @param non-empty-string $expectedUserDeprecationMessageRegularExpression
     */
    final protected function expectUserDeprecationMessageMatches(string $expectedUserDeprecationMessageRegularExpression): void
    {}

    /**
     * Returns a builder object to create mock objects using a fluent interface.
     *
     * @template RealInstanceType of object
     *
     * @param class-string<RealInstanceType> $className
     *
     * @return MockBuilder<RealInstanceType>
     */
    final protected function getMockBuilder(string $className): MockBuilder
    {}

    final protected function registerComparator(Comparator $comparator): void
    {}

    final protected function registerObjectExporter(ObjectExporter $objectExporter): void
    {}

    /**
     * @param class-string $classOrInterface
     */
    final protected function registerFailureType(string $classOrInterface): void
    {}

    /**
     * Creates a mock object for the specified interface or class.
     *
     * @template RealInstanceType of object
     *
     * @param class-string<RealInstanceType> $type
     *
     * @throws InvalidArgumentException
     * @throws MockObjectException
     * @throws NoPreviousThrowableException
     *
     * @return MockObject&RealInstanceType
     */
    final protected function createMock(string $type): MockObject
    {}

    /**
     * @param list<class-string> $interfaces
     *
     * @throws MockObjectException
     */
    final protected function createMockForIntersectionOfInterfaces(array $interfaces): MockObject
    {}

    /**
     * Creates a journal that the invocations of methods of mock objects can be
     * recorded in, in the order in which they happen.
     */
    final protected function createInvocationJournal(): InvocationJournal
    {}

    /**
     * Creates (and configures) a mock object for the specified interface or class.
     *
     * @template RealInstanceType of object
     *
     * @param class-string<RealInstanceType> $type
     * @param array<non-empty-string, mixed> $configuration
     *
     * @throws InvalidArgumentException
     * @throws MockObjectException
     * @throws NoPreviousThrowableException
     *
     * @return MockObject&RealInstanceType
     */
    final protected function createConfiguredMock(string $type, array $configuration): MockObject
    {}

    /**
     * Creates a partial mock object for the specified interface or class.
     *
     * @param class-string<RealInstanceType> $type
     * @param list<non-empty-string>         $methods
     *
     * @template RealInstanceType of object
     *
     * @throws InvalidArgumentException
     * @throws MockObjectException
     *
     * @return MockObject&RealInstanceType
     */
    final protected function createPartialMock(string $type, array $methods): MockObject
    {}

    /**
     * @param non-empty-string $additionalInformation
     */
    final protected function provideAdditionalInformation(string $additionalInformation): void
    {}

    protected function transformException(Throwable $t): Throwable
    {}

    /**
     * This method is called when a test method did not execute successfully.
     *
     * @throws Throwable
     */
    protected function onNotSuccessfulTest(Throwable $t): never
    {}

    /**
     * @param array<mixed> $testArguments
     */
    protected function invokeTestMethod(string $methodName, array $testArguments): mixed
    {}

    private function prepareEnvironment(): void
    {}

    private function restoreEnvironment(false|string $currentWorkingDirectory): void
    {}

    /**
     * A previously registered error handler may have turned an issue that
     * was triggered before this test was run, in a data provider for
     * example, into an exception: the exception is control flow of this
     * test and must be handled as if it was thrown while this test was
     * prepared.
     *
     * @see https://github.com/sebastianbergmann/phpunit/issues/6831
     *
     * @throws Throwable
     */
    private function throwThrowableFromDeferredIssue(): void
    {}

    /**
     * @param HookMethodsByType $hookMethods
     *
     * @throws Throwable
     */
    private function prepareTest(array $hookMethods): void
    {}

    /**
     * @param HookMethodsByType $hookMethods
     *
     * @throws Throwable
     */
    private function verifyTest(array $hookMethods): void
    {}

    /**
     * Sets the status of the test and emits the events for a throwable that
     * escaped from preparing, running, or verifying the test.
     *
     * Returns the throwable that is to be passed to onNotSuccessfulTest() or
     * null when the throwable is of a registered failure type.
     */
    private function handleThrowableFromTest(Throwable $t): ?Throwable
    {}

    private function stopOutputBuffering(): OutputBufferStopResult
    {}

    private function performOutputAssertions(): ?Throwable
    {}

    /**
     * Returns the throwable raised by the destructor of a mock object, if any.
     */
    private function discardMockObjects(): ?Throwable
    {}

    /**
     * Tears down the fixture. An exception raised in tearDown() is passed on
     * when no exception was raised before.
     *
     * @param HookMethodsByType $hookMethods
     */
    private function tearDownTest(array $hookMethods, ?Throwable $e): ?Throwable
    {}

    private function registerAsPassed(): void
    {}

    /**
     * @throws AssertionFailedError
     * @throws Exception
     * @throws ExpectationFailedException
     * @throws Throwable
     */
    private function runTest(): mixed
    {}

    /**
     * @throws SkippedTest
     */
    private function checkRequirements(): void
    {}

    private function isRegisteredFailure(Throwable $t): bool
    {}

    private function emitEventForCustomTestMethodInvocation(): void
    {}

    private function warnAboutConflictingOutputStringExpectation(string $expectedString): void
    {}

    /**
     * Returns a builder object to create test stubs using a fluent interface.
     *
     * @template RealInstanceType of object
     *
     * @param class-string<RealInstanceType> $className
     *
     * @return TestStubBuilder<RealInstanceType>
     */
    final protected static function getStubBuilder(string $className): TestStubBuilder
    {}

    /**
     * Creates a test stub for the specified interface or class.
     *
     * @template RealInstanceType of object
     *
     * @param class-string<RealInstanceType> $type
     *
     * @throws InvalidArgumentException
     * @throws MockObjectException
     * @throws NoPreviousThrowableException
     *
     * @return RealInstanceType&Stub
     */
    final protected static function createStub(string $type): Stub
    {}

    /**
     * @param list<class-string> $interfaces
     *
     * @throws MockObjectException
     */
    final protected static function createStubForIntersectionOfInterfaces(array $interfaces): Stub
    {}

    /**
     * Creates (and configures) a test stub for the specified interface or class.
     *
     * @template RealInstanceType of object
     *
     * @param class-string<RealInstanceType> $type
     * @param array<non-empty-string, mixed> $configuration
     *
     * @throws InvalidArgumentException
     * @throws MockObjectException
     * @throws NoPreviousThrowableException
     *
     * @return RealInstanceType&Stub
     */
    final protected static function createConfiguredStub(string $type, array $configuration): Stub
    {}

    private static function testDoubleFactory(): TestDoubleFactory
    {}
}
