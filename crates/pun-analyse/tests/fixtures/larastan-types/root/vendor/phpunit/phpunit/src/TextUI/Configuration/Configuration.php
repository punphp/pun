<?php declare(strict_types=1);
/*
 * This file is part of PHPUnit.
 *
 * (c) Sebastian Bergmann <sebastian@phpunit.de>
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */
namespace PHPUnit\TextUI\Configuration;

use function array_filter;
use function array_values;
use function explode;

/**
 * @immutable
 *
 * @no-named-arguments Parameter names are not covered by the backward compatibility promise for PHPUnit
 */
final readonly class Configuration
{
    public const string COLOR_NEVER   = 'never';
    public const string COLOR_AUTO    = 'auto';
    public const string COLOR_ALWAYS  = 'always';
    public const string COLOR_DEFAULT = self::COLOR_NEVER;

    /**
     * @var list<non-empty-string>
     */
    private array $cliArguments;

    /**
     * @var ?non-empty-string
     */
    private ?string $testFilesFile;

    /**
     * @var ?non-empty-string
     */
    private ?string $configurationFile;

    /**
     * @var ?non-empty-string
     */
    private ?string $bootstrap;

    /**
     * @var array<non-empty-string, non-empty-string>
     */
    private array $bootstrapForTestSuite;
    private bool $recordTestRunHistory;

    /**
     * @var ?non-empty-string
     */
    private ?string $cacheDirectory;

    /**
     * @var ?non-empty-string
     */
    private ?string $coverageCacheDirectory;
    private Source $source;
    private bool $pathCoverage;
    private bool $branchCoverage;
    private ?string $coverageDriver;

    /**
     * @var ?non-empty-string
     */
    private ?string $coverageClover;

    /**
     * @var ?non-empty-string
     */
    private ?string $coverageCobertura;

    /**
     * @var ?non-empty-string
     */
    private ?string $coverageCrap4j;

    /**
     * @var non-negative-int
     */
    private int $coverageCrap4jThreshold;

    /**
     * @var ?non-empty-string
     */
    private ?string $coverageHtml;
    private bool $coverageHtmlClassView;
    private bool $coverageHtmlFileView;

    /**
     * @var non-negative-int
     */
    private int $coverageHtmlLowUpperBound;

    /**
     * @var non-negative-int
     */
    private int $coverageHtmlHighLowerBound;

    /**
     * @var non-empty-string
     */
    private string $coverageHtmlColorSuccessLow;

    /**
     * @var non-empty-string
     */
    private string $coverageHtmlColorSuccessLowDark;

    /**
     * @var non-empty-string
     */
    private string $coverageHtmlColorSuccessMedium;

    /**
     * @var non-empty-string
     */
    private string $coverageHtmlColorSuccessMediumDark;

    /**
     * @var non-empty-string
     */
    private string $coverageHtmlColorSuccessHigh;

    /**
     * @var non-empty-string
     */
    private string $coverageHtmlColorSuccessHighDark;

    /**
     * @var non-empty-string
     */
    private string $coverageHtmlColorSuccessBar;

    /**
     * @var non-empty-string
     */
    private string $coverageHtmlColorSuccessBarDark;

    /**
     * @var non-empty-string
     */
    private string $coverageHtmlColorWarning;

    /**
     * @var non-empty-string
     */
    private string $coverageHtmlColorWarningDark;

    /**
     * @var non-empty-string
     */
    private string $coverageHtmlColorWarningBar;

    /**
     * @var non-empty-string
     */
    private string $coverageHtmlColorWarningBarDark;

    /**
     * @var non-empty-string
     */
    private string $coverageHtmlColorDanger;

    /**
     * @var non-empty-string
     */
    private string $coverageHtmlColorDangerDark;

    /**
     * @var non-empty-string
     */
    private string $coverageHtmlColorDangerBar;

    /**
     * @var non-empty-string
     */
    private string $coverageHtmlColorDangerBarDark;

    /**
     * @var non-empty-string
     */
    private string $coverageHtmlColorBreadcrumbs;

    /**
     * @var non-empty-string
     */
    private string $coverageHtmlColorBreadcrumbsDark;

    /**
     * @var ?non-empty-string
     */
    private ?string $coverageHtmlCustomCssFile;

    /**
     * @var ?non-empty-string
     */
    private ?string $coverageJsonl;

    /**
     * @var ?non-empty-string
     */
    private ?string $coverageOpenClover;

    /**
     * @var ?non-empty-string
     */
    private ?string $coveragePhp;

    /**
     * @var ?non-empty-string
     */
    private ?string $coverageText;
    private bool $coverageTextShowUncoveredFiles;
    private bool $coverageTextShowOnlySummary;

    /**
     * @var ?non-empty-string
     */
    private ?string $coverageXml;
    private bool $coverageXmlIncludeSource;

    /**
     * @var non-empty-string
     */
    private string $testRunHistoryFile;
    private bool $ignoreDeprecatedCodeUnitsFromCodeCoverage;
    private bool $disableCodeCoverageIgnore;
    private bool $disableCoverageTargeting;
    private bool $failOnAllIssues;
    private bool $failOnDeprecation;
    private bool $failOnSelfDeprecation;
    private bool $failOnDirectDeprecation;
    private bool $failOnIndirectDeprecation;
    private bool $failOnPhpunitDeprecation;
    private bool $failOnPhpunitNotice;
    private bool $failOnPhpunitWarning;
    private bool $failOnEmptyTestSuite;
    private bool $failOnIncomplete;
    private bool $failOnNotice;
    private bool $failOnRisky;
    private bool $failOnSkipped;
    private bool $failOnWarning;
    private bool $doNotFailOnDeprecation;
    private bool $doNotFailOnSelfDeprecation;
    private bool $doNotFailOnDirectDeprecation;
    private bool $doNotFailOnIndirectDeprecation;
    private bool $doNotFailOnPhpunitDeprecation;
    private bool $doNotFailOnPhpunitNotice;
    private bool $doNotFailOnPhpunitWarning;
    private bool $doNotFailOnEmptyTestSuite;
    private bool $doNotFailOnIncomplete;
    private bool $doNotFailOnNotice;
    private bool $doNotFailOnRisky;
    private bool $doNotFailOnSkipped;
    private bool $doNotFailOnWarning;

    /**
     * @var non-negative-int
     */
    private int $stopOnDefect;

    /**
     * @var non-negative-int
     */
    private int $stopOnDeprecation;

    /**
     * @var ?non-empty-string
     */
    private ?string $specificDeprecationToStopOn;

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
    private bool $outputToStandardErrorStream;

    /**
     * @var positive-int
     */
    private int $columns;
    private bool $noExtensions;

    /**
     * @var ?non-empty-string
     */
    private ?string $pharExtensionDirectory;

    /**
     * @var list<array{className: non-empty-string, parameters: array<string, string>}>
     */
    private array $extensionBootstrappers;
    private bool $backupGlobals;
    private bool $backupStaticProperties;
    private bool $beStrictAboutChangesToGlobalState;
    private bool $colors;
    private bool $processIsolation;
    private bool $enforceTimeLimit;

    /**
     * @var non-negative-int
     */
    private int $defaultTimeLimit;

    /**
     * @var positive-int
     */
    private int $diffContext;

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
    private bool $reportUselessTests;
    private bool $strictCoverage;
    private bool $requireCoverageContribution;
    private bool $disallowTestOutput;
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
    private bool $noProgress;
    private bool $noResults;
    private bool $noOutput;
    private int $executionOrder;
    private int $executionOrderDefects;
    private bool $resolveDependencies;
    private ?string $logfileTeamcity;
    private ?string $logfileJunit;

    /**
     * @var ?non-empty-string
     */
    private ?string $logfileOtr;
    private bool $includeGitInformation;
    private bool $includeGitInformationInOtrLogfile;
    private ?string $logfileTestdoxHtml;
    private ?string $logfileTestdoxText;

    /**
     * @var ?non-empty-string
     */
    private ?string $logEventsText;

    /**
     * @var ?non-empty-string
     */
    private ?string $logEventsVerboseText;

    /**
     * @var ?non-empty-list<non-empty-string>
     */
    private ?array $testsCovering;

    /**
     * @var ?non-empty-list<non-empty-string>
     */
    private ?array $testsUsing;

    /**
     * @var ?non-empty-list<non-empty-string>
     */
    private ?array $testsRequiringPhpExtension;
    private bool $compactOutput;
    private bool $teamCityOutput;
    private bool $testDoxOutput;
    private bool $testDoxOutputSummary;
    private ?string $filter;
    private ?string $excludeFilter;

    /**
     * @var ?non-empty-string
     */
    private ?string $testIdFilterFile;

    /**
     * @var ?non-empty-string
     */
    private ?string $testIdFilter;

    /**
     * @var list<non-empty-string>
     */
    private array $groups;

    /**
     * @var list<non-empty-string>
     */
    private array $excludeGroups;

    /**
     * @var positive-int
     */
    private int $randomOrderSeed;

    /**
     * @var positive-int
     */
    private int $repeat;

    /**
     * @var positive-int
     */
    private int $retry;

    /**
     * @var non-negative-int
     */
    private int $timeout;
    private bool $includeUncoveredFiles;
    private TestSuiteCollection $testSuite;
    private string $includeTestSuite;
    private string $excludeTestSuite;

    /**
     * @var ?non-empty-string
     */
    private ?string $defaultTestSuite;
    private bool $ignoreTestSelectionInXmlConfiguration;

    /**
     * @var non-empty-list<non-empty-string>
     */
    private array $testSuffixes;
    private Php $php;
    private bool $controlGarbageCollector;

    /**
     * @var positive-int
     */
    private int $numberOfTestsBeforeGarbageCollection;

    /**
     * @var null|non-empty-string
     */
    private ?string $generateBaseline;
    private bool $debug;
    private bool $withTelemetry;

    /**
     * @var non-negative-int
     */
    private int $shortenArraysForExportThreshold;
    private bool $warnWhenPhpIsNotConfiguredForDevelopment;
    private bool $cacheTestIndex;

    /**
     * @param list<non-empty-string>                                                      $cliArguments
     * @param ?non-empty-string                                                           $testFilesFile
     * @param ?non-empty-string                                                           $configurationFile
     * @param ?non-empty-string                                                           $bootstrap
     * @param array<non-empty-string, non-empty-string>                                   $bootstrapForTestSuite
     * @param ?non-empty-string                                                           $cacheDirectory
     * @param ?non-empty-string                                                           $coverageCacheDirectory
     * @param non-empty-string                                                            $testRunHistoryFile
     * @param ?non-empty-string                                                           $coverageClover
     * @param ?non-empty-string                                                           $coverageCobertura
     * @param ?non-empty-string                                                           $coverageCrap4j
     * @param non-negative-int                                                            $coverageCrap4jThreshold
     * @param ?non-empty-string                                                           $coverageHtml
     * @param non-negative-int                                                            $coverageHtmlLowUpperBound
     * @param non-negative-int                                                            $coverageHtmlHighLowerBound
     * @param non-empty-string                                                            $coverageHtmlColorSuccessLow
     * @param non-empty-string                                                            $coverageHtmlColorSuccessLowDark
     * @param non-empty-string                                                            $coverageHtmlColorSuccessMedium
     * @param non-empty-string                                                            $coverageHtmlColorSuccessMediumDark
     * @param non-empty-string                                                            $coverageHtmlColorSuccessHigh
     * @param non-empty-string                                                            $coverageHtmlColorSuccessHighDark
     * @param non-empty-string                                                            $coverageHtmlColorSuccessBar
     * @param non-empty-string                                                            $coverageHtmlColorSuccessBarDark
     * @param non-empty-string                                                            $coverageHtmlColorWarning
     * @param non-empty-string                                                            $coverageHtmlColorWarningDark
     * @param non-empty-string                                                            $coverageHtmlColorWarningBar
     * @param non-empty-string                                                            $coverageHtmlColorWarningBarDark
     * @param non-empty-string                                                            $coverageHtmlColorDanger
     * @param non-empty-string                                                            $coverageHtmlColorDangerDark
     * @param non-empty-string                                                            $coverageHtmlColorDangerBar
     * @param non-empty-string                                                            $coverageHtmlColorDangerBarDark
     * @param non-empty-string                                                            $coverageHtmlColorBreadcrumbs
     * @param non-empty-string                                                            $coverageHtmlColorBreadcrumbsDark
     * @param ?non-empty-string                                                           $coverageHtmlCustomCssFile
     * @param ?non-empty-string                                                           $coverageJsonl
     * @param ?non-empty-string                                                           $coverageOpenClover
     * @param ?non-empty-string                                                           $coveragePhp
     * @param ?non-empty-string                                                           $coverageText
     * @param ?non-empty-string                                                           $coverageXml
     * @param non-negative-int                                                            $stopOnDefect
     * @param non-negative-int                                                            $stopOnDeprecation
     * @param ?non-empty-string                                                           $specificDeprecationToStopOn
     * @param non-negative-int                                                            $stopOnError
     * @param non-negative-int                                                            $stopOnFailure
     * @param non-negative-int                                                            $stopOnIncomplete
     * @param non-negative-int                                                            $stopOnNotice
     * @param non-negative-int                                                            $stopOnRisky
     * @param non-negative-int                                                            $stopOnSkipped
     * @param non-negative-int                                                            $stopOnWarning
     * @param positive-int                                                                $columns
     * @param ?non-empty-string                                                           $pharExtensionDirectory
     * @param list<array{className: non-empty-string, parameters: array<string, string>}> $extensionBootstrappers
     * @param non-negative-int                                                            $defaultTimeLimit
     * @param positive-int                                                                $diffContext
     * @param positive-int                                                                $timeoutForSmallTests
     * @param positive-int                                                                $timeoutForMediumTests
     * @param positive-int                                                                $timeoutForLargeTests
     * @param ?non-empty-string                                                           $logfileOtr
     * @param ?non-empty-string                                                           $logEventsText
     * @param ?non-empty-string                                                           $logEventsVerboseText
     * @param ?non-empty-list<non-empty-string>                                           $testsCovering
     * @param ?non-empty-list<non-empty-string>                                           $testsUsing
     * @param ?non-empty-list<non-empty-string>                                           $testsRequiringPhpExtension
     * @param ?non-empty-string                                                           $testIdFilterFile
     * @param ?non-empty-string                                                           $testIdFilter
     * @param list<non-empty-string>                                                      $groups
     * @param list<non-empty-string>                                                      $excludeGroups
     * @param positive-int                                                                $randomOrderSeed
     * @param positive-int                                                                $repeat
     * @param positive-int                                                                $retry
     * @param non-negative-int                                                            $timeout
     * @param ?non-empty-string                                                           $defaultTestSuite
     * @param non-empty-list<non-empty-string>                                            $testSuffixes
     * @param positive-int                                                                $numberOfTestsBeforeGarbageCollection
     * @param null|non-empty-string                                                       $generateBaseline
     * @param non-negative-int                                                            $shortenArraysForExportThreshold
     */
    public function __construct(array $cliArguments, ?string $testFilesFile, ?string $configurationFile, ?string $bootstrap, array $bootstrapForTestSuite, bool $recordTestRunHistory, ?string $cacheDirectory, ?string $coverageCacheDirectory, Source $source, string $testRunHistoryFile, ?string $coverageClover, ?string $coverageCobertura, ?string $coverageCrap4j, int $coverageCrap4jThreshold, ?string $coverageHtml, bool $coverageHtmlClassView, bool $coverageHtmlFileView, int $coverageHtmlLowUpperBound, int $coverageHtmlHighLowerBound, string $coverageHtmlColorSuccessLow, string $coverageHtmlColorSuccessLowDark, string $coverageHtmlColorSuccessMedium, string $coverageHtmlColorSuccessMediumDark, string $coverageHtmlColorSuccessHigh, string $coverageHtmlColorSuccessHighDark, string $coverageHtmlColorSuccessBar, string $coverageHtmlColorSuccessBarDark, string $coverageHtmlColorWarning, string $coverageHtmlColorWarningDark, string $coverageHtmlColorWarningBar, string $coverageHtmlColorWarningBarDark, string $coverageHtmlColorDanger, string $coverageHtmlColorDangerDark, string $coverageHtmlColorDangerBar, string $coverageHtmlColorDangerBarDark, string $coverageHtmlColorBreadcrumbs, string $coverageHtmlColorBreadcrumbsDark, ?string $coverageHtmlCustomCssFile, ?string $coverageJsonl, ?string $coverageOpenClover, ?string $coveragePhp, ?string $coverageText, bool $coverageTextShowUncoveredFiles, bool $coverageTextShowOnlySummary, ?string $coverageXml, bool $coverageXmlIncludeSource, bool $pathCoverage, bool $branchCoverage, ?string $coverageDriver, bool $ignoreDeprecatedCodeUnitsFromCodeCoverage, bool $disableCodeCoverageIgnore, bool $disableCoverageTargeting, bool $failOnAllIssues, bool $failOnDeprecation, bool $failOnSelfDeprecation, bool $failOnDirectDeprecation, bool $failOnIndirectDeprecation, bool $failOnPhpunitDeprecation, bool $failOnPhpunitNotice, bool $failOnPhpunitWarning, bool $failOnEmptyTestSuite, bool $failOnIncomplete, bool $failOnNotice, bool $failOnRisky, bool $failOnSkipped, bool $failOnWarning, bool $doNotFailOnDeprecation, bool $doNotFailOnSelfDeprecation, bool $doNotFailOnDirectDeprecation, bool $doNotFailOnIndirectDeprecation, bool $doNotFailOnPhpunitDeprecation, bool $doNotFailOnPhpunitNotice, bool $doNotFailOnPhpunitWarning, bool $doNotFailOnEmptyTestSuite, bool $doNotFailOnIncomplete, bool $doNotFailOnNotice, bool $doNotFailOnRisky, bool $doNotFailOnSkipped, bool $doNotFailOnWarning, int $stopOnDefect, int $stopOnDeprecation, ?string $specificDeprecationToStopOn, int $stopOnError, int $stopOnFailure, int $stopOnIncomplete, int $stopOnNotice, int $stopOnRisky, int $stopOnSkipped, int $stopOnWarning, bool $outputToStandardErrorStream, int $columns, bool $noExtensions, ?string $pharExtensionDirectory, array $extensionBootstrappers, bool $backupGlobals, bool $backupStaticProperties, bool $beStrictAboutChangesToGlobalState, bool $colors, bool $processIsolation, bool $enforceTimeLimit, int $defaultTimeLimit, int $diffContext, int $timeoutForSmallTests, int $timeoutForMediumTests, int $timeoutForLargeTests, bool $reportUselessTests, bool $strictCoverage, bool $requireCoverageContribution, bool $disallowTestOutput, bool $displayDetailsOnAllIssues, bool $displayDetailsOnIncompleteTests, bool $displayDetailsOnSkippedTests, bool $displayDetailsOnTestsThatTriggerDeprecations, bool $displayDetailsOnPhpunitDeprecations, bool $displayDetailsOnPhpunitNotices, bool $displayDetailsOnTestsThatTriggerErrors, bool $displayDetailsOnTestsThatTriggerNotices, bool $displayDetailsOnTestsThatTriggerWarnings, bool $reverseDefectList, bool $requireCoverageMetadata, bool $requireCoverageMetadataOnSmallTests, bool $requireCoverageMetadataOnMediumTests, bool $requireCoverageMetadataOnLargeTests, bool $requireSealedMockObjects, bool $noProgress, bool $noResults, bool $noOutput, int $executionOrder, int $executionOrderDefects, bool $resolveDependencies, ?string $logfileTeamcity, ?string $logfileJunit, ?string $logfileOtr, bool $includeGitInformation, bool $includeGitInformationInOtrLogfile, ?string $logfileTestdoxHtml, ?string $logfileTestdoxText, ?string $logEventsText, ?string $logEventsVerboseText, bool $compactOutput, bool $teamCityOutput, bool $testDoxOutput, bool $testDoxOutputSummary, ?array $testsCovering, ?array $testsUsing, ?array $testsRequiringPhpExtension, ?string $filter, ?string $excludeFilter, ?string $testIdFilterFile, ?string $testIdFilter, array $groups, array $excludeGroups, int $randomOrderSeed, int $repeat, int $retry, int $timeout, bool $includeUncoveredFiles, TestSuiteCollection $testSuite, string $includeTestSuite, string $excludeTestSuite, ?string $defaultTestSuite, bool $ignoreTestSelectionInXmlConfiguration, array $testSuffixes, Php $php, bool $controlGarbageCollector, int $numberOfTestsBeforeGarbageCollection, ?string $generateBaseline, bool $debug, bool $withTelemetry, int $shortenArraysForExportThreshold, bool $warnWhenPhpIsNotConfiguredForDevelopment, bool $cacheTestIndex)
    {}

    /**
     * @phpstan-assert-if-true !empty $this->cliArguments
     */
    public function hasCliArguments(): bool
    {}

    /**
     * @return list<non-empty-string>
     */
    public function cliArguments(): array
    {}

    /**
     * @phpstan-assert-if-true !null $this->testFilesFile
     */
    public function hasTestFilesFile(): bool
    {}

    /**
     * @throws NoTestFilesFileException
     *
     * @return non-empty-string
     */
    public function testFilesFile(): string
    {}

    /**
     * @phpstan-assert-if-true !null $this->configurationFile
     */
    public function hasConfigurationFile(): bool
    {}

    /**
     * @throws NoConfigurationFileException
     *
     * @return non-empty-string
     */
    public function configurationFile(): string
    {}

    /**
     * @phpstan-assert-if-true !null $this->bootstrap
     */
    public function hasBootstrap(): bool
    {}

    /**
     * @throws NoBootstrapException
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

    public function recordTestRunHistory(): bool
    {}

    /**
     * @deprecated Use recordTestRunHistory() instead
     *
     * @codeCoverageIgnore
     */
    public function cacheResult(): bool
    {}

    /**
     * Whether PHPUnit maintains an index of the tests in a test file so that it
     * does not have to load a file that cannot contribute a test to the run.
     */
    public function cacheTestIndex(): bool
    {}

    /**
     * @phpstan-assert-if-true !null $this->cacheDirectory
     */
    public function hasCacheDirectory(): bool
    {}

    /**
     * @throws NoCacheDirectoryException
     *
     * @return non-empty-string
     */
    public function cacheDirectory(): string
    {}

    /**
     * @phpstan-assert-if-true !null $this->coverageCacheDirectory
     */
    public function hasCoverageCacheDirectory(): bool
    {}

    /**
     * @throws NoCoverageCacheDirectoryException
     *
     * @return non-empty-string
     */
    public function coverageCacheDirectory(): string
    {}

    public function source(): Source
    {}

    /**
     * @return non-empty-string
     */
    public function testRunHistoryFile(): string
    {}

    /**
     * @deprecated Use testRunHistoryFile() instead
     *
     * @codeCoverageIgnore
     *
     * @return non-empty-string
     */
    public function testResultCacheFile(): string
    {}

    public function ignoreDeprecatedCodeUnitsFromCodeCoverage(): bool
    {}

    public function disableCodeCoverageIgnore(): bool
    {}

    public function disableCoverageTargeting(): bool
    {}

    public function pathCoverage(): bool
    {}

    public function branchCoverage(): bool
    {}

    /**
     * @phpstan-assert-if-true !null $this->coverageDriver
     */
    public function hasCoverageDriver(): bool
    {}

    /**
     * @throws CodeCoverageDriverNotConfiguredException
     */
    public function coverageDriver(): string
    {}

    public function hasCoverageReport(): bool
    {}

    /**
     * @phpstan-assert-if-true !null $this->coverageClover
     */
    public function hasCoverageClover(): bool
    {}

    /**
     * @throws CodeCoverageReportNotConfiguredException
     *
     * @return non-empty-string
     */
    public function coverageClover(): string
    {}

    /**
     * @phpstan-assert-if-true !null $this->coverageCobertura
     */
    public function hasCoverageCobertura(): bool
    {}

    /**
     * @throws CodeCoverageReportNotConfiguredException
     *
     * @return non-empty-string
     */
    public function coverageCobertura(): string
    {}

    /**
     * @phpstan-assert-if-true !null $this->coverageCrap4j
     */
    public function hasCoverageCrap4j(): bool
    {}

    /**
     * @throws CodeCoverageReportNotConfiguredException
     *
     * @return non-empty-string
     */
    public function coverageCrap4j(): string
    {}

    /**
     * @return non-negative-int
     */
    public function coverageCrap4jThreshold(): int
    {}

    /**
     * @phpstan-assert-if-true !null $this->coverageHtml
     */
    public function hasCoverageHtml(): bool
    {}

    /**
     * @throws CodeCoverageReportNotConfiguredException
     *
     * @return non-empty-string
     */
    public function coverageHtml(): string
    {}

    public function coverageHtmlClassView(): bool
    {}

    public function coverageHtmlFileView(): bool
    {}

    /**
     * @return non-negative-int
     */
    public function coverageHtmlLowUpperBound(): int
    {}

    /**
     * @return non-negative-int
     */
    public function coverageHtmlHighLowerBound(): int
    {}

    /**
     * @return non-empty-string
     */
    public function coverageHtmlColorSuccessLow(): string
    {}

    /**
     * @return non-empty-string
     */
    public function coverageHtmlColorSuccessLowDark(): string
    {}

    /**
     * @return non-empty-string
     */
    public function coverageHtmlColorSuccessMedium(): string
    {}

    /**
     * @return non-empty-string
     */
    public function coverageHtmlColorSuccessMediumDark(): string
    {}

    /**
     * @return non-empty-string
     */
    public function coverageHtmlColorSuccessHigh(): string
    {}

    /**
     * @return non-empty-string
     */
    public function coverageHtmlColorSuccessHighDark(): string
    {}

    /**
     * @return non-empty-string
     */
    public function coverageHtmlColorSuccessBar(): string
    {}

    /**
     * @return non-empty-string
     */
    public function coverageHtmlColorSuccessBarDark(): string
    {}

    /**
     * @return non-empty-string
     */
    public function coverageHtmlColorWarning(): string
    {}

    /**
     * @return non-empty-string
     */
    public function coverageHtmlColorWarningDark(): string
    {}

    /**
     * @return non-empty-string
     */
    public function coverageHtmlColorWarningBar(): string
    {}

    /**
     * @return non-empty-string
     */
    public function coverageHtmlColorWarningBarDark(): string
    {}

    /**
     * @return non-empty-string
     */
    public function coverageHtmlColorDanger(): string
    {}

    /**
     * @return non-empty-string
     */
    public function coverageHtmlColorDangerDark(): string
    {}

    /**
     * @return non-empty-string
     */
    public function coverageHtmlColorDangerBar(): string
    {}

    /**
     * @return non-empty-string
     */
    public function coverageHtmlColorDangerBarDark(): string
    {}

    /**
     * @return non-empty-string
     */
    public function coverageHtmlColorBreadcrumbs(): string
    {}

    /**
     * @return non-empty-string
     */
    public function coverageHtmlColorBreadcrumbsDark(): string
    {}

    /**
     * @phpstan-assert-if-true !null $this->coverageHtmlCustomCssFile
     */
    public function hasCoverageHtmlCustomCssFile(): bool
    {}

    /**
     * @throws NoCustomCssFileException
     *
     * @return non-empty-string
     */
    public function coverageHtmlCustomCssFile(): string
    {}

    /**
     * @phpstan-assert-if-true !null $this->coverageJsonl
     */
    public function hasCoverageJsonl(): bool
    {}

    /**
     * @throws CodeCoverageReportNotConfiguredException
     *
     * @return non-empty-string
     */
    public function coverageJsonl(): string
    {}

    /**
     * @phpstan-assert-if-true !null $this->coverageOpenClover
     */
    public function hasCoverageOpenClover(): bool
    {}

    /**
     * @throws CodeCoverageReportNotConfiguredException
     *
     * @return non-empty-string
     */
    public function coverageOpenClover(): string
    {}

    /**
     * @phpstan-assert-if-true !null $this->coveragePhp
     */
    public function hasCoveragePhp(): bool
    {}

    /**
     * @throws CodeCoverageReportNotConfiguredException
     *
     * @return non-empty-string
     */
    public function coveragePhp(): string
    {}

    /**
     * @phpstan-assert-if-true !null $this->coverageText
     */
    public function hasCoverageText(): bool
    {}

    /**
     * @throws CodeCoverageReportNotConfiguredException
     *
     * @return non-empty-string
     */
    public function coverageText(): string
    {}

    public function coverageTextShowUncoveredFiles(): bool
    {}

    public function coverageTextShowOnlySummary(): bool
    {}

    /**
     * @phpstan-assert-if-true !null $this->coverageXml
     */
    public function hasCoverageXml(): bool
    {}

    /**
     * @throws CodeCoverageReportNotConfiguredException
     *
     * @return non-empty-string
     */
    public function coverageXml(): string
    {}

    public function coverageXmlIncludeSource(): bool
    {}

    public function failOnAllIssues(): bool
    {}

    public function failOnDeprecation(): bool
    {}

    public function failOnSelfDeprecation(): bool
    {}

    public function failOnDirectDeprecation(): bool
    {}

    public function failOnIndirectDeprecation(): bool
    {}

    public function failOnPhpunitDeprecation(): bool
    {}

    public function failOnPhpunitNotice(): bool
    {}

    public function failOnPhpunitWarning(): bool
    {}

    public function failOnEmptyTestSuite(): bool
    {}

    public function failOnIncomplete(): bool
    {}

    public function failOnNotice(): bool
    {}

    public function failOnRisky(): bool
    {}

    public function failOnSkipped(): bool
    {}

    public function failOnWarning(): bool
    {}

    public function doNotFailOnDeprecation(): bool
    {}

    public function doNotFailOnSelfDeprecation(): bool
    {}

    public function doNotFailOnDirectDeprecation(): bool
    {}

    public function doNotFailOnIndirectDeprecation(): bool
    {}

    public function doNotFailOnPhpunitDeprecation(): bool
    {}

    public function doNotFailOnPhpunitNotice(): bool
    {}

    public function doNotFailOnPhpunitWarning(): bool
    {}

    public function doNotFailOnEmptyTestSuite(): bool
    {}

    public function doNotFailOnIncomplete(): bool
    {}

    public function doNotFailOnNotice(): bool
    {}

    public function doNotFailOnRisky(): bool
    {}

    public function doNotFailOnSkipped(): bool
    {}

    public function doNotFailOnWarning(): bool
    {}

    /**
     * @deprecated
     *
     * @codeCoverageIgnore
     */
    public function stopOnDefect(): bool
    {}

    /**
     * @return non-negative-int
     */
    public function stopOnDefectThreshold(): int
    {}

    /**
     * @deprecated
     *
     * @codeCoverageIgnore
     */
    public function stopOnDeprecation(): bool
    {}

    /**
     * @return non-negative-int
     */
    public function stopOnDeprecationThreshold(): int
    {}

    /**
     * @phpstan-assert-if-true !null $this->specificDeprecationToStopOn
     */
    public function hasSpecificDeprecationToStopOn(): bool
    {}

    /**
     * @throws SpecificDeprecationToStopOnNotConfiguredException
     *
     * @return non-empty-string
     */
    public function specificDeprecationToStopOn(): string
    {}

    /**
     * @deprecated
     *
     * @codeCoverageIgnore
     */
    public function stopOnError(): bool
    {}

    /**
     * @return non-negative-int
     */
    public function stopOnErrorThreshold(): int
    {}

    /**
     * @deprecated
     *
     * @codeCoverageIgnore
     */
    public function stopOnFailure(): bool
    {}

    /**
     * @return non-negative-int
     */
    public function stopOnFailureThreshold(): int
    {}

    /**
     * @deprecated
     *
     * @codeCoverageIgnore
     */
    public function stopOnIncomplete(): bool
    {}

    /**
     * @return non-negative-int
     */
    public function stopOnIncompleteThreshold(): int
    {}

    /**
     * @deprecated
     *
     * @codeCoverageIgnore
     */
    public function stopOnNotice(): bool
    {}

    /**
     * @return non-negative-int
     */
    public function stopOnNoticeThreshold(): int
    {}

    /**
     * @deprecated
     *
     * @codeCoverageIgnore
     */
    public function stopOnRisky(): bool
    {}

    /**
     * @return non-negative-int
     */
    public function stopOnRiskyThreshold(): int
    {}

    /**
     * @deprecated
     *
     * @codeCoverageIgnore
     */
    public function stopOnSkipped(): bool
    {}

    /**
     * @return non-negative-int
     */
    public function stopOnSkippedThreshold(): int
    {}

    /**
     * @deprecated
     *
     * @codeCoverageIgnore
     */
    public function stopOnWarning(): bool
    {}

    /**
     * @return non-negative-int
     */
    public function stopOnWarningThreshold(): int
    {}

    public function outputToStandardErrorStream(): bool
    {}

    /**
     * @return positive-int
     */
    public function columns(): int
    {}

    public function noExtensions(): bool
    {}

    /**
     * @phpstan-assert-if-true !null $this->pharExtensionDirectory
     */
    public function hasPharExtensionDirectory(): bool
    {}

    /**
     * @throws NoPharExtensionDirectoryException
     *
     * @return non-empty-string
     */
    public function pharExtensionDirectory(): string
    {}

    /**
     * @return list<array{className: non-empty-string, parameters: array<string, string>}>
     */
    public function extensionBootstrappers(): array
    {}

    public function backupGlobals(): bool
    {}

    public function backupStaticProperties(): bool
    {}

    public function beStrictAboutChangesToGlobalState(): bool
    {}

    public function colors(): bool
    {}

    public function processIsolation(): bool
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
    public function diffContext(): int
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

    public function reportUselessTests(): bool
    {}

    public function strictCoverage(): bool
    {}

    public function requireCoverageContribution(): bool
    {}

    public function disallowTestOutput(): bool
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

    public function noProgress(): bool
    {}

    public function noResults(): bool
    {}

    public function noOutput(): bool
    {}

    public function executionOrder(): int
    {}

    public function executionOrderDefects(): int
    {}

    public function resolveDependencies(): bool
    {}

    /**
     * @phpstan-assert-if-true !null $this->logfileTeamcity
     */
    public function hasLogfileTeamcity(): bool
    {}

    /**
     * @throws LoggingNotConfiguredException
     */
    public function logfileTeamcity(): string
    {}

    /**
     * @phpstan-assert-if-true !null $this->logfileJunit
     */
    public function hasLogfileJunit(): bool
    {}

    /**
     * @throws LoggingNotConfiguredException
     */
    public function logfileJunit(): string
    {}

    /**
     * @phpstan-assert-if-true !null $this->logfileOtr
     */
    public function hasLogfileOtr(): bool
    {}

    /**
     * @throws LoggingNotConfiguredException
     *
     * @return non-empty-string
     */
    public function logfileOtr(): string
    {}

    public function includeGitInformationInOtrLogfile(): bool
    {}

    public function includeGitInformation(): bool
    {}

    /**
     * @phpstan-assert-if-true !null $this->logfileTestdoxHtml
     */
    public function hasLogfileTestdoxHtml(): bool
    {}

    /**
     * @throws LoggingNotConfiguredException
     */
    public function logfileTestdoxHtml(): string
    {}

    /**
     * @phpstan-assert-if-true !null $this->logfileTestdoxText
     */
    public function hasLogfileTestdoxText(): bool
    {}

    /**
     * @throws LoggingNotConfiguredException
     */
    public function logfileTestdoxText(): string
    {}

    /**
     * @phpstan-assert-if-true !null $this->logEventsText
     */
    public function hasLogEventsText(): bool
    {}

    /**
     * @throws LoggingNotConfiguredException
     *
     * @return non-empty-string
     */
    public function logEventsText(): string
    {}

    /**
     * @phpstan-assert-if-true !null $this->logEventsVerboseText
     */
    public function hasLogEventsVerboseText(): bool
    {}

    /**
     * @throws LoggingNotConfiguredException
     *
     * @return non-empty-string
     */
    public function logEventsVerboseText(): string
    {}

    public function outputIsCompact(): bool
    {}

    public function outputIsTeamCity(): bool
    {}

    public function outputIsTestDox(): bool
    {}

    public function testDoxOutputWithSummary(): bool
    {}

    /**
     * @phpstan-assert-if-true !empty $this->testsCovering
     */
    public function hasTestsCovering(): bool
    {}

    /**
     * @throws FilterNotConfiguredException
     *
     * @return list<string>
     */
    public function testsCovering(): array
    {}

    /**
     * @phpstan-assert-if-true !empty $this->testsUsing
     */
    public function hasTestsUsing(): bool
    {}

    /**
     * @throws FilterNotConfiguredException
     *
     * @return list<string>
     */
    public function testsUsing(): array
    {}

    /**
     * @phpstan-assert-if-true !empty $this->testsRequiringPhpExtension
     */
    public function hasTestsRequiringPhpExtension(): bool
    {}

    /**
     * @throws FilterNotConfiguredException
     *
     * @return non-empty-list<non-empty-string>
     */
    public function testsRequiringPhpExtension(): array
    {}

    /**
     * @phpstan-assert-if-true !null $this->filter
     */
    public function hasFilter(): bool
    {}

    /**
     * @throws FilterNotConfiguredException
     */
    public function filter(): string
    {}

    /**
     * @phpstan-assert-if-true !null $this->excludeFilter
     */
    public function hasExcludeFilter(): bool
    {}

    /**
     * @throws FilterNotConfiguredException
     */
    public function excludeFilter(): string
    {}

    /**
     * @phpstan-assert-if-true !null $this->testIdFilterFile
     */
    public function hasTestIdFilterFile(): bool
    {}

    /**
     * @throws FilterNotConfiguredException
     *
     * @return non-empty-string
     */
    public function testIdFilterFile(): string
    {}

    /**
     * @phpstan-assert-if-true !null $this->testIdFilter
     */
    public function hasTestIdFilter(): bool
    {}

    /**
     * @throws FilterNotConfiguredException
     *
     * @return non-empty-string
     */
    public function testIdFilter(): string
    {}

    /**
     * @phpstan-assert-if-true !empty $this->groups
     */
    public function hasGroups(): bool
    {}

    /**
     * @throws FilterNotConfiguredException
     *
     * @return non-empty-list<non-empty-string>
     */
    public function groups(): array
    {}

    /**
     * @phpstan-assert-if-true !empty $this->excludeGroups
     */
    public function hasExcludeGroups(): bool
    {}

    /**
     * @throws FilterNotConfiguredException
     *
     * @return non-empty-list<non-empty-string>
     */
    public function excludeGroups(): array
    {}

    /**
     * @return positive-int
     */
    public function randomOrderSeed(): int
    {}

    /**
     * @return positive-int
     */
    public function repeat(): int
    {}

    /**
     * @return positive-int
     */
    public function retry(): int
    {}

    /**
     * @phpstan-assert-if-true positive-int $this->timeout
     */
    public function hasTimeout(): bool
    {}

    /**
     * The time limit for the test run in seconds.
     *
     * @throws TimeoutNotConfiguredException
     *
     * @return positive-int
     */
    public function timeout(): int
    {}

    public function includeUncoveredFiles(): bool
    {}

    public function testSuite(): TestSuiteCollection
    {}

    /**
     * @return list<non-empty-string>
     */
    public function includeTestSuites(): array
    {}

    /**
     * @return list<non-empty-string>
     */
    public function excludeTestSuites(): array
    {}

    /**
     * @phpstan-assert-if-true !null $this->defaultTestSuite
     */
    public function hasDefaultTestSuite(): bool
    {}

    /**
     * @throws NoDefaultTestSuiteException
     *
     * @return non-empty-string
     */
    public function defaultTestSuite(): string
    {}

    public function ignoreTestSelectionInXmlConfiguration(): bool
    {}

    /**
     * @return non-empty-list<non-empty-string>
     */
    public function testSuffixes(): array
    {}

    public function php(): Php
    {}

    public function controlGarbageCollector(): bool
    {}

    /**
     * @return positive-int
     */
    public function numberOfTestsBeforeGarbageCollection(): int
    {}

    /**
     * @phpstan-assert-if-true !null $this->generateBaseline
     */
    public function hasGenerateBaseline(): bool
    {}

    /**
     * @throws NoBaselineException
     *
     * @return non-empty-string
     */
    public function generateBaseline(): string
    {}

    public function debug(): bool
    {}

    public function withTelemetry(): bool
    {}

    /**
     * @return non-negative-int
     */
    public function shortenArraysForExportThreshold(): int
    {}

    public function warnWhenPhpIsNotConfiguredForDevelopment(): bool
    {}
}
