<?php declare(strict_types=1);
/*
 * This file is part of PHPUnit.
 *
 * (c) Sebastian Bergmann <sebastian@phpunit.de>
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */
namespace PHPUnit\TextUI\XmlConfiguration;

/**
 * @no-named-arguments Parameter names are not covered by the backward compatibility promise for PHPUnit
 *
 * @internal This class is not covered by the backward compatibility promise for PHPUnit
 *
 * @immutable
 */
final readonly class PHPUnit
{
    /**
     * @var ?non-empty-string
     */
    private ?string $cacheDirectory;
    private bool $recordTestRunHistory;
    private int|string $columns;

    /**
     * @var non-empty-string
     */
    private string $colors;
    private bool $stderr;
    private bool $displayDetailsOnAllIssues;
    private bool $displayDetailsOnIncompleteTests;
    private bool $displayDetailsOnSkippedTests;
    private bool $displayDetailsOnTestsThatTriggerDeprecations;
    private bool $displayDetailsOnPhpunitDeprecations;
    private bool $displayDetailsOnPhpunitNotices;
    private bool $displayDetailsOnTestsThatTriggerErrors;
    private bool $displayDetailsOnTestsThatTriggerNotices;
    private bool $displayDetailsOnTestsThatTriggerWarnings;
    private bool $reverseDefectList;
    private bool $requireCoverageMetadata;
    private bool $requireCoverageMetadataOnSmallTests;
    private bool $requireCoverageMetadataOnMediumTests;
    private bool $requireCoverageMetadataOnLargeTests;
    private bool $requireSealedMockObjects;

    /**
     * @var ?non-empty-string
     */
    private ?string $bootstrap;

    /**
     * @var array<non-empty-string, non-empty-string>
     */
    private array $bootstrapForTestSuite;
    private bool $processIsolation;
    private bool $failOnAllIssues;
    private bool $failOnDeprecation;
    private bool $hasFailOnDeprecation;
    private bool $failOnSelfDeprecation;
    private bool $hasFailOnSelfDeprecation;
    private bool $failOnDirectDeprecation;
    private bool $hasFailOnDirectDeprecation;
    private bool $failOnIndirectDeprecation;
    private bool $hasFailOnIndirectDeprecation;
    private bool $failOnPhpunitDeprecation;
    private bool $hasFailOnPhpunitDeprecation;
    private bool $failOnPhpunitNotice;
    private bool $hasFailOnPhpunitNotice;
    private bool $failOnPhpunitWarning;
    private bool $hasFailOnPhpunitWarning;
    private bool $failOnEmptyTestSuite;
    private bool $hasFailOnEmptyTestSuite;
    private bool $failOnIncomplete;
    private bool $hasFailOnIncomplete;
    private bool $failOnNotice;
    private bool $hasFailOnNotice;
    private bool $failOnRisky;
    private bool $hasFailOnRisky;
    private bool $failOnSkipped;
    private bool $hasFailOnSkipped;
    private bool $failOnWarning;
    private bool $hasFailOnWarning;

    /**
     * @var non-negative-int
     */
    private int $stopOnDefect;

    /**
     * @var non-negative-int
     */
    private int $stopOnDeprecation;

    /**
     * @var non-negative-int
     */
    private int $stopOnError;

    /**
     * @var non-negative-int
     */
    private int $stopOnFailure;

    /**
     * @var non-negative-int
     */
    private int $stopOnIncomplete;

    /**
     * @var non-negative-int
     */
    private int $stopOnNotice;

    /**
     * @var non-negative-int
     */
    private int $stopOnRisky;

    /**
     * @var non-negative-int
     */
    private int $stopOnSkipped;

    /**
     * @var non-negative-int
     */
    private int $stopOnWarning;

    /**
     * @var ?non-empty-string
     */
    private ?string $extensionsDirectory;
    private bool $beStrictAboutChangesToGlobalState;
    private bool $beStrictAboutOutputDuringTests;
    private bool $beStrictAboutTestsThatDoNotTestAnything;
    private bool $beStrictAboutCoverageMetadata;
    private bool $requireCoverageContribution;
    private bool $enforceTimeLimit;

    /**
     * @var non-negative-int
     */
    private int $defaultTimeLimit;

    /**
     * @var positive-int
     */
    private int $timeoutForSmallTests;

    /**
     * @var positive-int
     */
    private int $timeoutForMediumTests;

    /**
     * @var positive-int
     */
    private int $timeoutForLargeTests;

    /**
     * @var ?non-empty-string
     */
    private ?string $defaultTestSuite;
    private int $executionOrder;
    private bool $resolveDependencies;
    private bool $defectsFirst;
    private bool $backupGlobals;
    private bool $backupStaticProperties;
    private bool $testdoxPrinter;
    private bool $testdoxPrinterSummary;
    private bool $controlGarbageCollector;

    /**
     * @var positive-int
     */
    private int $numberOfTestsBeforeGarbageCollection;

    /**
     * @var positive-int
     */
    private int $diffContext;

    /**
     * @var non-negative-int
     */
    private int $shortenArraysForExportThreshold;
    private bool $warnWhenPhpIsNotConfiguredForDevelopment;
    private bool $cacheTestIndex;

    /**
     * @param ?non-empty-string                         $cacheDirectory
     * @param non-empty-string                          $colors
     * @param ?non-empty-string                         $bootstrap
     * @param array<non-empty-string, non-empty-string> $bootstrapForTestSuite
     * @param non-negative-int                          $stopOnDefect
     * @param non-negative-int                          $stopOnDeprecation
     * @param non-negative-int                          $stopOnError
     * @param non-negative-int                          $stopOnFailure
     * @param non-negative-int                          $stopOnIncomplete
     * @param non-negative-int                          $stopOnNotice
     * @param non-negative-int                          $stopOnRisky
     * @param non-negative-int                          $stopOnSkipped
     * @param non-negative-int                          $stopOnWarning
     * @param ?non-empty-string                         $extensionsDirectory
     * @param non-negative-int                          $defaultTimeLimit
     * @param positive-int                              $timeoutForSmallTests
     * @param positive-int                              $timeoutForMediumTests
     * @param positive-int                              $timeoutForLargeTests
     * @param ?non-empty-string                         $defaultTestSuite
     * @param positive-int                              $numberOfTestsBeforeGarbageCollection
     * @param non-negative-int                          $shortenArraysForExportThreshold
     * @param positive-int                              $diffContext
     */
    public function __construct(?string $cacheDirectory, bool $recordTestRunHistory, int|string $columns, string $colors, bool $stderr, bool $displayDetailsOnAllIssues, bool $displayDetailsOnIncompleteTests, bool $displayDetailsOnSkippedTests, bool $displayDetailsOnTestsThatTriggerDeprecations, bool $displayDetailsOnPhpunitDeprecations, bool $displayDetailsOnPhpunitNotices, bool $displayDetailsOnTestsThatTriggerErrors, bool $displayDetailsOnTestsThatTriggerNotices, bool $displayDetailsOnTestsThatTriggerWarnings, bool $reverseDefectList, bool $requireCoverageMetadata, bool $requireCoverageMetadataOnSmallTests, bool $requireCoverageMetadataOnMediumTests, bool $requireCoverageMetadataOnLargeTests, bool $requireSealedMockObjects, ?string $bootstrap, array $bootstrapForTestSuite, bool $processIsolation, bool $failOnAllIssues, bool $failOnDeprecation, bool $hasFailOnDeprecation, bool $failOnSelfDeprecation, bool $hasFailOnSelfDeprecation, bool $failOnDirectDeprecation, bool $hasFailOnDirectDeprecation, bool $failOnIndirectDeprecation, bool $hasFailOnIndirectDeprecation, bool $failOnPhpunitDeprecation, bool $hasFailOnPhpunitDeprecation, bool $failOnPhpunitNotice, bool $hasFailOnPhpunitNotice, bool $failOnPhpunitWarning, bool $hasFailOnPhpunitWarning, bool $failOnEmptyTestSuite, bool $hasFailOnEmptyTestSuite, bool $failOnIncomplete, bool $hasFailOnIncomplete, bool $failOnNotice, bool $hasFailOnNotice, bool $failOnRisky, bool $hasFailOnRisky, bool $failOnSkipped, bool $hasFailOnSkipped, bool $failOnWarning, bool $hasFailOnWarning, int $stopOnDefect, int $stopOnDeprecation, int $stopOnError, int $stopOnFailure, int $stopOnIncomplete, int $stopOnNotice, int $stopOnRisky, int $stopOnSkipped, int $stopOnWarning, ?string $extensionsDirectory, bool $beStrictAboutChangesToGlobalState, bool $beStrictAboutOutputDuringTests, bool $beStrictAboutTestsThatDoNotTestAnything, bool $beStrictAboutCoverageMetadata, bool $requireCoverageContribution, bool $enforceTimeLimit, int $defaultTimeLimit, int $timeoutForSmallTests, int $timeoutForMediumTests, int $timeoutForLargeTests, ?string $defaultTestSuite, int $executionOrder, bool $resolveDependencies, bool $defectsFirst, bool $backupGlobals, bool $backupStaticProperties, bool $testdoxPrinter, bool $testdoxPrinterSummary, bool $controlGarbageCollector, int $numberOfTestsBeforeGarbageCollection, int $shortenArraysForExportThreshold, int $diffContext, bool $warnWhenPhpIsNotConfiguredForDevelopment, bool $cacheTestIndex)
    {}

    /**
     * @phpstan-assert-if-true !null $this->cacheDirectory
     */
    public function hasCacheDirectory(): bool
    {}

    /**
     * @throws Exception
     *
     * @return non-empty-string
     */
    public function cacheDirectory(): string
    {}

    public function recordTestRunHistory(): bool
    {}

    public function cacheTestIndex(): bool
    {}

    public function columns(): int|string
    {}

    /**
     * @return non-empty-string
     */
    public function colors(): string
    {}

    public function stderr(): bool
    {}

    public function displayDetailsOnAllIssues(): bool
    {}

    public function displayDetailsOnIncompleteTests(): bool
    {}

    public function displayDetailsOnSkippedTests(): bool
    {}

    public function displayDetailsOnTestsThatTriggerDeprecations(): bool
    {}

    public function displayDetailsOnPhpunitDeprecations(): bool
    {}

    public function displayDetailsOnPhpunitNotices(): bool
    {}

    public function displayDetailsOnTestsThatTriggerErrors(): bool
    {}

    public function displayDetailsOnTestsThatTriggerNotices(): bool
    {}

    public function displayDetailsOnTestsThatTriggerWarnings(): bool
    {}

    public function reverseDefectList(): bool
    {}

    public function requireCoverageMetadata(): bool
    {}

    public function requireCoverageMetadataOnSmallTests(): bool
    {}

    public function requireCoverageMetadataOnMediumTests(): bool
    {}

    public function requireCoverageMetadataOnLargeTests(): bool
    {}

    public function requireSealedMockObjects(): bool
    {}

    /**
     * @phpstan-assert-if-true !null $this->bootstrap
     */
    public function hasBootstrap(): bool
    {}

    /**
     * @throws Exception
     *
     * @return non-empty-string
     */
    public function bootstrap(): string
    {}

    /**
     * @return array<non-empty-string, non-empty-string>
     */
    public function bootstrapForTestSuite(): array
    {}

    public function processIsolation(): bool
    {}

    public function failOnAllIssues(): bool
    {}

    public function failOnDeprecation(): bool
    {}

    public function hasFailOnDeprecation(): bool
    {}

    public function failOnSelfDeprecation(): bool
    {}

    public function hasFailOnSelfDeprecation(): bool
    {}

    public function failOnDirectDeprecation(): bool
    {}

    public function hasFailOnDirectDeprecation(): bool
    {}

    public function failOnIndirectDeprecation(): bool
    {}

    public function hasFailOnIndirectDeprecation(): bool
    {}

    public function failOnPhpunitDeprecation(): bool
    {}

    public function hasFailOnPhpunitDeprecation(): bool
    {}

    public function failOnPhpunitNotice(): bool
    {}

    public function hasFailOnPhpunitNotice(): bool
    {}

    public function failOnPhpunitWarning(): bool
    {}

    public function hasFailOnPhpunitWarning(): bool
    {}

    public function failOnEmptyTestSuite(): bool
    {}

    public function hasFailOnEmptyTestSuite(): bool
    {}

    public function failOnIncomplete(): bool
    {}

    public function hasFailOnIncomplete(): bool
    {}

    public function failOnNotice(): bool
    {}

    public function hasFailOnNotice(): bool
    {}

    public function failOnRisky(): bool
    {}

    public function hasFailOnRisky(): bool
    {}

    public function failOnSkipped(): bool
    {}

    public function hasFailOnSkipped(): bool
    {}

    public function failOnWarning(): bool
    {}

    public function hasFailOnWarning(): bool
    {}

    /**
     * @return non-negative-int
     */
    public function stopOnDefect(): int
    {}

    /**
     * @return non-negative-int
     */
    public function stopOnDeprecation(): int
    {}

    /**
     * @return non-negative-int
     */
    public function stopOnError(): int
    {}

    /**
     * @return non-negative-int
     */
    public function stopOnFailure(): int
    {}

    /**
     * @return non-negative-int
     */
    public function stopOnIncomplete(): int
    {}

    /**
     * @return non-negative-int
     */
    public function stopOnNotice(): int
    {}

    /**
     * @return non-negative-int
     */
    public function stopOnRisky(): int
    {}

    /**
     * @return non-negative-int
     */
    public function stopOnSkipped(): int
    {}

    /**
     * @return non-negative-int
     */
    public function stopOnWarning(): int
    {}

    /**
     * @phpstan-assert-if-true !null $this->extensionsDirectory
     */
    public function hasExtensionsDirectory(): bool
    {}

    /**
     * @throws Exception
     *
     * @return non-empty-string
     */
    public function extensionsDirectory(): string
    {}

    public function beStrictAboutChangesToGlobalState(): bool
    {}

    public function beStrictAboutOutputDuringTests(): bool
    {}

    public function beStrictAboutTestsThatDoNotTestAnything(): bool
    {}

    public function beStrictAboutCoverageMetadata(): bool
    {}

    public function requireCoverageContribution(): bool
    {}

    public function enforceTimeLimit(): bool
    {}

    /**
     * @return non-negative-int
     */
    public function defaultTimeLimit(): int
    {}

    /**
     * @return positive-int
     */
    public function timeoutForSmallTests(): int
    {}

    /**
     * @return positive-int
     */
    public function timeoutForMediumTests(): int
    {}

    /**
     * @return positive-int
     */
    public function timeoutForLargeTests(): int
    {}

    /**
     * @phpstan-assert-if-true !null $this->defaultTestSuite
     */
    public function hasDefaultTestSuite(): bool
    {}

    /**
     * @throws Exception
     *
     * @return non-empty-string
     */
    public function defaultTestSuite(): string
    {}

    public function executionOrder(): int
    {}

    public function resolveDependencies(): bool
    {}

    public function defectsFirst(): bool
    {}

    public function backupGlobals(): bool
    {}

    public function backupStaticProperties(): bool
    {}

    public function testdoxPrinter(): bool
    {}

    public function testdoxPrinterSummary(): bool
    {}

    public function controlGarbageCollector(): bool
    {}

    /**
     * @return positive-int
     */
    public function numberOfTestsBeforeGarbageCollection(): int
    {}

    /**
     * @return non-negative-int
     */
    public function shortenArraysForExportThreshold(): int
    {}

    /**
     * @return positive-int
     */
    public function diffContext(): int
    {}

    public function warnWhenPhpIsNotConfiguredForDevelopment(): bool
    {}
}
