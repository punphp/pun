<?php declare(strict_types=1);
/*
 * This file is part of PHPUnit.
 *
 * (c) Sebastian Bergmann <sebastian@phpunit.de>
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */
namespace PHPUnit\TextUI\CliArguments;

/**
 * @no-named-arguments Parameter names are not covered by the backward compatibility promise for PHPUnit
 *
 * @internal This class is not covered by the backward compatibility promise for PHPUnit
 *
 * @immutable
 */
final readonly class Configuration
{
    /**
     * @var list<non-empty-string>
     */
    private array $arguments;
    private ?string $testFilesFile;
    private ?string $testIdFile;
    private ?string $testIdFilter;
    private ?bool $all;
    private ?string $atLeastVersion;
    private ?bool $backupGlobals;
    private ?bool $backupStaticProperties;
    private ?bool $beStrictAboutChangesToGlobalState;
    private ?string $bootstrap;
    private ?string $cacheDirectory;
    private ?bool $recordTestRunHistory;
    private bool $checkPhpConfiguration;
    private ?bool $warnWhenPhpIsNotConfiguredForDevelopment;
    private bool $checkVersion;
    private ?string $colors;
    private null|int|string $columns;
    private ?string $configurationFile;

    /**
     * @var ?non-empty-list<non-empty-string>
     */
    private ?array $coverageFilter;
    private ?string $coverageClover;
    private ?string $coverageCobertura;
    private ?string $coverageCrap4J;
    private ?string $coverageHtml;
    private ?bool $withoutClassView;
    private ?bool $withoutFileView;
    private ?string $coverageJsonl;
    private ?string $coverageOpenClover;
    private ?string $coveragePhp;
    private ?string $coverageText;
    private ?bool $coverageTextShowUncoveredFiles;
    private ?bool $coverageTextShowOnlySummary;
    private ?string $coverageXml;
    private ?bool $excludeSourceFromXmlCoverage;
    private ?bool $pathCoverage;
    private ?bool $branchCoverage;
    private bool $warmCoverageCache;
    private ?int $defaultTimeLimit;

    /**
     * @var ?positive-int
     */
    private ?int $diffContext;
    private ?bool $disableCodeCoverageIgnore;
    private ?bool $disableCoverageTargeting;
    private ?bool $disallowTestOutput;
    private ?bool $enforceTimeLimit;

    /**
     * @var ?non-empty-list<non-empty-string>
     */
    private ?array $excludeGroups;
    private ?int $executionOrder;
    private ?int $executionOrderDefects;
    private ?bool $failOnAllIssues;
    private ?bool $failOnDeprecation;
    private ?bool $failOnSelfDeprecation;
    private ?bool $failOnDirectDeprecation;
    private ?bool $failOnIndirectDeprecation;
    private ?bool $failOnPhpunitDeprecation;
    private ?bool $failOnPhpunitNotice;
    private ?bool $failOnPhpunitWarning;
    private ?bool $failOnEmptyTestSuite;
    private ?bool $failOnIncomplete;
    private ?bool $failOnNotice;
    private ?bool $failOnRisky;
    private ?bool $failOnSkipped;
    private ?bool $failOnWarning;
    private ?bool $doNotFailOnDeprecation;
    private ?bool $doNotFailOnSelfDeprecation;
    private ?bool $doNotFailOnDirectDeprecation;
    private ?bool $doNotFailOnIndirectDeprecation;
    private ?bool $doNotFailOnPhpunitDeprecation;
    private ?bool $doNotFailOnPhpunitNotice;
    private ?bool $doNotFailOnPhpunitWarning;
    private ?bool $doNotFailOnEmptyTestSuite;
    private ?bool $doNotFailOnIncomplete;
    private ?bool $doNotFailOnNotice;
    private ?bool $doNotFailOnRisky;
    private ?bool $doNotFailOnSkipped;
    private ?bool $doNotFailOnWarning;
    private ?int $stopOnDefect;
    private ?int $stopOnDeprecation;
    private ?string $specificDeprecationToStopOn;
    private ?int $stopOnError;
    private ?int $stopOnFailure;
    private ?int $stopOnIncomplete;
    private ?int $stopOnNotice;
    private ?int $stopOnRisky;
    private ?int $stopOnSkipped;
    private ?int $stopOnWarning;
    private ?string $filter;
    private ?string $excludeFilter;
    private ?string $generateBaseline;
    private ?string $useBaseline;
    private bool $ignoreBaseline;
    private bool $generateConfiguration;
    private bool $migrateConfiguration;
    private bool $validateConfiguration;

    /**
     * @var ?non-empty-list<non-empty-string>
     */
    private ?array $groups;

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
    private bool $help;
    private ?string $includePath;

    /**
     * @var ?non-empty-array<non-empty-string, non-empty-string>
     */
    private ?array $iniSettings;
    private ?string $junitLogfile;
    private ?string $otrLogfile;
    private ?bool $includeGitInformation;
    private bool $listGroups;
    private bool $listSuites;
    private bool $listTestFiles;
    private bool $listTestIds;
    private bool $listTests;
    private ?string $listTestsXml;
    private ?bool $noCoverage;
    private ?bool $noExtensions;
    private ?bool $noOutput;
    private ?bool $noProgress;
    private ?bool $noResults;
    private ?bool $noLogging;
    private ?bool $processIsolation;
    private ?int $randomOrderSeed;

    /**
     * @var ?positive-int
     */
    private ?int $repeat;

    /**
     * @var ?positive-int
     */
    private ?int $retry;

    /**
     * @var ?positive-int
     */
    private ?int $timeout;
    private ?bool $reportUselessTests;
    private ?bool $resolveDependencies;
    private ?bool $reverseList;
    private ?bool $stderr;
    private ?bool $strictCoverage;
    private ?bool $requireCoverageContribution;
    private ?string $teamcityLogfile;
    private ?bool $compactPrinter;
    private ?bool $teamCityPrinter;
    private ?string $testdoxHtmlFile;
    private ?string $testdoxTextFile;
    private ?bool $testdoxPrinter;
    private ?bool $testdoxPrinterSummary;

    /**
     * @var ?non-empty-list<non-empty-string>
     */
    private ?array $testSuffixes;
    private ?string $testSuite;
    private ?string $excludeTestSuite;
    private bool $useDefaultConfiguration;
    private ?bool $displayDetailsOnAllIssues;
    private ?bool $displayDetailsOnIncompleteTests;
    private ?bool $displayDetailsOnSkippedTests;
    private ?bool $displayDetailsOnTestsThatTriggerDeprecations;
    private ?bool $displayDetailsOnPhpunitDeprecations;
    private ?bool $displayDetailsOnPhpunitNotices;
    private ?bool $displayDetailsOnTestsThatTriggerErrors;
    private ?bool $displayDetailsOnTestsThatTriggerNotices;
    private ?bool $displayDetailsOnTestsThatTriggerWarnings;
    private bool $version;
    private ?string $logEventsText;
    private ?string $logEventsVerboseText;
    private bool $debug;
    private bool $withTelemetry;

    /**
     * @var ?non-empty-list<non-empty-string>
     */
    private ?array $extensions;
    private ?bool $cacheTestIndex;

    /**
     * @param list<non-empty-string>                               $arguments
     * @param ?positive-int                                        $diffContext
     * @param ?non-empty-list<non-empty-string>                    $excludeGroups
     * @param ?non-empty-list<non-empty-string>                    $groups
     * @param ?non-empty-list<non-empty-string>                    $testsCovering
     * @param ?non-empty-list<non-empty-string>                    $testsUsing
     * @param ?non-empty-list<non-empty-string>                    $testsRequiringPhpExtension
     * @param ?non-empty-array<non-empty-string, non-empty-string> $iniSettings
     * @param ?positive-int                                        $repeat
     * @param ?positive-int                                        $retry
     * @param ?positive-int                                        $timeout
     * @param ?non-empty-list<non-empty-string>                    $testSuffixes
     * @param ?non-empty-list<non-empty-string>                    $coverageFilter
     * @param ?non-empty-list<non-empty-string>                    $extensions
     */
    public function __construct(array $arguments, ?string $testFilesFile, ?string $testIdFile, ?string $testIdFilter, ?bool $all, ?string $atLeastVersion, ?bool $backupGlobals, ?bool $backupStaticProperties, ?bool $beStrictAboutChangesToGlobalState, ?string $bootstrap, ?string $cacheDirectory, ?bool $recordTestRunHistory, bool $checkPhpConfiguration, ?bool $warnWhenPhpIsNotConfiguredForDevelopment, bool $checkVersion, ?string $colors, null|int|string $columns, ?string $configurationFile, ?string $coverageClover, ?string $coverageCobertura, ?string $coverageCrap4J, ?string $coverageHtml, ?bool $withoutClassView, ?bool $withoutFileView, ?string $coverageJsonl, ?string $coverageOpenClover, ?string $coveragePhp, ?string $coverageText, ?bool $coverageTextShowUncoveredFiles, ?bool $coverageTextShowOnlySummary, ?string $coverageXml, ?bool $coverageXmlIncludeSource, ?bool $pathCoverage, ?bool $branchCoverage, bool $warmCoverageCache, ?int $defaultTimeLimit, ?int $diffContext, ?bool $disableCodeCoverageIgnore, ?bool $disableCoverageTargeting, ?bool $disallowTestOutput, ?bool $enforceTimeLimit, ?array $excludeGroups, ?int $executionOrder, ?int $executionOrderDefects, ?bool $failOnAllIssues, ?bool $failOnDeprecation, ?bool $failOnSelfDeprecation, ?bool $failOnDirectDeprecation, ?bool $failOnIndirectDeprecation, ?bool $failOnPhpunitDeprecation, ?bool $failOnPhpunitNotice, ?bool $failOnPhpunitWarning, ?bool $failOnEmptyTestSuite, ?bool $failOnIncomplete, ?bool $failOnNotice, ?bool $failOnRisky, ?bool $failOnSkipped, ?bool $failOnWarning, ?bool $doNotFailOnDeprecation, ?bool $doNotFailOnSelfDeprecation, ?bool $doNotFailOnDirectDeprecation, ?bool $doNotFailOnIndirectDeprecation, ?bool $doNotFailOnPhpunitDeprecation, ?bool $doNotFailOnPhpunitNotice, ?bool $doNotFailOnPhpunitWarning, ?bool $doNotFailOnEmptyTestSuite, ?bool $doNotFailOnIncomplete, ?bool $doNotFailOnNotice, ?bool $doNotFailOnRisky, ?bool $doNotFailOnSkipped, ?bool $doNotFailOnWarning, ?int $stopOnDefect, ?int $stopOnDeprecation, ?string $specificDeprecationToStopOn, ?int $stopOnError, ?int $stopOnFailure, ?int $stopOnIncomplete, ?int $stopOnNotice, ?int $stopOnRisky, ?int $stopOnSkipped, ?int $stopOnWarning, ?string $filter, ?string $excludeFilter, ?string $generateBaseline, ?string $useBaseline, bool $ignoreBaseline, bool $generateConfiguration, bool $migrateConfiguration, bool $validateConfiguration, ?array $groups, ?array $testsCovering, ?array $testsUsing, ?array $testsRequiringPhpExtension, bool $help, ?string $includePath, ?array $iniSettings, ?string $junitLogfile, ?string $otrLogfile, ?bool $includeGitInformation, bool $listGroups, bool $listSuites, bool $listTestFiles, bool $listTestIds, bool $listTests, ?string $listTestsXml, ?bool $noCoverage, ?bool $noExtensions, ?bool $noOutput, ?bool $noProgress, ?bool $noResults, ?bool $noLogging, ?bool $processIsolation, ?int $randomOrderSeed, ?int $repeat, ?int $retry, ?int $timeout, ?bool $reportUselessTests, ?bool $resolveDependencies, ?bool $reverseList, ?bool $stderr, ?bool $strictCoverage, ?bool $requireCoverageContribution, ?string $teamcityLogfile, ?string $testdoxHtmlFile, ?string $testdoxTextFile, ?array $testSuffixes, ?string $testSuite, ?string $excludeTestSuite, bool $useDefaultConfiguration, ?bool $displayDetailsOnAllIssues, ?bool $displayDetailsOnIncompleteTests, ?bool $displayDetailsOnSkippedTests, ?bool $displayDetailsOnTestsThatTriggerDeprecations, ?bool $displayDetailsOnPhpunitDeprecations, ?bool $displayDetailsOnPhpunitNotices, ?bool $displayDetailsOnTestsThatTriggerErrors, ?bool $displayDetailsOnTestsThatTriggerNotices, ?bool $displayDetailsOnTestsThatTriggerWarnings, bool $version, ?array $coverageFilter, ?string $logEventsText, ?string $logEventsVerboseText, ?bool $printerCompact, ?bool $printerTeamCity, ?bool $testdoxPrinter, ?bool $testdoxPrinterSummary, bool $debug, bool $withTelemetry, ?array $extensions, ?bool $cacheTestIndex)
    {}

    /**
     * @return list<non-empty-string>
     */
    public function arguments(): array
    {}

    /**
     * @phpstan-assert-if-true !null $this->testFilesFile
     */
    public function hasTestFilesFile(): bool
    {}

    /**
     * @throws Exception
     */
    public function testFilesFile(): string
    {}

    /**
     * @phpstan-assert-if-true !null $this->testIdFile
     */
    public function hasTestIdFile(): bool
    {}

    /**
     * @throws Exception
     */
    public function testIdFile(): string
    {}

    /**
     * @phpstan-assert-if-true !null $this->testIdFilter
     */
    public function hasTestIdFilter(): bool
    {}

    /**
     * @throws Exception
     */
    public function testIdFilter(): string
    {}

    /**
     * @phpstan-assert-if-true !null $this->all
     */
    public function hasAll(): bool
    {}

    /**
     * @throws Exception
     */
    public function all(): bool
    {}

    /**
     * @phpstan-assert-if-true !null $this->atLeastVersion
     */
    public function hasAtLeastVersion(): bool
    {}

    /**
     * @throws Exception
     */
    public function atLeastVersion(): string
    {}

    /**
     * @phpstan-assert-if-true !null $this->backupGlobals
     */
    public function hasBackupGlobals(): bool
    {}

    /**
     * @throws Exception
     */
    public function backupGlobals(): bool
    {}

    /**
     * @phpstan-assert-if-true !null $this->backupStaticProperties
     */
    public function hasBackupStaticProperties(): bool
    {}

    /**
     * @throws Exception
     */
    public function backupStaticProperties(): bool
    {}

    /**
     * @phpstan-assert-if-true !null $this->beStrictAboutChangesToGlobalState
     */
    public function hasBeStrictAboutChangesToGlobalState(): bool
    {}

    /**
     * @throws Exception
     */
    public function beStrictAboutChangesToGlobalState(): bool
    {}

    /**
     * @phpstan-assert-if-true !null $this->bootstrap
     */
    public function hasBootstrap(): bool
    {}

    /**
     * @throws Exception
     */
    public function bootstrap(): string
    {}

    /**
     * @phpstan-assert-if-true !null $this->cacheDirectory
     */
    public function hasCacheDirectory(): bool
    {}

    /**
     * @throws Exception
     */
    public function cacheDirectory(): string
    {}

    /**
     * @phpstan-assert-if-true !null $this->cacheTestIndex
     */
    public function hasCacheTestIndex(): bool
    {}

    /**
     * @throws Exception
     */
    public function cacheTestIndex(): bool
    {}

    /**
     * @phpstan-assert-if-true !null $this->recordTestRunHistory
     */
    public function hasRecordTestRunHistory(): bool
    {}

    /**
     * @throws Exception
     */
    public function recordTestRunHistory(): bool
    {}

    public function checkPhpConfiguration(): bool
    {}

    /**
     * @phpstan-assert-if-true !null $this->warnWhenPhpIsNotConfiguredForDevelopment
     */
    public function hasWarnWhenPhpIsNotConfiguredForDevelopment(): bool
    {}

    /**
     * @throws Exception
     */
    public function warnWhenPhpIsNotConfiguredForDevelopment(): bool
    {}

    public function checkVersion(): bool
    {}

    /**
     * @phpstan-assert-if-true !null $this->colors
     */
    public function hasColors(): bool
    {}

    /**
     * @throws Exception
     */
    public function colors(): string
    {}

    /**
     * @phpstan-assert-if-true !null $this->columns
     */
    public function hasColumns(): bool
    {}

    /**
     * @throws Exception
     */
    public function columns(): int|string
    {}

    /**
     * @phpstan-assert-if-true !null $this->configurationFile
     */
    public function hasConfigurationFile(): bool
    {}

    /**
     * @throws Exception
     */
    public function configurationFile(): string
    {}

    /**
     * @phpstan-assert-if-true !null $this->coverageFilter
     */
    public function hasCoverageFilter(): bool
    {}

    /**
     * @throws Exception
     *
     * @return non-empty-list<non-empty-string>
     */
    public function coverageFilter(): array
    {}

    /**
     * @phpstan-assert-if-true !null $this->coverageClover
     */
    public function hasCoverageClover(): bool
    {}

    /**
     * @throws Exception
     */
    public function coverageClover(): string
    {}

    /**
     * @phpstan-assert-if-true !null $this->coverageCobertura
     */
    public function hasCoverageCobertura(): bool
    {}

    /**
     * @throws Exception
     */
    public function coverageCobertura(): string
    {}

    /**
     * @phpstan-assert-if-true !null $this->coverageCrap4J
     */
    public function hasCoverageCrap4J(): bool
    {}

    /**
     * @throws Exception
     */
    public function coverageCrap4J(): string
    {}

    /**
     * @phpstan-assert-if-true !null $this->coverageHtml
     */
    public function hasCoverageHtml(): bool
    {}

    /**
     * @throws Exception
     */
    public function coverageHtml(): string
    {}

    /**
     * @phpstan-assert-if-true !null $this->withoutClassView
     */
    public function hasWithoutClassView(): bool
    {}

    /**
     * @throws Exception
     */
    public function withoutClassView(): bool
    {}

    /**
     * @phpstan-assert-if-true !null $this->withoutFileView
     */
    public function hasWithoutFileView(): bool
    {}

    /**
     * @throws Exception
     */
    public function withoutFileView(): bool
    {}

    /**
     * @phpstan-assert-if-true !null $this->coverageJsonl
     */
    public function hasCoverageJsonl(): bool
    {}

    /**
     * @throws Exception
     */
    public function coverageJsonl(): string
    {}

    /**
     * @phpstan-assert-if-true !null $this->coverageOpenClover
     */
    public function hasCoverageOpenClover(): bool
    {}

    /**
     * @throws Exception
     */
    public function coverageOpenClover(): string
    {}

    /**
     * @phpstan-assert-if-true !null $this->coveragePhp
     */
    public function hasCoveragePhp(): bool
    {}

    /**
     * @throws Exception
     */
    public function coveragePhp(): string
    {}

    /**
     * @phpstan-assert-if-true !null $this->coverageText
     */
    public function hasCoverageText(): bool
    {}

    /**
     * @throws Exception
     */
    public function coverageText(): string
    {}

    /**
     * @phpstan-assert-if-true !null $this->coverageTextShowUncoveredFiles
     */
    public function hasCoverageTextShowUncoveredFiles(): bool
    {}

    /**
     * @throws Exception
     */
    public function coverageTextShowUncoveredFiles(): bool
    {}

    /**
     * @phpstan-assert-if-true !null $this->coverageTextShowOnlySummary
     */
    public function hasCoverageTextShowOnlySummary(): bool
    {}

    /**
     * @throws Exception
     */
    public function coverageTextShowOnlySummary(): bool
    {}

    /**
     * @phpstan-assert-if-true !null $this->coverageXml
     */
    public function hasCoverageXml(): bool
    {}

    /**
     * @throws Exception
     */
    public function coverageXml(): string
    {}

    /**
     * @phpstan-assert-if-true !null $this->excludeSourceFromXmlCoverage
     */
    public function hasExcludeSourceFromXmlCoverage(): bool
    {}

    /**
     * @throws Exception
     */
    public function excludeSourceFromXmlCoverage(): bool
    {}

    /**
     * @phpstan-assert-if-true !null $this->pathCoverage
     */
    public function hasPathCoverage(): bool
    {}

    /**
     * @throws Exception
     */
    public function pathCoverage(): bool
    {}

    /**
     * @phpstan-assert-if-true !null $this->branchCoverage
     */
    public function hasBranchCoverage(): bool
    {}

    /**
     * @throws Exception
     */
    public function branchCoverage(): bool
    {}

    public function warmCoverageCache(): bool
    {}

    /**
     * @phpstan-assert-if-true !null $this->defaultTimeLimit
     */
    public function hasDefaultTimeLimit(): bool
    {}

    /**
     * @throws Exception
     */
    public function defaultTimeLimit(): int
    {}

    /**
     * @phpstan-assert-if-true !null $this->diffContext
     */
    public function hasDiffContext(): bool
    {}

    /**
     * @throws Exception
     *
     * @return positive-int
     */
    public function diffContext(): int
    {}

    /**
     * @phpstan-assert-if-true !null $this->disableCodeCoverageIgnore
     */
    public function hasDisableCodeCoverageIgnore(): bool
    {}

    /**
     * @throws Exception
     */
    public function disableCodeCoverageIgnore(): bool
    {}

    /**
     * @phpstan-assert-if-true !null $this->disableCoverageTargeting
     */
    public function hasDisableCoverageTargeting(): bool
    {}

    /**
     * @throws Exception
     */
    public function disableCoverageTargeting(): bool
    {}

    /**
     * @phpstan-assert-if-true !null $this->disallowTestOutput
     */
    public function hasDisallowTestOutput(): bool
    {}

    /**
     * @throws Exception
     */
    public function disallowTestOutput(): bool
    {}

    /**
     * @phpstan-assert-if-true !null $this->enforceTimeLimit
     */
    public function hasEnforceTimeLimit(): bool
    {}

    /**
     * @throws Exception
     */
    public function enforceTimeLimit(): bool
    {}

    /**
     * @phpstan-assert-if-true !null $this->excludeGroups
     */
    public function hasExcludeGroups(): bool
    {}

    /**
     * @throws Exception
     *
     * @return non-empty-list<non-empty-string>
     */
    public function excludeGroups(): array
    {}

    /**
     * @phpstan-assert-if-true !null $this->executionOrder
     */
    public function hasExecutionOrder(): bool
    {}

    /**
     * @throws Exception
     */
    public function executionOrder(): int
    {}

    /**
     * @phpstan-assert-if-true !null $this->executionOrderDefects
     */
    public function hasExecutionOrderDefects(): bool
    {}

    /**
     * @throws Exception
     */
    public function executionOrderDefects(): int
    {}

    /**
     * @phpstan-assert-if-true !null $this->failOnAllIssues
     */
    public function hasFailOnAllIssues(): bool
    {}

    /**
     * @throws Exception
     */
    public function failOnAllIssues(): bool
    {}

    /**
     * @phpstan-assert-if-true !null $this->failOnDeprecation
     */
    public function hasFailOnDeprecation(): bool
    {}

    /**
     * @throws Exception
     */
    public function failOnDeprecation(): bool
    {}

    /**
     * @phpstan-assert-if-true !null $this->failOnSelfDeprecation
     */
    public function hasFailOnSelfDeprecation(): bool
    {}

    /**
     * @throws Exception
     */
    public function failOnSelfDeprecation(): bool
    {}

    /**
     * @phpstan-assert-if-true !null $this->failOnDirectDeprecation
     */
    public function hasFailOnDirectDeprecation(): bool
    {}

    /**
     * @throws Exception
     */
    public function failOnDirectDeprecation(): bool
    {}

    /**
     * @phpstan-assert-if-true !null $this->failOnIndirectDeprecation
     */
    public function hasFailOnIndirectDeprecation(): bool
    {}

    /**
     * @throws Exception
     */
    public function failOnIndirectDeprecation(): bool
    {}

    /**
     * @phpstan-assert-if-true !null $this->failOnPhpunitDeprecation
     */
    public function hasFailOnPhpunitDeprecation(): bool
    {}

    /**
     * @throws Exception
     */
    public function failOnPhpunitDeprecation(): bool
    {}

    /**
     * @phpstan-assert-if-true !null $this->failOnPhpunitNotice
     */
    public function hasFailOnPhpunitNotice(): bool
    {}

    /**
     * @throws Exception
     */
    public function failOnPhpunitNotice(): bool
    {}

    /**
     * @phpstan-assert-if-true !null $this->failOnPhpunitWarning
     */
    public function hasFailOnPhpunitWarning(): bool
    {}

    /**
     * @throws Exception
     */
    public function failOnPhpunitWarning(): bool
    {}

    /**
     * @phpstan-assert-if-true !null $this->failOnEmptyTestSuite
     */
    public function hasFailOnEmptyTestSuite(): bool
    {}

    /**
     * @throws Exception
     */
    public function failOnEmptyTestSuite(): bool
    {}

    /**
     * @phpstan-assert-if-true !null $this->failOnIncomplete
     */
    public function hasFailOnIncomplete(): bool
    {}

    /**
     * @throws Exception
     */
    public function failOnIncomplete(): bool
    {}

    /**
     * @phpstan-assert-if-true !null $this->failOnNotice
     */
    public function hasFailOnNotice(): bool
    {}

    /**
     * @throws Exception
     */
    public function failOnNotice(): bool
    {}

    /**
     * @phpstan-assert-if-true !null $this->failOnRisky
     */
    public function hasFailOnRisky(): bool
    {}

    /**
     * @throws Exception
     */
    public function failOnRisky(): bool
    {}

    /**
     * @phpstan-assert-if-true !null $this->failOnSkipped
     */
    public function hasFailOnSkipped(): bool
    {}

    /**
     * @throws Exception
     */
    public function failOnSkipped(): bool
    {}

    /**
     * @phpstan-assert-if-true !null $this->failOnWarning
     */
    public function hasFailOnWarning(): bool
    {}

    /**
     * @throws Exception
     */
    public function failOnWarning(): bool
    {}

    /**
     * @phpstan-assert-if-true !null $this->doNotFailOnDeprecation
     */
    public function hasDoNotFailOnDeprecation(): bool
    {}

    /**
     * @throws Exception
     */
    public function doNotFailOnDeprecation(): bool
    {}

    /**
     * @phpstan-assert-if-true !null $this->doNotFailOnSelfDeprecation
     */
    public function hasDoNotFailOnSelfDeprecation(): bool
    {}

    /**
     * @throws Exception
     */
    public function doNotFailOnSelfDeprecation(): bool
    {}

    /**
     * @phpstan-assert-if-true !null $this->doNotFailOnDirectDeprecation
     */
    public function hasDoNotFailOnDirectDeprecation(): bool
    {}

    /**
     * @throws Exception
     */
    public function doNotFailOnDirectDeprecation(): bool
    {}

    /**
     * @phpstan-assert-if-true !null $this->doNotFailOnIndirectDeprecation
     */
    public function hasDoNotFailOnIndirectDeprecation(): bool
    {}

    /**
     * @throws Exception
     */
    public function doNotFailOnIndirectDeprecation(): bool
    {}

    /**
     * @phpstan-assert-if-true !null $this->doNotFailOnPhpunitDeprecation
     */
    public function hasDoNotFailOnPhpunitDeprecation(): bool
    {}

    /**
     * @throws Exception
     */
    public function doNotFailOnPhpunitDeprecation(): bool
    {}

    /**
     * @phpstan-assert-if-true !null $this->doNotFailOnPhpunitNotice
     */
    public function hasDoNotFailOnPhpunitNotice(): bool
    {}

    /**
     * @throws Exception
     */
    public function doNotFailOnPhpunitNotice(): bool
    {}

    /**
     * @phpstan-assert-if-true !null $this->doNotFailOnPhpunitWarning
     */
    public function hasDoNotFailOnPhpunitWarning(): bool
    {}

    /**
     * @throws Exception
     */
    public function doNotFailOnPhpunitWarning(): bool
    {}

    /**
     * @phpstan-assert-if-true !null $this->doNotFailOnEmptyTestSuite
     */
    public function hasDoNotFailOnEmptyTestSuite(): bool
    {}

    /**
     * @throws Exception
     */
    public function doNotFailOnEmptyTestSuite(): bool
    {}

    /**
     * @phpstan-assert-if-true !null $this->doNotFailOnIncomplete
     */
    public function hasDoNotFailOnIncomplete(): bool
    {}

    /**
     * @throws Exception
     */
    public function doNotFailOnIncomplete(): bool
    {}

    /**
     * @phpstan-assert-if-true !null $this->doNotFailOnNotice
     */
    public function hasDoNotFailOnNotice(): bool
    {}

    /**
     * @throws Exception
     */
    public function doNotFailOnNotice(): bool
    {}

    /**
     * @phpstan-assert-if-true !null $this->doNotFailOnRisky
     */
    public function hasDoNotFailOnRisky(): bool
    {}

    /**
     * @throws Exception
     */
    public function doNotFailOnRisky(): bool
    {}

    /**
     * @phpstan-assert-if-true !null $this->doNotFailOnSkipped
     */
    public function hasDoNotFailOnSkipped(): bool
    {}

    /**
     * @throws Exception
     */
    public function doNotFailOnSkipped(): bool
    {}

    /**
     * @phpstan-assert-if-true !null $this->doNotFailOnWarning
     */
    public function hasDoNotFailOnWarning(): bool
    {}

    /**
     * @throws Exception
     */
    public function doNotFailOnWarning(): bool
    {}

    /**
     * @phpstan-assert-if-true !null $this->stopOnDefect
     */
    public function hasStopOnDefect(): bool
    {}

    /**
     * @throws Exception
     */
    public function stopOnDefect(): int
    {}

    /**
     * @phpstan-assert-if-true !null $this->stopOnDeprecation
     */
    public function hasStopOnDeprecation(): bool
    {}

    /**
     * @throws Exception
     */
    public function stopOnDeprecation(): int
    {}

    /**
     * @phpstan-assert-if-true !null $this->specificDeprecationToStopOn
     */
    public function hasSpecificDeprecationToStopOn(): bool
    {}

    /**
     * @throws Exception
     */
    public function specificDeprecationToStopOn(): string
    {}

    /**
     * @phpstan-assert-if-true !null $this->stopOnError
     */
    public function hasStopOnError(): bool
    {}

    /**
     * @throws Exception
     */
    public function stopOnError(): int
    {}

    /**
     * @phpstan-assert-if-true !null $this->stopOnFailure
     */
    public function hasStopOnFailure(): bool
    {}

    /**
     * @throws Exception
     */
    public function stopOnFailure(): int
    {}

    /**
     * @phpstan-assert-if-true !null $this->stopOnIncomplete
     */
    public function hasStopOnIncomplete(): bool
    {}

    /**
     * @throws Exception
     */
    public function stopOnIncomplete(): int
    {}

    /**
     * @phpstan-assert-if-true !null $this->stopOnNotice
     */
    public function hasStopOnNotice(): bool
    {}

    /**
     * @throws Exception
     */
    public function stopOnNotice(): int
    {}

    /**
     * @phpstan-assert-if-true !null $this->stopOnRisky
     */
    public function hasStopOnRisky(): bool
    {}

    /**
     * @throws Exception
     */
    public function stopOnRisky(): int
    {}

    /**
     * @phpstan-assert-if-true !null $this->stopOnSkipped
     */
    public function hasStopOnSkipped(): bool
    {}

    /**
     * @throws Exception
     */
    public function stopOnSkipped(): int
    {}

    /**
     * @phpstan-assert-if-true !null $this->stopOnWarning
     */
    public function hasStopOnWarning(): bool
    {}

    /**
     * @throws Exception
     */
    public function stopOnWarning(): int
    {}

    /**
     * @phpstan-assert-if-true !null $this->excludeFilter
     */
    public function hasExcludeFilter(): bool
    {}

    /**
     * @throws Exception
     */
    public function excludeFilter(): string
    {}

    /**
     * @phpstan-assert-if-true !null $this->filter
     */
    public function hasFilter(): bool
    {}

    /**
     * @throws Exception
     */
    public function filter(): string
    {}

    /**
     * @phpstan-assert-if-true !null $this->generateBaseline
     */
    public function hasGenerateBaseline(): bool
    {}

    /**
     * @throws Exception
     */
    public function generateBaseline(): string
    {}

    /**
     * @phpstan-assert-if-true !null $this->useBaseline
     */
    public function hasUseBaseline(): bool
    {}

    /**
     * @throws Exception
     */
    public function useBaseline(): string
    {}

    public function ignoreBaseline(): bool
    {}

    public function generateConfiguration(): bool
    {}

    public function migrateConfiguration(): bool
    {}

    public function validateConfiguration(): bool
    {}

    /**
     * @phpstan-assert-if-true !null $this->groups
     */
    public function hasGroups(): bool
    {}

    /**
     * @throws Exception
     *
     * @return non-empty-list<non-empty-string>
     */
    public function groups(): array
    {}

    /**
     * @phpstan-assert-if-true !null $this->testsCovering
     */
    public function hasTestsCovering(): bool
    {}

    /**
     * @throws Exception
     *
     * @return non-empty-list<non-empty-string>
     */
    public function testsCovering(): array
    {}

    /**
     * @phpstan-assert-if-true !null $this->testsUsing
     */
    public function hasTestsUsing(): bool
    {}

    /**
     * @throws Exception
     *
     * @return non-empty-list<non-empty-string>
     */
    public function testsUsing(): array
    {}

    /**
     * @phpstan-assert-if-true !null $this->testsRequiringPhpExtension
     */
    public function hasTestsRequiringPhpExtension(): bool
    {}

    /**
     * @throws Exception
     *
     * @return non-empty-list<non-empty-string>
     */
    public function testsRequiringPhpExtension(): array
    {}

    public function help(): bool
    {}

    /**
     * @phpstan-assert-if-true !null $this->includePath
     */
    public function hasIncludePath(): bool
    {}

    /**
     * @throws Exception
     */
    public function includePath(): string
    {}

    /**
     * @phpstan-assert-if-true !null $this->iniSettings
     */
    public function hasIniSettings(): bool
    {}

    /**
     * @throws Exception
     *
     * @return non-empty-array<non-empty-string, non-empty-string>
     */
    public function iniSettings(): array
    {}

    /**
     * @phpstan-assert-if-true !null $this->junitLogfile
     */
    public function hasJunitLogfile(): bool
    {}

    /**
     * @throws Exception
     */
    public function junitLogfile(): string
    {}

    /**
     * @phpstan-assert-if-true !null $this->otrLogfile
     */
    public function hasOtrLogfile(): bool
    {}

    /**
     * @throws Exception
     */
    public function otrLogfile(): string
    {}

    /**
     * @phpstan-assert-if-true !null $this->includeGitInformation
     */
    public function hasIncludeGitInformation(): bool
    {}

    /**
     * @throws Exception
     */
    public function includeGitInformation(): bool
    {}

    public function listGroups(): bool
    {}

    public function listSuites(): bool
    {}

    public function listTestFiles(): bool
    {}

    public function listTestIds(): bool
    {}

    public function listTests(): bool
    {}

    /**
     * @phpstan-assert-if-true !null $this->listTestsXml
     */
    public function hasListTestsXml(): bool
    {}

    /**
     * @throws Exception
     */
    public function listTestsXml(): string
    {}

    /**
     * @phpstan-assert-if-true !null $this->noCoverage
     */
    public function hasNoCoverage(): bool
    {}

    /**
     * @throws Exception
     */
    public function noCoverage(): bool
    {}

    /**
     * @phpstan-assert-if-true !null $this->noExtensions
     */
    public function hasNoExtensions(): bool
    {}

    /**
     * @throws Exception
     */
    public function noExtensions(): bool
    {}

    /**
     * @phpstan-assert-if-true !null $this->noOutput
     */
    public function hasNoOutput(): bool
    {}

    /**
     * @throws Exception
     */
    public function noOutput(): bool
    {}

    /**
     * @phpstan-assert-if-true !null $this->noProgress
     */
    public function hasNoProgress(): bool
    {}

    /**
     * @throws Exception
     */
    public function noProgress(): bool
    {}

    /**
     * @phpstan-assert-if-true !null $this->noResults
     */
    public function hasNoResults(): bool
    {}

    /**
     * @throws Exception
     */
    public function noResults(): bool
    {}

    /**
     * @phpstan-assert-if-true !null $this->noLogging
     */
    public function hasNoLogging(): bool
    {}

    /**
     * @throws Exception
     */
    public function noLogging(): bool
    {}

    /**
     * @phpstan-assert-if-true !null $this->processIsolation
     */
    public function hasProcessIsolation(): bool
    {}

    /**
     * @throws Exception
     */
    public function processIsolation(): bool
    {}

    /**
     * @phpstan-assert-if-true !null $this->randomOrderSeed
     */
    public function hasRandomOrderSeed(): bool
    {}

    /**
     * @throws Exception
     */
    public function randomOrderSeed(): int
    {}

    /**
     * @phpstan-assert-if-true !null $this->repeat
     */
    public function hasRepeat(): bool
    {}

    /**
     * @throws Exception
     *
     * @return positive-int
     */
    public function repeat(): int
    {}

    /**
     * @phpstan-assert-if-true !null $this->retry
     */
    public function hasRetry(): bool
    {}

    /**
     * @throws Exception
     *
     * @return positive-int
     */
    public function retry(): int
    {}

    /**
     * @phpstan-assert-if-true !null $this->timeout
     */
    public function hasTimeout(): bool
    {}

    /**
     * @throws Exception
     *
     * @return positive-int
     */
    public function timeout(): int
    {}

    /**
     * @phpstan-assert-if-true !null $this->reportUselessTests
     */
    public function hasReportUselessTests(): bool
    {}

    /**
     * @throws Exception
     */
    public function reportUselessTests(): bool
    {}

    /**
     * @phpstan-assert-if-true !null $this->resolveDependencies
     */
    public function hasResolveDependencies(): bool
    {}

    /**
     * @throws Exception
     */
    public function resolveDependencies(): bool
    {}

    /**
     * @phpstan-assert-if-true !null $this->reverseList
     */
    public function hasReverseList(): bool
    {}

    /**
     * @throws Exception
     */
    public function reverseList(): bool
    {}

    /**
     * @phpstan-assert-if-true !null $this->stderr
     */
    public function hasStderr(): bool
    {}

    /**
     * @throws Exception
     */
    public function stderr(): bool
    {}

    /**
     * @phpstan-assert-if-true !null $this->strictCoverage
     */
    public function hasStrictCoverage(): bool
    {}

    /**
     * @throws Exception
     */
    public function strictCoverage(): bool
    {}

    /**
     * @phpstan-assert-if-true !null $this->requireCoverageContribution
     */
    public function hasRequireCoverageContribution(): bool
    {}

    /**
     * @throws Exception
     */
    public function requireCoverageContribution(): bool
    {}

    /**
     * @phpstan-assert-if-true !null $this->teamcityLogfile
     */
    public function hasTeamcityLogfile(): bool
    {}

    /**
     * @throws Exception
     */
    public function teamcityLogfile(): string
    {}

    /**
     * @phpstan-assert-if-true !null $this->compactPrinter
     */
    public function hasCompactPrinter(): bool
    {}

    /**
     * @throws Exception
     */
    public function compactPrinter(): bool
    {}

    /**
     * @phpstan-assert-if-true !null $this->teamCityPrinter
     */
    public function hasTeamCityPrinter(): bool
    {}

    /**
     * @throws Exception
     */
    public function teamCityPrinter(): bool
    {}

    /**
     * @phpstan-assert-if-true !null $this->testdoxHtmlFile
     */
    public function hasTestdoxHtmlFile(): bool
    {}

    /**
     * @throws Exception
     */
    public function testdoxHtmlFile(): string
    {}

    /**
     * @phpstan-assert-if-true !null $this->testdoxTextFile
     */
    public function hasTestdoxTextFile(): bool
    {}

    /**
     * @throws Exception
     */
    public function testdoxTextFile(): string
    {}

    /**
     * @phpstan-assert-if-true !null $this->testdoxPrinter
     */
    public function hasTestDoxPrinter(): bool
    {}

    /**
     * @throws Exception
     */
    public function testdoxPrinter(): bool
    {}

    /**
     * @phpstan-assert-if-true !null $this->testdoxPrinterSummary
     */
    public function hasTestDoxPrinterSummary(): bool
    {}

    /**
     * @throws Exception
     */
    public function testdoxPrinterSummary(): bool
    {}

    /**
     * @phpstan-assert-if-true !null $this->testSuffixes
     */
    public function hasTestSuffixes(): bool
    {}

    /**
     * @throws Exception
     *
     * @return non-empty-list<non-empty-string>
     */
    public function testSuffixes(): array
    {}

    /**
     * @phpstan-assert-if-true !null $this->testSuite
     */
    public function hasTestSuite(): bool
    {}

    /**
     * @throws Exception
     */
    public function testSuite(): string
    {}

    /**
     * @phpstan-assert-if-true !null $this->excludeTestSuite
     */
    public function hasExcludedTestSuite(): bool
    {}

    /**
     * @throws Exception
     */
    public function excludedTestSuite(): string
    {}

    public function useDefaultConfiguration(): bool
    {}

    /**
     * @phpstan-assert-if-true !null $this->displayDetailsOnAllIssues
     */
    public function hasDisplayDetailsOnAllIssues(): bool
    {}

    /**
     * @throws Exception
     */
    public function displayDetailsOnAllIssues(): bool
    {}

    /**
     * @phpstan-assert-if-true !null $this->displayDetailsOnIncompleteTests
     */
    public function hasDisplayDetailsOnIncompleteTests(): bool
    {}

    /**
     * @throws Exception
     */
    public function displayDetailsOnIncompleteTests(): bool
    {}

    /**
     * @phpstan-assert-if-true !null $this->displayDetailsOnSkippedTests
     */
    public function hasDisplayDetailsOnSkippedTests(): bool
    {}

    /**
     * @throws Exception
     */
    public function displayDetailsOnSkippedTests(): bool
    {}

    /**
     * @phpstan-assert-if-true !null $this->displayDetailsOnTestsThatTriggerDeprecations
     */
    public function hasDisplayDetailsOnTestsThatTriggerDeprecations(): bool
    {}

    /**
     * @throws Exception
     */
    public function displayDetailsOnTestsThatTriggerDeprecations(): bool
    {}

    /**
     * @phpstan-assert-if-true !null $this->displayDetailsOnPhpunitDeprecations
     */
    public function hasDisplayDetailsOnPhpunitDeprecations(): bool
    {}

    /**
     * @throws Exception
     */
    public function displayDetailsOnPhpunitDeprecations(): bool
    {}

    /**
     * @phpstan-assert-if-true !null $this->displayDetailsOnPhpunitNotices
     */
    public function hasDisplayDetailsOnPhpunitNotices(): bool
    {}

    /**
     * @throws Exception
     */
    public function displayDetailsOnPhpunitNotices(): bool
    {}

    /**
     * @phpstan-assert-if-true !null $this->displayDetailsOnTestsThatTriggerErrors
     */
    public function hasDisplayDetailsOnTestsThatTriggerErrors(): bool
    {}

    /**
     * @throws Exception
     */
    public function displayDetailsOnTestsThatTriggerErrors(): bool
    {}

    /**
     * @phpstan-assert-if-true !null $this->displayDetailsOnTestsThatTriggerNotices
     */
    public function hasDisplayDetailsOnTestsThatTriggerNotices(): bool
    {}

    /**
     * @throws Exception
     */
    public function displayDetailsOnTestsThatTriggerNotices(): bool
    {}

    /**
     * @phpstan-assert-if-true !null $this->displayDetailsOnTestsThatTriggerWarnings
     */
    public function hasDisplayDetailsOnTestsThatTriggerWarnings(): bool
    {}

    /**
     * @throws Exception
     */
    public function displayDetailsOnTestsThatTriggerWarnings(): bool
    {}

    public function version(): bool
    {}

    /**
     * @phpstan-assert-if-true !null $this->logEventsText
     */
    public function hasLogEventsText(): bool
    {}

    /**
     * @throws Exception
     */
    public function logEventsText(): string
    {}

    /**
     * @phpstan-assert-if-true !null $this->logEventsVerboseText
     */
    public function hasLogEventsVerboseText(): bool
    {}

    /**
     * @throws Exception
     */
    public function logEventsVerboseText(): string
    {}

    public function debug(): bool
    {}

    public function withTelemetry(): bool
    {}

    /**
     * @phpstan-assert-if-true !null $this->extensions
     */
    public function hasExtensions(): bool
    {}

    /**
     * @throws Exception
     *
     * @return non-empty-list<non-empty-string>
     */
    public function extensions(): array
    {}
}
