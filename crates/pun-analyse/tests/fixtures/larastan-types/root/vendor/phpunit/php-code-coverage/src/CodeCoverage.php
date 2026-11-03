<?php declare(strict_types=1);
/*
 * This file is part of phpunit/php-code-coverage.
 *
 * (c) Sebastian Bergmann <sebastian@phpunit.de>
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */
namespace SebastianBergmann\CodeCoverage;

use SebastianBergmann\CodeCoverage\Data\ProcessedCodeCoverageData;
use SebastianBergmann\CodeCoverage\Data\RawCodeCoverageData;
use SebastianBergmann\CodeCoverage\Driver\Driver;
use SebastianBergmann\CodeCoverage\Driver\Granularity;
use SebastianBergmann\CodeCoverage\Node\Builder;
use SebastianBergmann\CodeCoverage\Node\Directory;
use SebastianBergmann\CodeCoverage\StaticAnalysis\FileAnalyser;
use SebastianBergmann\CodeCoverage\StaticAnalysis\Registry;
use SebastianBergmann\CodeCoverage\Test\Target\MapBuilder;
use SebastianBergmann\CodeCoverage\Test\Target\Mapper;
use SebastianBergmann\CodeCoverage\Test\Target\TargetCollection;
use SebastianBergmann\CodeCoverage\Test\Target\TargetCollectionValidator;
use SebastianBergmann\CodeCoverage\Test\Target\ValidationResult;
use SebastianBergmann\CodeCoverage\Test\TestSize;
use SebastianBergmann\CodeCoverage\Test\TestStatus;

/**
 * Provides collection functionality for PHP code coverage information.
 *
 * @phpstan-type TestType array{size: string, status: string, time: float}
 * @phpstan-type TargetedLines array<non-empty-string, list<positive-int>>
 *
 * @no-named-arguments Parameter names are not covered by the backward compatibility promise for phpunit/php-code-coverage
 */
final class CodeCoverage
{
    private const string UNCOVERED_FILES = 'UNCOVERED_FILES';
    private readonly Driver $driver;
    private readonly Filter $filter;
    private ?Mapper $targetMapper = null;

    /**
     * @var ?non-empty-string
     */
    private ?string $cacheDirectory                  = null;
    private bool $checkForUnintentionallyCoveredCode = false;
    private bool $includeUncoveredFiles              = true;
    private bool $ignoreDeprecatedCode               = false;
    private bool $useAnnotationsForIgnoringCode      = true;
    private bool $collectDataNotFilteredUsingTargets = false;

    /**
     * The code coverage data for the test that was processed most recently, as
     * it was before it was filtered using the code coverage targets that test
     * declares.
     */
    private ?RawCodeCoverageData $dataNotFilteredUsingTargets = null;

    /**
     * @var list<class-string>
     */
    private array $parentClassesExcludedFromUnintentionallyCoveredCodeCheck = [];

    /**
     * @var ?non-empty-string
     */
    private ?string $currentId     = null;
    private ?TestSize $currentSize = null;
    private ProcessedCodeCoverageData $data;

    /**
     * @var array<non-empty-string, TestType>
     */
    private array $tests             = [];
    private ?Directory $cachedReport = null;

    public function __construct(Driver $driver, Filter $filter)
    {}

    /**
     * @internal This method is not covered by the backward compatibility promise for phpunit/php-code-coverage
     */
    public function getReport(): Directory
    {}

    /**
     * Clears collected code coverage data.
     */
    public function clear(): void
    {}

    /**
     * @internal
     */
    public function clearCache(): void
    {}

    /**
     * Returns the filter object used.
     */
    public function filter(): Filter
    {}

    /**
     * Returns the collected code coverage data.
     */
    public function getData(bool $raw = false): ProcessedCodeCoverageData
    {}

    /**
     * Sets the coverage data.
     */
    public function setData(ProcessedCodeCoverageData $data): void
    {}

    /**
     * @return array<non-empty-string, TestType>
     */
    public function getTests(): array
    {}

    /**
     * @param array<non-empty-string, TestType> $tests
     */
    public function setTests(array $tests): void
    {}

