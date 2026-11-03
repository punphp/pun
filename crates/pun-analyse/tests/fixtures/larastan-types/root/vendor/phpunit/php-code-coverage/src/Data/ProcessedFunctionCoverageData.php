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

use function array_map;
use NoDiscard;

/**
 * @internal This class is not covered by the backward compatibility promise for phpunit/php-code-coverage
 *
 * @no-named-arguments Parameter names are not covered by the backward compatibility promise for phpunit/php-code-coverage
 *
 * @phpstan-import-type TestIndexType from ProcessedCodeCoverageData
 * @phpstan-import-type FunctionCoverageType from RawCodeCoverageData
 * @phpstan-import-type BranchCoverageType from RawCodeCoverageData
 * @phpstan-import-type PathCoverageType from RawCodeCoverageData
 */
final readonly class ProcessedFunctionCoverageData
{
    /** @var array<int, ProcessedBranchCoverageData> */
    public array $branches;

    /** @var array<int, ProcessedPathCoverageData> */
    public array $paths;

    /**
     * @param FunctionCoverageType $xdebugCoverageData
     */
    public static function fromXdebugCoverage(array $xdebugCoverageData): self
    {}

    /**
     * @param array<int, ProcessedBranchCoverageData> $branches
     * @param array<int, ProcessedPathCoverageData>   $paths
     */
    public function __construct(
        array $branches,
        array $paths,
    ) {}

    #[NoDiscard]
    public function merge(self $data): self
    {}

    /**
     * @param array<TestIndexType, TestIndexType> $remap
     */
    #[NoDiscard]
    public function withRemappedTestIndexes(array $remap): self
    {}

    /**
     * @param TestIndexType $testIndex
     * @param positive-int  $count
     */
    public function recordBranchHit(int $branchId, int $testIndex, int $count): void
    {}

    /**
     * @param TestIndexType $testIndex
     * @param positive-int  $count
     */
    public function recordPathHit(int $pathId, int $testIndex, int $count): void
    {}
}
