<?php declare(strict_types=1);
/*
 * This file is part of PHPUnit.
 *
 * (c) Sebastian Bergmann <sebastian@phpunit.de>
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */
namespace PHPUnit\Runner;

use const DEBUG_BACKTRACE_IGNORE_ARGS;
use const E_COMPILE_ERROR;
use const E_COMPILE_WARNING;
use const E_CORE_ERROR;
use const E_CORE_WARNING;
use const E_DEPRECATED;
use const E_ERROR;
use const E_NOTICE;
use const E_PARSE;
use const E_RECOVERABLE_ERROR;
use const E_USER_DEPRECATED;
use const E_USER_ERROR;
use const E_USER_NOTICE;
use const E_USER_WARNING;
use const E_WARNING;
use function array_any;
use function array_reverse;
use function array_slice;
use function array_unshift;
use function array_values;
use function assert;
use function count;
use function debug_backtrace;
use function defined;
use function error_reporting;
use function is_callable;
use function preg_match;
use function restore_error_handler;
use function set_error_handler;
use function sprintf;
use PHPUnit\Event;
use PHPUnit\Event\Code\IssueTrigger\Code;
use PHPUnit\Event\Code\IssueTrigger\IssueTrigger;
use PHPUnit\Event\Code\NoTestCaseObjectOnCallStackException;
use PHPUnit\Event\Code\TestMethod;
use PHPUnit\Framework\TestCase;
use PHPUnit\Metadata\IgnoreDeprecations;
use PHPUnit\Metadata\Parser\Registry as MetadataParserRegistry;
use PHPUnit\Runner\Baseline\Baseline;
use PHPUnit\Runner\Baseline\Issue;
use PHPUnit\Runner\IssueTriggerResolver\DefaultResolver as DefaultIssueTriggerResolver;
use PHPUnit\Runner\IssueTriggerResolver\Resolver as IssueTriggerResolver;
use PHPUnit\TextUI\Configuration\Registry as ConfigurationRegistry;
use PHPUnit\TextUI\Configuration\SourceFilter;
use PHPUnit\Util\ExcludeList;
use Throwable;

/**
 * @no-named-arguments Parameter names are not covered by the backward compatibility promise for PHPUnit
 *
 * @internal This class is not covered by the backward compatibility promise for PHPUnit
 *
 * @phpstan-type DeprecationMethod array{className: string, methodName: non-empty-string}
 * @phpstan-type DeprecationTriggers array{functions: list<non-empty-string>, methods: list<DeprecationMethod>}
 * @phpstan-type StackFrame array{function: string, line?: int, file?: string, class?: class-string, type?: '->'|'::', args?: list<mixed>, object?: object}
 */
final class ErrorHandler
{
    private const int UNHANDLEABLE_LEVELS   = E_ERROR | E_PARSE | E_CORE_ERROR | E_CORE_WARNING | E_COMPILE_ERROR | E_COMPILE_WARNING;
    private const int INSUPPRESSIBLE_LEVELS = E_ERROR | E_PARSE | E_CORE_ERROR | E_COMPILE_ERROR | E_USER_ERROR | E_RECOVERABLE_ERROR;
    private static ?self $instance          = null;
    private ?Baseline $baseline             = null;
    private ExcludeList $excludeList;
    private bool $enabled                          = false;
    private ?int $originalErrorReportingLevel      = null;
    private ?int $deferredIssueErrorReportingLevel = null;

    /**
     * @var ?array{int, string, string, int}
     */
    private ?array $forwardedError = null;

    /**
     * @var ?callable
     */
    private mixed $previousErrorHandler = null;

    /**
     * @var ?callable
     */
    private mixed $previousNonTestCaseErrorHandler = null;
    private readonly bool $identifyIssueTrigger;

    /**
     * @var array<string, list<array{int, string, string, int, int}>>
     */
    private array $testCaseContextIssues = [];
    private ?string $testCaseContext     = null;
    private readonly Event\Emitter $emitter;

    /**
     * @var ?list<callable>
     */
    private ?array $backupErrorHandlers = null;

    /**
     * @var ?DeprecationTriggers
     */
    private ?array $deprecationTriggers = null;

    /**
     * @var non-empty-list<IssueTriggerResolver>
     */
    private array $issueTriggerResolvers;

    /**
     * @var list<DeprecationFilter>
     */
    private array $deprecationFilters = [];

    public static function instance(): self
    {}

    public function __construct(bool $identifyIssueTrigger, Event\Emitter $emitter)
    {}

    /**
     * @throws NoTestCaseObjectOnCallStackException
     */
    public function __invoke(int $errorNumber, string $errorString, string $errorFile, int $errorLine): bool
    {}

    public function handleNonTestCaseIssue(int $errorNumber, string $errorString, string $errorFile, int $errorLine): true
    {}