    /**
     * Returns the files that could not be parsed for static analysis,
     * mapped to the parser's error message.
     *
     * Code coverage for these files is based on the raw data reported by the
     * driver: no refinement of executable lines, no dead code detection, and
     * no information about code units.
     *
     * @return array<non-empty-string, non-empty-string>
     */
    public function parseErrors(): array
    {}

    /**
     * @param non-empty-string $id
     */
    public function start(string $id, ?TestSize $size = null, bool $clear = false): void
    {}

    public function stop(bool $append = true, ?TestStatus $status = null, null|false|TargetCollection $covers = null, ?TargetCollection $uses = null, float $time = 0.0): RawCodeCoverageData
    {}

    /**
     * @param ?non-empty-string $id
     *
     * @throws ReflectionException
     * @throws TestIdMissingException
     * @throws UnintentionallyCoveredCodeException
     */
    public function append(RawCodeCoverageData $rawData, ?string $id = null, bool $append = true, ?TestStatus $status = null, null|false|TargetCollection $covers = null, ?TargetCollection $uses = null, float $time = 0.0, ?TestSize $size = null): void
    {}

    public function merge(self $that): void
    {}

    public function enableCheckForUnintentionallyCoveredCode(): void
    {}

    public function disableCheckForUnintentionallyCoveredCode(): void
    {}

    public function includeUncoveredFiles(): void
    {}

    public function excludeUncoveredFiles(): void
    {}

    public function enableAnnotationsForIgnoringCode(): void
    {}

    public function disableAnnotationsForIgnoringCode(): void
    {}

    public function ignoreDeprecatedCode(): void
    {}

    public function doNotIgnoreDeprecatedCode(): void
    {}

    /**
     * Keeps the code coverage data for each test as it was before it was
     * filtered using the code coverage targets that test declares.
     *
     * The data is kept for one test at a time, and is only meaningful until the
     * next test is processed.
     */
    public function enableCollectionOfDataNotFilteredUsingTargets(): void
    {}

    public function disableCollectionOfDataNotFilteredUsingTargets(): void
    {}

    /**
     * @phpstan-assert-if-true !null $this->dataNotFilteredUsingTargets
     */
    public function hasDataNotFilteredUsingTargets(): bool
    {}

    /**
     * Returns the code coverage data for the test that was processed most
     * recently, as it was before it was filtered using the code coverage
     * targets that test declares.
     *
     * The data is still filtered using the code coverage filter: code that is
     * not part of the code that is subject to code coverage analysis is not
     * reported by the driver in the first place.
     *
     * @throws DataNotFilteredUsingTargetsNotCollectedException
     */
    public function dataNotFilteredUsingTargets(): RawCodeCoverageData
    {}

    /**
     * @phpstan-assert-if-true !null $this->cacheDirectory
     */
    public function cachesStaticAnalysis(): bool
    {}

    /**
     * @param non-empty-string $directory
     */
    public function cacheStaticAnalysis(string $directory): void
    {}

    public function doNotCacheStaticAnalysis(): void
    {}

    /**
     * @throws StaticAnalysisCacheNotConfiguredException
     *
     * @return non-empty-string
     */
    public function cacheDirectory(): string
    {}

    /**
     * @param class-string $className
     */
    public function excludeSubclassesOfThisClassFromUnintentionallyCoveredCodeCheck(string $className): void
    {}

    /**
     * @throws BranchCoverageNotSupportedException
     * @throws PathCoverageNotSupportedException
     *
     * @deprecated
     */
    public function enableBranchAndPathCoverage(): void
    {}

    /**
     * @deprecated
     */
    public function disableBranchAndPathCoverage(): void
    {}

    /**
     * @deprecated
     */
    public function collectsBranchAndPathCoverage(): bool
    {}

    public function validate(TargetCollection $targets): ValidationResult
    {}

    /**
     * @return array{name: non-empty-string, version: non-empty-string}
     *
     * @internal This method is not covered by the backward compatibility promise for phpunit/php-code-coverage
     */
    public function driverInformation(): array
    {}

    /**
     * @throws UnintentionallyCoveredCodeException
     */
    private function addUncoveredFilesFromFilter(): void
    {}

    private function targetMapper(): Mapper
    {}

    private function analyser(): FileAnalyser
    {}
}
