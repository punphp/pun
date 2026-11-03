<?php declare(strict_types=1);
/*
 * This file is part of PHPUnit.
 *
 * (c) Sebastian Bergmann <sebastian@phpunit.de>
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */
namespace PHPUnit\Metadata;

use Closure;
use PHPUnit\Metadata\Version\Requirement;
use PHPUnit\Runner\Extension\Extension;

/**
 * @immutable
 *
 * @no-named-arguments Parameter names are not covered by the backward compatibility promise for PHPUnit
 */
abstract readonly class Metadata
{
    private Level $level;

    public static function after(int $priority): After
    {}

    public static function afterClass(int $priority): AfterClass
    {}

    public static function allowMockObjectsWithoutExpectationsOnClass(): AllowMockObjectsWithoutExpectations
    {}

    public static function allowMockObjectsWithoutExpectationsOnMethod(): AllowMockObjectsWithoutExpectations
    {}

    public static function backupGlobalsOnClass(bool $enabled): BackupGlobals
    {}

    public static function backupGlobalsOnMethod(bool $enabled): BackupGlobals
    {}

    public static function backupStaticPropertiesOnClass(bool $enabled): BackupStaticProperties
    {}

    public static function backupStaticPropertiesOnMethod(bool $enabled): BackupStaticProperties
    {}

    public static function before(int $priority): Before
    {}

    public static function beforeClass(int $priority): BeforeClass
    {}

    /**
     * @param non-empty-string $namespace
     */
    public static function coversNamespace(string $namespace): CoversNamespace
    {}

    /**
     * @param class-string $className
     */
    public static function coversClass(string $className): CoversClass
    {}

    /**
     * @param class-string $className
     */
    public static function coversClassesThatExtendClass(string $className): CoversClassesThatExtendClass
    {}

    /**
     * @param class-string $interfaceName
     */
    public static function coversClassesThatImplementInterface(string $interfaceName): CoversClassesThatImplementInterface
    {}

    /**
     * @param trait-string $traitName
     */
    public static function coversTrait(string $traitName): CoversTrait
    {}

    /**
     * @param class-string     $className
     * @param non-empty-string $methodName
     */
    public static function coversMethod(string $className, string $methodName): CoversMethod
    {}

    /**
     * @param non-empty-string $functionName
     */
    public static function coversFunction(string $functionName): CoversFunction
    {}

    /**
     * @param non-empty-string $path
     */
    public static function coversFile(string $path): CoversFile
    {}

    /**
     * @param non-empty-string $directory
     */
    public static function coversDirectory(string $directory): CoversDirectory
    {}

    /**
     * @param non-empty-string $directory
     */
    public static function coversDirectoryRecursively(string $directory): CoversDirectoryRecursively
    {}

    public static function coversNothingOnClass(): CoversNothing
    {}

    public static function coversNothingOnMethod(): CoversNothing
    {}

    /**
     * @param class-string     $className
     * @param non-empty-string $methodName
     */
    public static function dataProvider(string $className, string $methodName, bool $validateArgumentCount, bool $skipWhenEmpty): DataProvider
    {}

    public static function dataProviderClosure(Closure $callable, bool $validateArgumentCount): DataProviderClosure
    {}

    /**
     * @param class-string $className
     */
    public static function dependsOnClass(string $className, bool $deepClone, bool $shallowClone): DependsOnClass
    {}

    /**
     * @param class-string     $className
     * @param non-empty-string $methodName
     */
    public static function dependsOnMethod(string $className, string $methodName, bool $deepClone, bool $shallowClone): DependsOnMethod
    {}

    public static function disableReturnValueGenerationForTestDoubles(): DisableReturnValueGenerationForTestDoubles
    {}

    public static function doesNotPerformAssertionsOnClass(): DoesNotPerformAssertions
    {}

    public static function doesNotPerformAssertionsOnMethod(): DoesNotPerformAssertions
    {}

    /**
     * @param non-empty-string $globalVariableName
     */
    public static function excludeGlobalVariableFromBackupOnClass(string $globalVariableName): ExcludeGlobalVariableFromBackup
    {}

    /**
     * @param non-empty-string $globalVariableName
     */
    public static function excludeGlobalVariableFromBackupOnMethod(string $globalVariableName): ExcludeGlobalVariableFromBackup
    {}

    /**
     * @param class-string     $className
     * @param non-empty-string $propertyName
     */
    public static function excludeStaticPropertyFromBackupOnClass(string $className, string $propertyName): ExcludeStaticPropertyFromBackup
    {}

    /**
     * @param class-string     $className
     * @param non-empty-string $propertyName
     */
    public static function excludeStaticPropertyFromBackupOnMethod(string $className, string $propertyName): ExcludeStaticPropertyFromBackup
    {}

    /**
     * @param non-empty-string $groupName
     */
    public static function groupOnClass(string $groupName): Group
    {}

    /**
     * @param non-empty-string $groupName
     */
    public static function groupOnMethod(string $groupName): Group
    {}

    /**
     * @param null|non-empty-string $messagePattern
     */
    public static function ignoreDeprecationsOnClass(?string $messagePattern = null): IgnoreDeprecations
    {}

    /**
     * @param null|non-empty-string $messagePattern
     */
    public static function ignoreDeprecationsOnMethod(?string $messagePattern = null): IgnoreDeprecations
    {}

    /**
     * @internal This method is not covered by the backward compatibility promise for PHPUnit
     */
    public static function ignorePhpunitDeprecationsOnClass(): IgnorePhpunitDeprecations
    {}

    /**
     * @internal This method is not covered by the backward compatibility promise for PHPUnit
     */
    public static function ignorePhpunitDeprecationsOnMethod(): IgnorePhpunitDeprecations
    {}

    /**
     * @param non-empty-string $message
     */
    public static function invalidAttributeOnClass(string $message): InvalidAttribute
    {}

    /**
     * @param non-empty-string $message
     */
    public static function invalidAttributeOnMethod(string $message): InvalidAttribute
    {}

    public static function postCondition(int $priority): PostCondition
    {}

    public static function preCondition(int $priority): PreCondition
    {}

    public static function preserveGlobalStateOnClass(bool $enabled): PreserveGlobalState
    {}

    public static function preserveGlobalStateOnMethod(bool $enabled): PreserveGlobalState
    {}

    /**
     * @param non-empty-string $functionName
     */
    public static function requiresFunctionOnClass(string $functionName): RequiresFunction
    {}

    /**
     * @param non-empty-string $functionName
     */
    public static function requiresFunctionOnMethod(string $functionName): RequiresFunction
    {}

    /**
     * @param non-empty-string $className
     */
    public static function requiresClassOnClass(string $className): RequiresClass
    {}

    /**
     * @param non-empty-string $className
     */
    public static function requiresClassOnMethod(string $className): RequiresClass
    {}

    /**
     * @param non-empty-string $className
     * @param non-empty-string $methodName
     */
    public static function requiresMethodOnClass(string $className, string $methodName): RequiresMethod
    {}

    /**
     * @param non-empty-string $className
     * @param non-empty-string $methodName
     */
    public static function requiresMethodOnMethod(string $className, string $methodName): RequiresMethod
    {}

    /**
     * @param non-empty-string $operatingSystem
     */
    public static function requiresOperatingSystemOnClass(string $operatingSystem): RequiresOperatingSystem
    {}

    /**
     * @param non-empty-string $operatingSystem
     */
    public static function requiresOperatingSystemOnMethod(string $operatingSystem): RequiresOperatingSystem
    {}

    /**
     * @param non-empty-string $operatingSystemFamily
     */
    public static function requiresOperatingSystemFamilyOnClass(string $operatingSystemFamily): RequiresOperatingSystemFamily
    {}

    /**
     * @param non-empty-string $operatingSystemFamily
     */
    public static function requiresOperatingSystemFamilyOnMethod(string $operatingSystemFamily): RequiresOperatingSystemFamily
    {}

    public static function requiresPhpOnClass(Requirement $versionRequirement): RequiresPhp
    {}

    public static function requiresPhpOnMethod(Requirement $versionRequirement): RequiresPhp
    {}

    /**
     * @param non-empty-string $extension
     */
    public static function requiresPhpExtensionOnClass(string $extension, ?Requirement $versionRequirement): RequiresPhpExtension
    {}

    /**
     * @param non-empty-string $extension
     */
    public static function requiresPhpExtensionOnMethod(string $extension, ?Requirement $versionRequirement): RequiresPhpExtension
    {}

    public static function requiresPhpunitOnClass(Requirement $versionRequirement): RequiresPhpunit
    {}

    public static function requiresPhpunitOnMethod(Requirement $versionRequirement): RequiresPhpunit
    {}

    /**
     * @param class-string<Extension> $extensionClass
     */
    public static function requiresPhpunitExtensionOnClass(string $extensionClass): RequiresPhpunitExtension
    {}

    /**
     * @param class-string<Extension> $extensionClass
     */
    public static function requiresPhpunitExtensionOnMethod(string $extensionClass): RequiresPhpunitExtension
    {}

    /**
     * @param non-empty-string $environmentVariableName
     */
    public static function requiresEnvironmentVariableOnClass(string $environmentVariableName, null|string $value): RequiresEnvironmentVariable
    {}

    /**
     * @param non-empty-string $environmentVariableName
     */
    public static function requiresEnvironmentVariableOnMethod(string $environmentVariableName, null|string $value): RequiresEnvironmentVariable
    {}

    /**
     * @param non-empty-string $environmentVariableName
     */
    public static function withEnvironmentVariableOnClass(string $environmentVariableName, null|string $value): WithEnvironmentVariable
    {}

    /**
     * @param non-empty-string $environmentVariableName
     */
    public static function withEnvironmentVariableOnMethod(string $environmentVariableName, null|string $value): WithEnvironmentVariable
    {}

    /**
     * @param non-empty-string $setting
     * @param non-empty-string $value
     */
    public static function requiresSettingOnClass(string $setting, string $value): RequiresSetting
    {}

    /**
     * @param non-empty-string $setting
     * @param non-empty-string $value
     */
    public static function requiresSettingOnMethod(string $setting, string $value): RequiresSetting
    {}

    public static function runTestsInSeparateProcesses(): RunTestsInSeparateProcesses
    {}

    /**
     * @param positive-int $times
     * @param positive-int $failureThreshold
     */
    public static function repeat(int $times, int $failureThreshold): Repeat
    {}

    /**
     * @param positive-int $maxAttempts
     */
    public static function retry(int $maxAttempts): Retry
    {}

    public static function runInSeparateProcess(): RunInSeparateProcess
    {}

    public static function test(): Test
    {}

    /**
     * @param non-empty-string $text
     */
    public static function testDoxOnClass(string $text): TestDox
    {}

    /**
     * @param non-empty-string $text
     */
    public static function testDoxOnMethod(string $text): TestDox
    {}

    /**
     * @param class-string     $className
     * @param non-empty-string $methodName
     */
    public static function testDoxFormatter(string $className, string $methodName): TestDoxFormatter
    {}

    /**
     * @param ?non-empty-string $name
     */
    public static function testWith(mixed $data, ?string $name = null): TestWith
    {}

    /**
     * @param non-empty-string $namespace
     */
    public static function usesNamespace(string $namespace): UsesNamespace
    {}

    /**
     * @param class-string $className
     */
    public static function usesClass(string $className): UsesClass
    {}

    /**
     * @param class-string $className
     */
    public static function usesClassesThatExtendClass(string $className): UsesClassesThatExtendClass
    {}

    /**
     * @param class-string $interfaceName
     */
    public static function usesClassesThatImplementInterface(string $interfaceName): UsesClassesThatImplementInterface
    {}

    /**
     * @param trait-string $traitName
     */
    public static function usesTrait(string $traitName): UsesTrait
    {}

    /**
     * @param non-empty-string $functionName
     */
    public static function usesFunction(string $functionName): UsesFunction
    {}

    /**
     * @param class-string     $className
     * @param non-empty-string $methodName
     */
    public static function usesMethod(string $className, string $methodName): UsesMethod
    {}

    /**
     * @param non-empty-string $path
     */
    public static function usesFile(string $path): UsesFile
    {}

    /**
     * @param non-empty-string $directory
     */
    public static function usesDirectory(string $directory): UsesDirectory
    {}

    /**
     * @param non-empty-string $directory
     */
    public static function usesDirectoryRecursively(string $directory): UsesDirectoryRecursively
    {}

    public static function withoutErrorHandler(): WithoutErrorHandler
    {}

    /**
     * @param null|non-empty-string $messagePattern
     */
    public static function ignorePhpunitWarnings(?string $messagePattern): IgnorePhpunitWarnings
    {}

    protected function __construct(Level $level)
    {}

    public function isClassLevel(): bool
    {}

    public function isMethodLevel(): bool
    {}

    /**
     * @phpstan-assert-if-true After $this
     */
    public function isAfter(): bool
    {}

    /**
     * @phpstan-assert-if-true AfterClass $this
     */
    public function isAfterClass(): bool
    {}

    /**
     * @phpstan-assert-if-true AllowMockObjectsWithoutExpectations $this
     */
    public function isAllowMockObjectsWithoutExpectations(): bool
    {}

    /**
     * @phpstan-assert-if-true BackupGlobals $this
     */
    public function isBackupGlobals(): bool
    {}

    /**
     * @phpstan-assert-if-true BackupStaticProperties $this
     */
    public function isBackupStaticProperties(): bool
    {}

    /**
     * @phpstan-assert-if-true BeforeClass $this
     */
    public function isBeforeClass(): bool
    {}

    /**
     * @phpstan-assert-if-true Before $this
     */
    public function isBefore(): bool
    {}

    /**
     * @phpstan-assert-if-true CoversNamespace $this
     */
    public function isCoversNamespace(): bool
    {}

    /**
     * @phpstan-assert-if-true CoversClass $this
     */
    public function isCoversClass(): bool
    {}

    /**
     * @phpstan-assert-if-true CoversClassesThatExtendClass $this
     */
    public function isCoversClassesThatExtendClass(): bool
    {}

    /**
     * @phpstan-assert-if-true CoversClassesThatImplementInterface $this
     */
    public function isCoversClassesThatImplementInterface(): bool
    {}

    /**
     * @phpstan-assert-if-true CoversTrait $this
     */
    public function isCoversTrait(): bool
    {}

    /**
     * @phpstan-assert-if-true CoversFunction $this
     */
    public function isCoversFunction(): bool
    {}

    /**
     * @phpstan-assert-if-true CoversMethod $this
     */
    public function isCoversMethod(): bool
    {}

    /**
     * @phpstan-assert-if-true CoversFile $this
     */
    public function isCoversFile(): bool
    {}

    /**
     * @phpstan-assert-if-true CoversDirectory $this
     */
    public function isCoversDirectory(): bool
    {}

    /**
     * @phpstan-assert-if-true CoversDirectoryRecursively $this
     */
    public function isCoversDirectoryRecursively(): bool
    {}

    /**
     * @phpstan-assert-if-true CoversNothing $this
     */
    public function isCoversNothing(): bool
    {}

    /**
     * @phpstan-assert-if-true DataProvider $this
     */
    public function isDataProvider(): bool
    {}

    /**
     * @phpstan-assert-if-true DataProviderClosure $this
     */
    public function isDataProviderClosure(): bool
    {}

    /**
     * @phpstan-assert-if-true DependsOnClass $this
     */
    public function isDependsOnClass(): bool
    {}

    /**
     * @phpstan-assert-if-true DependsOnMethod $this
     */
    public function isDependsOnMethod(): bool
    {}

    /**
     * @phpstan-assert-if-true DisableReturnValueGenerationForTestDoubles $this
     */
    public function isDisableReturnValueGenerationForTestDoubles(): bool
    {}

    /**
     * @phpstan-assert-if-true DoesNotPerformAssertions $this
     */
    public function isDoesNotPerformAssertions(): bool
    {}

    /**
     * @phpstan-assert-if-true ExcludeGlobalVariableFromBackup $this
     */
    public function isExcludeGlobalVariableFromBackup(): bool
    {}

    /**
     * @phpstan-assert-if-true ExcludeStaticPropertyFromBackup $this
     */
    public function isExcludeStaticPropertyFromBackup(): bool
    {}

    /**
     * @phpstan-assert-if-true Group $this
     */
    public function isGroup(): bool
    {}

    /**
     * @phpstan-assert-if-true IgnoreDeprecations $this
     */
    public function isIgnoreDeprecations(): bool
    {}

    /**
     * @phpstan-assert-if-true IgnorePhpunitDeprecations $this
     *
     * @internal This method is not covered by the backward compatibility promise for PHPUnit
     */
    public function isIgnorePhpunitDeprecations(): bool
    {}

    /**
     * @phpstan-assert-if-true InvalidAttribute $this
     */
    public function isInvalidAttribute(): bool
    {}

    /**
     * @phpstan-assert-if-true Repeat $this
     */
    public function isRepeat(): bool
    {}

    /**
     * @phpstan-assert-if-true Retry $this
     */
    public function isRetry(): bool
    {}

    /**
     * @phpstan-assert-if-true RunInSeparateProcess $this
     */
    public function isRunInSeparateProcess(): bool
    {}

    /**
     * @phpstan-assert-if-true RunTestsInSeparateProcesses $this
     */
    public function isRunTestsInSeparateProcesses(): bool
    {}

    /**
     * @phpstan-assert-if-true Test $this
     */
    public function isTest(): bool
    {}

    /**
     * @phpstan-assert-if-true PreCondition $this
     */
    public function isPreCondition(): bool
    {}

    /**
     * @phpstan-assert-if-true PostCondition $this
     */
    public function isPostCondition(): bool
    {}

    /**
     * @phpstan-assert-if-true PreserveGlobalState $this
     */
    public function isPreserveGlobalState(): bool
    {}

    /**
     * @phpstan-assert-if-true RequiresMethod $this
     */
    public function isRequiresMethod(): bool
    {}

    /**
     * @phpstan-assert-if-true RequiresClass $this
     */
    public function isRequiresClass(): bool
    {}

    /**
     * @phpstan-assert-if-true RequiresFunction $this
     */
    public function isRequiresFunction(): bool
    {}

    /**
     * @phpstan-assert-if-true RequiresOperatingSystem $this
     */
    public function isRequiresOperatingSystem(): bool
    {}

    /**
     * @phpstan-assert-if-true RequiresOperatingSystemFamily $this
     */
    public function isRequiresOperatingSystemFamily(): bool
    {}

    /**
     * @phpstan-assert-if-true RequiresPhp $this
     */
    public function isRequiresPhp(): bool
    {}

    /**
     * @phpstan-assert-if-true RequiresPhpExtension $this
     */
    public function isRequiresPhpExtension(): bool
    {}

    /**
     * @phpstan-assert-if-true RequiresPhpunit $this
     */
    public function isRequiresPhpunit(): bool
    {}

    /**
     * @phpstan-assert-if-true RequiresPhpunitExtension $this
     */
    public function isRequiresPhpunitExtension(): bool
    {}

    /**
     * @phpstan-assert-if-true RequiresEnvironmentVariable $this
     */
    public function isRequiresEnvironmentVariable(): bool
    {}

    /**
     * @phpstan-assert-if-true WithEnvironmentVariable $this
     */
    public function isWithEnvironmentVariable(): bool
    {}

    /**
     * @phpstan-assert-if-true RequiresSetting $this
     */
    public function isRequiresSetting(): bool
    {}

    /**
     * @phpstan-assert-if-true TestDox $this
     */
    public function isTestDox(): bool
    {}

    /**
     * @phpstan-assert-if-true TestDoxFormatter $this
     */
    public function isTestDoxFormatter(): bool
    {}

    /**
     * @phpstan-assert-if-true TestWith $this
     */
    public function isTestWith(): bool
    {}

    /**
     * @phpstan-assert-if-true UsesNamespace $this
     */
    public function isUsesNamespace(): bool
    {}

    /**
     * @phpstan-assert-if-true UsesClass $this
     */
    public function isUsesClass(): bool
    {}

    /**
     * @phpstan-assert-if-true UsesClassesThatExtendClass $this
     */
    public function isUsesClassesThatExtendClass(): bool
    {}

    /**
     * @phpstan-assert-if-true UsesClassesThatImplementInterface $this
     */
    public function isUsesClassesThatImplementInterface(): bool
    {}

    /**
     * @phpstan-assert-if-true UsesTrait $this
     */
    public function isUsesTrait(): bool
    {}

    /**
     * @phpstan-assert-if-true UsesFunction $this
     */
    public function isUsesFunction(): bool
    {}

    /**
     * @phpstan-assert-if-true UsesMethod $this
     */
    public function isUsesMethod(): bool
    {}

    /**
     * @phpstan-assert-if-true UsesFile $this
     */
    public function isUsesFile(): bool
    {}

    /**
     * @phpstan-assert-if-true UsesDirectory $this
     */
    public function isUsesDirectory(): bool
    {}

    /**
     * @phpstan-assert-if-true UsesDirectoryRecursively $this
     */
    public function isUsesDirectoryRecursively(): bool
    {}

    /**
     * @phpstan-assert-if-true WithoutErrorHandler $this
     */
    public function isWithoutErrorHandler(): bool
    {}

    /**
     * @phpstan-assert-if-true IgnorePhpunitWarnings $this
     */
    public function isIgnorePhpunitWarnings(): bool
    {}
}
