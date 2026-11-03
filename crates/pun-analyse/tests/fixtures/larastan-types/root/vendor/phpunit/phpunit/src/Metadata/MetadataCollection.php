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

use function array_filter;
use function array_merge;
use function count;
use Countable;
use IteratorAggregate;

/**
 * @template-implements IteratorAggregate<non-negative-int, Metadata>
 *
 * @immutable
 *
 * @no-named-arguments Parameter names are not covered by the backward compatibility promise for PHPUnit
 */
final readonly class MetadataCollection implements Countable, IteratorAggregate
{
    /**
     * @var list<Metadata>
     */
    private array $metadata;

    /**
     * @param list<Metadata> $metadata
     */
    public static function fromArray(array $metadata): self
    {}

    private function __construct(Metadata ...$metadata)
    {}

    /**
     * @return list<Metadata>
     */
    public function asArray(): array
    {}

    public function count(): int
    {}

    /**
     * @phpstan-assert-if-true 0 $this->count()
     * @phpstan-assert-if-true array{} $this->asArray()
     */
    public function isEmpty(): bool
    {}

    /**
     * @phpstan-assert-if-true positive-int $this->count()
     * @phpstan-assert-if-true non-empty-list<Metadata> $this->asArray()
     */
    public function isNotEmpty(): bool
    {}

    public function getIterator(): MetadataCollectionIterator
    {}

    public function mergeWith(self $other): self
    {}

    public function isClassLevel(): self
    {}

    public function isMethodLevel(): self
    {}

    public function isAfter(): self
    {}

    public function isAfterClass(): self
    {}

    public function isAllowMockObjectsWithoutExpectations(): self
    {}

    public function isBackupGlobals(): self
    {}

    public function isBackupStaticProperties(): self
    {}

    public function isBeforeClass(): self
    {}

    public function isBefore(): self
    {}

    public function isCoversNamespace(): self
    {}

    public function isCoversClass(): self
    {}

    public function isCoversClassesThatExtendClass(): self
    {}

    public function isCoversClassesThatImplementInterface(): self
    {}

    public function isCoversTrait(): self
    {}

    public function isCoversFunction(): self
    {}

    public function isCoversMethod(): self
    {}

    public function isCoversFile(): self
    {}

    public function isCoversDirectory(): self
    {}

    public function isCoversDirectoryRecursively(): self
    {}

    public function isExcludeGlobalVariableFromBackup(): self
    {}

    public function isExcludeStaticPropertyFromBackup(): self
    {}

    public function isCoversNothing(): self
    {}

    public function isDataProvider(): self
    {}

    public function isDataProviderClosure(): self
    {}

    public function isDepends(): self
    {}

    public function isDependsOnClass(): self
    {}

    public function isDependsOnMethod(): self
    {}

    public function isDisableReturnValueGenerationForTestDoubles(): self
    {}

    public function isDoesNotPerformAssertions(): self
    {}

    public function isGroup(): self
    {}

    public function isIgnoreDeprecations(): self
    {}

    /**
     * @internal This method is not covered by the backward compatibility promise for PHPUnit
     */
    public function isIgnorePhpunitDeprecations(): self
    {}

    public function isIgnorePhpunitWarnings(): self
    {}

    public function isInvalidAttribute(): self
    {}

    public function isRepeat(): self
    {}

    public function isRetry(): self
    {}

    public function isRunInSeparateProcess(): self
    {}

    public function isRunTestsInSeparateProcesses(): self
    {}

    public function isTest(): self
    {}

    public function isPreCondition(): self
    {}

    public function isPostCondition(): self
    {}

    public function isPreserveGlobalState(): self
    {}

    public function isRequiresMethod(): self
    {}

    public function isRequiresClass(): self
    {}

    public function isRequiresFunction(): self
    {}

    public function isRequiresOperatingSystem(): self
    {}

    public function isRequiresOperatingSystemFamily(): self
    {}

    public function isRequiresPhp(): self
    {}

    public function isRequiresPhpExtension(): self
    {}

    public function isRequiresPhpunit(): self
    {}

    public function isRequiresPhpunitExtension(): self
    {}

    public function isRequiresEnvironmentVariable(): self
    {}

    public function isWithEnvironmentVariable(): self
    {}

    public function isRequiresSetting(): self
    {}

    public function isTestDox(): self
    {}

    public function isTestDoxFormatter(): self
    {}

    public function isTestWith(): self
    {}

    public function isUsesNamespace(): self
    {}

    public function isUsesClass(): self
    {}

    public function isUsesClassesThatExtendClass(): self
    {}

    public function isUsesClassesThatImplementInterface(): self
    {}

    public function isUsesTrait(): self
    {}

    public function isUsesFunction(): self
    {}

    public function isUsesMethod(): self
    {}

    public function isUsesFile(): self
    {}

    public function isUsesDirectory(): self
    {}

    public function isUsesDirectoryRecursively(): self
    {}

    public function isWithoutErrorHandler(): self
    {}
}
