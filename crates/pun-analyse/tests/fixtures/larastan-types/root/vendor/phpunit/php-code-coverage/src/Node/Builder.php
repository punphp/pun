<?php declare(strict_types=1);
/*
 * This file is part of phpunit/php-code-coverage.
 *
 * (c) Sebastian Bergmann <sebastian@phpunit.de>
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */
namespace SebastianBergmann\CodeCoverage\Node;

use const DIRECTORY_SEPARATOR;
use function array_pop;
use function explode;
use function is_array;
use function is_file;
use function sha1_file;
use function substr;
use SebastianBergmann\CodeCoverage\CodeCoverage;
use SebastianBergmann\CodeCoverage\Data\ProcessedCodeCoverageData;
use SebastianBergmann\CodeCoverage\StaticAnalysis\FileAnalyser;
use SebastianBergmann\CodeCoverage\Test\TestSizes;
use SebastianBergmann\CodeCoverage\Util\PathReducer;

/**
 * @internal This class is not covered by the backward compatibility promise for phpunit/php-code-coverage
 *
 * @no-named-arguments Parameter names are not covered by the backward compatibility promise for phpunit/php-code-coverage
 *
 * @phpstan-import-type TestType from CodeCoverage
 * @phpstan-import-type TestIndexType from ProcessedCodeCoverageData
 * @phpstan-import-type TestSizeSet from TestSizes
 *
 * @phpstan-type TestDataType array{name: non-empty-string, size: string, status: string, time: float}
 */
final readonly class Builder
{
    private FileAnalyser $analyser;

    public function __construct(FileAnalyser $analyser)
    {}

    /**
     * @param array<non-empty-string, TestType> $testResults
     */
    public function build(ProcessedCodeCoverageData $codeCoverage, array $testResults, string $basePath = ''): Directory
    {}

    /**
     * Combines the test index to test id table of the coverage data with the test results into
     * a map of test index to test data, so that consumers of the report tree can resolve the
     * test indexes used in the coverage data without a separate lookup table.
     *
     * @param array<TestIndexType, non-empty-string> $testIds
     * @param array<non-empty-string, TestType>      $testResults
     *
     * @return array<TestIndexType, TestDataType>
     */
    private function testDataByTestIndex(array $testIds, array $testResults): array
    {}

    /**
     * @param array<array-key, mixed>            $items
     * @param array<TestIndexType, TestDataType> $tests
     * @param array<TestIndexType, TestSizeSet>  $sizeBitByTestIndex
     */
    private function addItems(Directory $root, array $items, array $tests, array $sizeBitByTestIndex, bool $collectsHitCounts): void
    {}

    /**
     * Builds an array representation of the directory structure.
     *
     * For instance,
     *
     * <code>
     * Array
     * (
     *     [Money.php] => Array
     *         (
     *             ...
     *         )
     *
     *     [MoneyBag.php] => Array
     *         (
     *             ...
     *         )
     * )
     * </code>
     *
     * is transformed into
     *
     * <code>
     * Array
     * (
     *     [.] => Array
     *         (
     *             [Money.php] => Array
     *                 (
     *                     ...
     *                 )
     *
     *             [MoneyBag.php] => Array
     *                 (
     *                     ...
     *                 )
     *         )
     * )
     * </code>
     *
     * @return array<array-key, mixed>
     */
    private function buildDirectoryStructure(ProcessedCodeCoverageData $codeCoverage): array
    {}
}
