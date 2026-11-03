<?php declare(strict_types=1);
/*
 * This file is part of phpunit/php-code-coverage.
 *
 * (c) Sebastian Bergmann <sebastian@phpunit.de>
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */
namespace SebastianBergmann\CodeCoverage\Data;

use function array_flip;
use function array_key_exists;
use function array_keys;
use function count;
use function is_array;
use function ksort;
use function max;
use SebastianBergmann\CodeCoverage\Driver\Driver;

/**
 * @internal This class is not covered by the backward compatibility promise for phpunit/php-code-coverage
 *
 * @no-named-arguments Parameter names are not covered by the backward compatibility promise for phpunit/php-code-coverage
 *
 * @phpstan-import-type FunctionCoverageType from RawCodeCoverageData as RawFunctionCoverageType
 *
 * @phpstan-type TestIdType non-empty-string
 * @phpstan-type TestIndexType non-negative-int
 * @phpstan-type FunctionCoverageType array<non-empty-string, array<non-empty-string, ProcessedFunctionCoverageData>>
 * @phpstan-type LineCoverageType array<non-empty-string, array<positive-int, null|array<TestIndexType, positive-int>>>
 */
final class ProcessedCodeCoverageData
{
    /**
     * Line coverage data.
     * An array of filenames, each having an array of linenumbers, each executable line having a map of
     * test index to the number of times the testcase executed the line (1 for drivers that do not
     * collect hit counts).
     *
     * @var LineCoverageType
     */
    private array $lineCoverage = [];

    /**
     * Test case ids are interned: the first time a test case records coverage it is assigned the
     * next free index, and all per-line, per-branch, and per-path hit maps are keyed by that index
     * instead of the id string. This keeps the hit maps small and makes inserting and merging
     * cheap. Use testIds() to resolve an index back to the test case id.
     *
     * @var array<TestIdType, TestIndexType>
     */
    private array $testIdToIndex = [];

    /**
     * The next index that testIndex() will assign. Tracked separately instead of derived
     * from count($this->testIdToIndex) so that a sparse or non-zero-based index table
     * installed through setTestIds() cannot cause two test case ids to share an index.
     *
     * @var TestIndexType
     */
    private int $nextTestIndex = 0;

    /**
     * Function coverage data.
     * Maintains base format of raw data (@see RawCodeCoverageData), but each 'hit' entry is a map
     * of test index to the number of times the testcase traversed the branch or path (1 for drivers that do not
     * collect hit counts).
     *
     * @var FunctionCoverageType
     */
    private array $functionCoverage = [];

    /**
     * Files whose data was not collected by a driver but seeded from static analysis because no
     * test executed them. Which lines are executable for such a file is decided by static
     * analysis alone, whereas for an executed file it also depends on which lines the driver
     * reports. Merging both kinds of data for the same file would therefore count lines as not
     * executed that no driver ever reports, so merge() lets data collected by a driver replace
     * seeded data instead of merging it.
     *
     * @var array<non-empty-string, true>
     */
    private array $filesSeededFromStaticAnalysis = [];

    /**
     * Whether the hit counts in this object are exact execution counts (the driver that collected
     * the data counts how often a line was executed) or only mean "executed at least once".
     */
    private bool $collectsHitCounts;
    private bool $lineCoverageSorted     = true;
    private bool $functionCoverageSorted = true;

    public function __construct(bool $collectsHitCounts = false)
    {}

    public function collectsHitCounts(): bool
    {}

    public function initializeUnseenData(RawCodeCoverageData $rawData): void
    {}

    /**
     * Initializes the data for files that were not executed by any test from data that was seeded
     * from static analysis, see RawCodeCoverageData::fromUncoveredFile().
     */
    public function initializeUncoveredFiles(RawCodeCoverageData $rawData): void
    {}

    /**
     * @param non-empty-string $testCaseId
     */
    public function markCodeAsExecutedByTestCase(string $testCaseId, RawCodeCoverageData $executedCode): void
    {}

