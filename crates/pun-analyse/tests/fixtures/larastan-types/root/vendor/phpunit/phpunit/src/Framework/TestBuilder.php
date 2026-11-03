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

use function array_merge;
use function assert;
use function get_parent_class;
use function preg_match;
use function range;
use function sprintf;
use PHPUnit\Event\Emitter;
use PHPUnit\Metadata\Api\DataProvider;
use PHPUnit\Metadata\Api\Groups;
use PHPUnit\Metadata\Api\ProvidedData;
use PHPUnit\Metadata\Api\Requirements;
use PHPUnit\Metadata\BackupGlobals;
use PHPUnit\Metadata\BackupStaticProperties;
use PHPUnit\Metadata\ExcludeGlobalVariableFromBackup;
use PHPUnit\Metadata\ExcludeStaticPropertyFromBackup;
use PHPUnit\Metadata\Metadata;
use PHPUnit\Metadata\MetadataCollection;
use PHPUnit\Metadata\Parser\Registry as MetadataRegistry;
use PHPUnit\Metadata\PreserveGlobalState;
use PHPUnit\Metadata\Repeat as RepeatMetadata;
use PHPUnit\Metadata\Retry as RetryMetadata;
use PHPUnit\Runner\ErrorHandler;
use PHPUnit\Runner\Filter\CompiledNameFilter;
use PHPUnit\TextUI\Configuration\Registry as ConfigurationRegistry;
use ReflectionClass;
use ReflectionMethod;
use ReflectionNamedType;

/**
 * @no-named-arguments Parameter names are not covered by the backward compatibility promise for PHPUnit
 *
 * @internal This class is not covered by the backward compatibility promise for PHPUnit
 *
 * @phpstan-type BackupSettings array{backupGlobals: ?true, backupGlobalsExcludeList: list<string>, backupStaticProperties: ?true, backupStaticPropertiesExcludeList: array<class-string, list<non-empty-string>>}
 */
final readonly class TestBuilder
{
    private Emitter $emitter;

    public function __construct(Emitter $emitter)
    {}

    /**
     * @param ReflectionClass<TestCase> $theClass
     * @param non-empty-string          $methodName
     * @param list<non-empty-string>    $groups
     * @param positive-int              $numberOfRuns
     * @param positive-int              $failureThreshold
     * @param positive-int              $maxAttempts
     *
     * @throws InvalidDataProviderException
     */
    public function build(ReflectionClass $theClass, string $methodName, array $groups = [], int $numberOfRuns = 1, int $failureThreshold = 1, int $maxAttempts = 1): Test
    {}

    /**
     * @param non-empty-string       $methodName
     * @param class-string<TestCase> $className
     * @param array<ProvidedData>    $data
     * @param BackupSettings         $backupSettings
     * @param list<non-empty-string> $groups
     * @param positive-int           $numberOfRuns
     * @param positive-int           $failureThreshold
     * @param positive-int           $maxAttempts
     */
    private function buildDataProviderTestSuite(string $methodName, string $className, array $data, bool $runTestInSeparateProcess, ?bool $preserveGlobalState, array $backupSettings, array $groups, int $numberOfRuns = 1, int $failureThreshold = 1, int $maxAttempts = 1): DataProviderTestSuite
    {}

    /**
     * @param class-string<TestCase> $className
     * @param non-empty-string       $methodName
     * @param positive-int           $numberOfRuns
     * @param positive-int           $failureThreshold
     * @param BackupSettings         $backupSettings
     * @param list<non-empty-string> $groups
     */
    private function buildRepeatTestSuite(string $className, string $methodName, int $numberOfRuns, int $failureThreshold, bool $runTestInSeparateProcess, ?bool $preserveGlobalState, array $backupSettings, array $groups): RepeatTestSuite
    {}

    /**
     * @param class-string<TestCase> $className
     * @param non-empty-string       $methodName
     * @param positive-int           $maxAttempts
     * @param BackupSettings         $backupSettings
     * @param list<non-empty-string> $groups
     */
    private function buildRetryTestSuite(string $className, string $methodName, int $maxAttempts, bool $runTestInSeparateProcess, ?bool $preserveGlobalState, array $backupSettings, array $groups): RetryTestSuite
    {}

    /**
     * @param BackupSettings $backupSettings
     */
    private function configureTestCase(TestCase $test, bool $runTestInSeparateProcess, ?bool $preserveGlobalState, array $backupSettings): void
    {}

    /**
     * @param class-string<TestCase> $className
     * @param non-empty-string       $methodName
     *
     * @return BackupSettings
     */
    private function backupSettings(string $className, string $methodName): array
    {}

    /**
     * @param class-string<TestCase> $className
     * @param non-empty-string       $methodName
     */
    private function shouldGlobalStateBePreserved(string $className, string $methodName): ?bool
    {}

    /**
     * Metadata on a test method takes precedence over metadata on the class
     * that declares it.
     */
    private function methodOrClassLevelMetadata(MetadataCollection $forMethod, MetadataCollection $forClass): ?Metadata
    {}

    /**
     * @param class-string<TestCase> $className
     * @param non-empty-string       $methodName
     */
    private function shouldTestMethodBeRunInSeparateProcess(string $className, string $methodName): bool
    {}

    /**
     * @param class-string     $className
     * @param non-empty-string $methodName
     */
    private function requirementsSatisfied(string $className, string $methodName): bool
    {}

    /**
     * @param class-string<TestCase> $className
     * @param non-empty-string       $methodName
     */
    private function filterExcludesMethod(string $className, string $methodName): bool
    {}

    /**
     * A test method is eligible for repetition and retrying when it declares
     * an explicit void return type and does not depend on another test.
     *
     * @param ReflectionClass<TestCase> $theClass
     * @param class-string<TestCase>    $className
     * @param non-empty-string          $methodName
     */
    private function isEligibleForRepeatOrRetry(ReflectionClass $theClass, string $className, string $methodName): bool
    {}

    /**
     * @param 'Repeat'|'Retry'          $attribute
     * @param 'repeated'|'retried'      $verb
     * @param ReflectionClass<TestCase> $theClass
     * @param class-string<TestCase>    $className
     * @param non-empty-string          $methodName
     */
    private function warnWhenMethodIsIneligible(string $attribute, string $verb, ReflectionClass $theClass, string $className, string $methodName): void
    {}

    private function hasVoidReturnType(ReflectionMethod $method): bool
    {}

    /**
     * @param class-string     $className
     * @param non-empty-string $methodName
     */
    private function doesNotDependOnAnotherTest(string $className, string $methodName): bool
    {}
}
