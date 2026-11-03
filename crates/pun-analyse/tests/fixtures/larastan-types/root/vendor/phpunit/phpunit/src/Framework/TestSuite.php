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
use function array_all;
use function array_merge;
use function array_pop;
use function array_reverse;
use function assert;
use function call_user_func;
use function count;
use function implode;
use function is_callable;
use function is_file;
use function is_subclass_of;
use function sprintf;
use function str_ends_with;
use function str_starts_with;
use function trim;
use Iterator;
use IteratorAggregate;
use PHPUnit\Event;
use PHPUnit\Event\Code\TestMethod;
use PHPUnit\Event\NoPreviousThrowableException;
use PHPUnit\Metadata\Api\Dependencies;
use PHPUnit\Metadata\Api\Groups;
use PHPUnit\Metadata\Api\HookMethods;
use PHPUnit\Metadata\Api\Requirements;
use PHPUnit\Metadata\InvalidAttribute;
use PHPUnit\Metadata\MetadataCollection;
use PHPUnit\Metadata\Parser\Registry as MetadataRegistry;
use PHPUnit\Runner\Exception as RunnerException;
use PHPUnit\Runner\Filter\Factory;
use PHPUnit\Runner\Phpt\TestCase as PhptTestCase;
use PHPUnit\Runner\TestSuiteLoader;
use PHPUnit\TestRunner\TestResult\Facade as TestResultFacade;
use PHPUnit\Util\Filter;
use PHPUnit\Util\Reflection;
use PHPUnit\Util\Test as TestUtil;
use ReflectionClass;
use ReflectionMethod;
use SebastianBergmann\CodeCoverage\InvalidArgumentException;
use SebastianBergmann\CodeCoverage\UnintentionallyCoveredCodeException;
use Throwable;

/**
 * @template-implements IteratorAggregate<non-negative-int, Test>
 *
 * @no-named-arguments Parameter names are not covered by the backward compatibility promise for PHPUnit
 *
 * @internal This class is not covered by the backward compatibility promise for PHPUnit
 */
class TestSuite implements IteratorAggregate, Reorderable, Test
{
    /**
     * @var non-empty-string
     */
    private string $name;

    /**
     * The name of a group that is a number is an integer key.
     *
     * @var array<int|non-empty-string, list<PhptTestCase|TestCase>>
     */
    private array $groupedTests = [];

    /**
     * The name of a group that is a number is an integer key.
     *
     * @var ?array<int|non-empty-string, list<non-empty-string>>
     */
    private ?array $groups = null;

    /**
     * @var ?list<ExecutionOrderDependency>
     */
    private ?array $requiredTests = null;

    /**
     * @var list<Test>
     */
    private array $tests = [];

    /**
     * @var ?list<ExecutionOrderDependency>
     */
    private ?array $providedTests    = null;
    private ?Factory $iteratorFilter = null;
    private bool $wasRun             = false;
    private Event\Emitter $emitter;

    /**
     * Whether this test suite holds the tests of a test class, which is only
     * known from how it was created: a test suite that merely has the name of
     * a test class, such as a test suite of the XML configuration file that
     * is named like the test class it holds, is not for that test class.
     */
    private bool $isForTestClass = false;

    /**
     * @param non-empty-string $name
     */
    public static function empty(string $name, Event\Emitter $emitter): static
    {}

    /**
     * @param ReflectionClass<TestCase> $class
     * @param list<non-empty-string>    $groups
     * @param positive-int              $numberOfRuns
     * @param positive-int              $maxAttempts
     */
    public static function fromClassReflector(ReflectionClass $class, Event\Emitter $emitter, array $groups = [], int $numberOfRuns = 1, int $maxAttempts = 1): static
    {}

    /**
     * @param non-empty-string $name
     */
    final private function __construct(string $name, Event\Emitter $emitter)
    {}

    /**
     * Adds a test to the suite.
     *
     * @param list<non-empty-string> $groups
     */
    public function addTest(Test $test, array $groups = []): void
    {}

    /**
     * Adds the tests from the given class to the suite.
     *
     * @param ReflectionClass<TestCase> $testClass
     * @param list<non-empty-string>    $groups
     * @param positive-int              $numberOfRuns
     * @param positive-int              $maxAttempts
     *
     * @throws Exception
     */
    public function addTestSuite(ReflectionClass $testClass, array $groups = [], int $numberOfRuns = 1, int $maxAttempts = 1): void
    {}

    /**
     * Wraps both <code>addTest()</code> and <code>addTestSuite</code>
     * as well as the separate import statements for the user's convenience.
     *
     * If the named file cannot be read or there are no new tests that can be
     * added, a <code>PHPUnit\Framework\WarningTestCase</code> will be created instead,
     * leaving the current test run untouched.
     *
     * @param list<non-empty-string> $groups
     * @param positive-int           $numberOfRuns
     * @param positive-int           $maxAttempts
     *
     * @throws Exception
     */
    public function addTestFile(string $filename, array $groups = [], int $numberOfRuns = 1, int $maxAttempts = 1): void
    {}