    public function registerForNonTestCaseContext(): void
    {}

    public function restoreForNonTestCaseContext(): void
    {}

    public function enable(TestCase $test): ?Throwable
    {}

    public function disable(): void
    {}

    /**
     * @return list<non-empty-string>
     */
    public function snapshotErrorHandlers(): array
    {}

    /**
     * @return list<non-empty-string>
     */
    public function restoreErrorHandlers(bool $inIsolation): array
    {}

    public function useBaseline(Baseline $baseline): void
    {}

    /**
     * @param DeprecationTriggers $deprecationTriggers
     */
    public function useDeprecationTriggers(array $deprecationTriggers): void
    {}

    public function addIssueTriggerResolver(IssueTriggerResolver $resolver): void
    {}

    public function addDeprecationFilter(DeprecationFilter $filter): void
    {}

    public function enterTestCaseContext(string $className, string $methodName): void
    {}

    public function leaveTestCaseContext(): void
    {}

    /**
     * @param non-empty-string $file
     * @param positive-int     $line
     * @param non-empty-string $description
     */
    private function ignoredByBaseline(string $file, int $line, string $description): bool
    {}

    /**
     * @param null|non-empty-string $errorFile
     */
    private function triggerWithoutTest(bool $isUserland, string $errorString, ?string $errorFile = null): IssueTrigger
    {}

    /**
     * @param null|non-empty-string $errorFile
     */
    private function trigger(TestMethod $test, bool $isUserland, string $errorString, ?string $errorFile = null): IssueTrigger
    {}

    /**
     * @param list<StackFrame> $trace
     */
    private function triggerForUserlandDeprecation(TestMethod $test, string $message, array $trace): IssueTrigger
    {}

    /**
     * @param non-empty-string $file
     */
    private function categorizeFile(string $file, TestMethod $test): Code
    {}

    /**
     * @param non-empty-string $file
     */
    private function categorizeFileWithoutTest(string $file): Code
    {}

    /**
     * @param list<StackFrame> $trace
     */
    private function triggerForUserlandDeprecationWithoutTest(string $message, array $trace): IssueTrigger
    {}

    /**
     * @return list<StackFrame>
     */
    private function filteredStackTrace(): array
    {}

    /**
     * @return ?StackFrame
     */
    private function guessDeprecationFrame(): ?array
    {}

    /**
     * Finds the frame of the outermost of the (consecutive) configured deprecation
     * trigger functions and methods that the deprecation was triggered through.
     *
     * The file and line of this frame point to the code that called into the
     * deprecation trigger, in other words the location where the deprecation
     * was actually triggered.
     *
     * @param list<StackFrame> $trace
     *
     * @return ?non-negative-int
     */
    private function deprecationTriggerFramePosition(array $trace): ?int
    {}

    /**
     * @param StackFrame $frame
     */
    private function frameMatchesDeprecationTrigger(array $frame): bool
    {}

    /**
     * @param non-empty-string $errorFile
     * @param positive-int     $errorLine
     *
     * @return array{non-empty-string, positive-int}
     */
    private function applyDeprecationFrame(string $errorFile, int $errorLine): array
    {}

    /**
     * @return list<StackFrame>
     */
    private function errorStackTrace(bool $ignoreArgs = true): array
    {}

    /**
     * @param StackFrame       $frame
     * @param non-empty-string $function
     */
    private function frameIsFunction(array $frame, string $function): bool
    {}

    /**
     * @param StackFrame        $frame
     * @param DeprecationMethod $method
     */
    private function frameIsMethod(array $frame, array $method): bool
    {}

    /**
     * @param non-empty-string $errorFile
     * @param positive-int     $errorLine
     *
     * @return non-empty-string
     */
    private function stackTrace(string $errorFile, int $errorLine): string
    {}

    private function triggerTestCaseContextIssues(TestCase $test): ?Throwable
    {}

    private function testCaseContext(string $className, string $methodName): string
    {}

    /**
     * @param list<non-empty-string> $messages
     *
     * @return list<callable>
     */
    private function activeErrorHandlers(array &$messages = []): array
    {}

    /**
     * @param list<callable> $handlers
     *
     * @return list<callable>
     */
    private function handlersAboveSelf(array $handlers): array
    {}

    /**
     * @param list<callable> $handlers
     */
    private function isOnStack(array $handlers): bool
    {}

    private function deprecationIgnoredByTest(TestMethod $test, string $message): bool
    {}

    private function deprecationIgnoredByFilter(string $message, string $file, int $line, IssueTrigger $trigger): bool
    {}

    private function forwardToPreviousErrorHandler(int $errorNumber, string $errorString, string $errorFile, int $errorLine): bool
    {}

    private function forwardToPreviousNonTestCaseErrorHandler(int $errorNumber, string $errorString, string $errorFile, int $errorLine): void
    {}
}