    /**
     * @param LineCoverageType $lineCoverage
     */
    public function setLineCoverage(array $lineCoverage): void
    {}

    /**
     * @return LineCoverageType
     */
    public function lineCoverage(): array
    {}

    /**
     * @param FunctionCoverageType $functionCoverage
     */
    public function setFunctionCoverage(array $functionCoverage): void
    {}

    /**
     * @return FunctionCoverageType
     */
    public function functionCoverage(): array
    {}

    /**
     * @return list<non-empty-string>
     */
    public function coveredFiles(): array
    {}

    /**
     * @param array<TestIndexType, TestIdType> $testIds
     */
    public function setTestIds(array $testIds): void
    {}

    /**
     * @return array<TestIndexType, TestIdType>
     */
    public function testIds(): array
    {}

    /**
     * @param non-empty-string $oldFile
     * @param non-empty-string $newFile
     */
    public function renameFile(string $oldFile, string $newFile): void
    {}

    /**
     * Hit counts for a test case that occurs in both operands are combined with max(), not summed:
     * the same test case id on both sides means the same test execution was observed twice (for
     * example when coverage data collected in parallel is merged), not that the test ran twice.
     * This preserves the deduplication semantics of the list-based representation that was used
     * before hit counts were recorded. Accumulation of hit counts within a single test run
     * happens in markCodeAsExecutedByTestCase() and recordHit() instead.
     *
     * The merged data only contains exact hit counts if both operands do.
     *
     * Data for a file that was seeded from static analysis is replaced by data for that file
     * that was collected by a driver, it is not merged with it.
     */
    public function merge(self $newData): void
    {}

    private function sortLineCoverage(): void
    {}

    /**
     * @param TestIdType $testCaseId
     *
     * @return TestIndexType
     */
    private function testIndex(string $testCaseId): int
    {}

    /**
     * The test indexes of another ProcessedCodeCoverageData object are meaningless in the context
     * of this object: the same index may refer to a different test case id. This method interns
     * the other object's test case ids into this object's index table and returns the other
     * object's line and function coverage with all test indexes translated accordingly. When both
     * objects agree on the numbering the data is returned as-is.
     *
     * @return array{0: LineCoverageType, 1: FunctionCoverageType}
     */
    private function withTestIndexesRemappedToThisObject(self $newData): array
    {}

    /**
     * Determine the priority for a line.
     *
     * 1 = the line is not set
     * 2 = the line has not been tested
     * 3 = the line is dead code
     * 4 = the line has been tested
     *
     * During a merge, a higher number is better.
     *
     * @param array<positive-int, null|array<TestIndexType, positive-int>> $data
     * @param positive-int                                                 $line
     *
     * @return 1|2|3|4
     */
    private function priorityForLine(array $data, int $line): int
    {}

    /**
     * @param null|array<TestIndexType, positive-int> $data
     *
     * @return 2|3|4
     */
    private function priorityForValue(null|array $data): int
    {}

    /**
     * For a function we have never seen before, copy all data over and simply init the 'hit' array.
     *
     * @param non-empty-string                                      $file
     * @param non-empty-string                                      $functionName
     * @param ProcessedFunctionCoverageData|RawFunctionCoverageType $functionData
     */
    private function initPreviouslyUnseenFunction(string $file, string $functionName, array|ProcessedFunctionCoverageData $functionData): void
    {}

    /**
     * For a function we have seen before, only copy over and init the 'hit' array for any unseen branches and paths.
     * Techniques such as mocking and where the contents of a file are different vary during tests (e.g. compiling
     * containers) mean that the functions inside a file cannot be relied upon to be static.
     *
     * @param non-empty-string                                      $file
     * @param non-empty-string                                      $functionName
     * @param ProcessedFunctionCoverageData|RawFunctionCoverageType $functionData
     */
    private function initPreviouslySeenFunction(string $file, string $functionName, array|ProcessedFunctionCoverageData $functionData): void
    {}
}