    /**
     * Wrapper for addTestFile() that adds multiple test files.
     *
     * @param iterable<string> $fileNames
     * @param positive-int     $numberOfRuns
     * @param positive-int     $maxAttempts
     *
     * @throws Exception
     */
    public function addTestFiles(iterable $fileNames, int $numberOfRuns = 1, int $maxAttempts = 1): void
    {}

    /**
     * Counts the number of test cases that will be run by this test.
     */
    public function count(): int
    {}

    public function isEmpty(): bool
    {}

    /**
     * @return non-empty-string
     */
    public function name(): string
    {}

    /**
     * The identifiers of the tests are only determined when they are asked
     * for, which is only the case when tests are filtered by group. A test
     * that is run builds the event value object that provides its identifier
     * anyway, and caches it, so determining the identifiers eagerly only adds
     * work for tests that are never run.
     *
     * The name of a group that is a number is an integer key.
     *
     * @return array<int|non-empty-string, list<non-empty-string>>
     */
    public function groups(): array
    {}

    /**
     * The repetitions of a repeated test and the attempts of a retried test
     * are aggregated by an IterativeTestSuite whose runTests() method
     * implements the repetition and retry logic. Running the test cases
     * returned by this method individually bypasses this logic: an
     * IterativeTestSuite must be treated as an atomic unit.
     *
     * @return list<PhptTestCase|TestCase>
     */
    public function collect(): array
    {}

    /**
     * @throws Event\RuntimeException
     * @throws Exception
     * @throws InvalidArgumentException
     * @throws NoPreviousThrowableException
     * @throws UnintentionallyCoveredCodeException
     */
    public function run(): void
    {}

    /**
     * Returns the tests aggregated by this test suite: all of them, and not
     * only those that the test selection has selected.
     *
     * The test selection that --filter, --group, --exclude-group, and
     * --filter-test-id configure is a filter iterator that injectFilter()
     * puts on the test suite, so it only takes effect while the test suite is
     * iterated. Iterate the test suite wherever the selected tests are
     * wanted, as count(), isEmpty(), and the takeTests() that run() uses do.
     *
     * The filter accepts every test suite it is asked about and applies the
     * selection to the tests inside it, so iterating can yield a test suite
     * that the selection has emptied.
     *
     * @return list<Test>
     */
    public function tests(): array
    {}

    /**
     * Set tests of the test suite.
     *
     * @param list<Test> $tests
     */
    public function setTests(array $tests): void
    {}

    /**
     * Mark the test suite as skipped.
     *
     * @throws SkippedTestSuiteError
     */
    public function markTestSuiteSkipped(string $message = ''): never
    {}

    /**
     * @return Iterator<non-negative-int, Test>
     */
    public function getIterator(): Iterator
    {}

    public function injectFilter(Factory $filter): void
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
     * @phpstan-assert-if-true class-string<TestCase> $this->name
     */
    public function isForTestClass(): bool
    {}

    /**
     * Runs the tests aggregated by this test suite.
     *
     * @param list<Test> $tests
     *
     * @throws Event\RuntimeException
     * @throws Exception
     * @throws InvalidArgumentException
     * @throws NoPreviousThrowableException
     * @throws UnintentionallyCoveredCodeException
     */
    protected function runTests(array $tests, Event\Emitter $emitter): void
    {}

    /**
     * @param ReflectionClass<TestCase> $class
     * @param list<non-empty-string>    $groups
     * @param positive-int              $numberOfRuns
     * @param positive-int              $maxAttempts
     *
     * @throws Exception
     */
    protected function addTestMethod(ReflectionClass $class, ReflectionMethod $method, array $groups, int $numberOfRuns = 1, int $maxAttempts = 1): void
    {}

    /**
     * Removes the tests aggregated by this test suite from it and returns them.
     *
     * @return list<Test>
     */
    private function takeTests(): array
    {}

    /**
     * @param class-string     $className
     * @param non-empty-string $methodName
     *
     * @return list<non-empty-string>
     */
    private function metadataErrorsFor(string $className, string $methodName): array
    {}

    private function clearCaches(): void
    {}

    /**
     * @param list<non-empty-string> $groups
     */
    private function containsOnlyVirtualGroups(array $groups): bool
    {}

    /**
     * @throws Exception
     */
    private function exceptionToString(InvalidDataProviderException $e): string
    {}

    /**
     * @throws Exception
     * @throws NoPreviousThrowableException
     */
    private function invokeMethodsBeforeFirstTest(Event\Emitter $emitter, Event\TestSuite\TestSuite $testSuiteValueObjectForEvents): bool
    {}

    private function invokeMethodsAfterLastTest(Event\Emitter $emitter): void
    {}
}
