<?php declare(strict_types=1);
/*
 * This file is part of phpunit/php-code-coverage.
 *
 * (c) Sebastian Bergmann <sebastian@phpunit.de>
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */
namespace SebastianBergmann\CodeCoverage\Report;

use function round;
use function sprintf;
use RoundingMode;

/**
 * @no-named-arguments Parameter names are not covered by the backward compatibility promise for phpunit/php-code-coverage
 */
final readonly class Summary
{
    /**
     * @var non-negative-int
     */
    private int $numberOfExecutableLines;

    /**
     * @var non-negative-int
     */
    private int $numberOfExecutedLines;

    /**
     * @var non-negative-int
     */
    private int $numberOfExecutableBranches;

    /**
     * @var non-negative-int
     */
    private int $numberOfExecutedBranches;

    /**
     * @var non-negative-int
     */
    private int $numberOfExecutablePaths;

    /**
     * @var non-negative-int
     */
    private int $numberOfExecutedPaths;

    /**
     * @var non-negative-int
     */
    private int $numberOfFilesWithoutBranchCoverageData;

    /**
     * @param non-negative-int $numberOfExecutableLines
     * @param non-negative-int $numberOfExecutedLines
     * @param non-negative-int $numberOfExecutableBranches
     * @param non-negative-int $numberOfExecutedBranches
     * @param non-negative-int $numberOfExecutablePaths
     * @param non-negative-int $numberOfExecutedPaths
     * @param non-negative-int $numberOfFilesWithoutBranchCoverageData
     */
    public function __construct(int $numberOfExecutableLines, int $numberOfExecutedLines, int $numberOfExecutableBranches, int $numberOfExecutedBranches, int $numberOfExecutablePaths, int $numberOfExecutedPaths, int $numberOfFilesWithoutBranchCoverageData = 0)
    {}

    /**
     * @return non-negative-int
     */
    public function numberOfExecutableLines(): int
    {}

    /**
     * @return non-negative-int
     */
    public function numberOfExecutedLines(): int
    {}

    public function lineCoverageAsPercentage(): float
    {}

    public function hasBranchAndPathCoverage(): bool
    {}

    /**
     * @return non-negative-int
     */
    public function numberOfExecutableBranches(): int
    {}

    /**
     * @return non-negative-int
     */
    public function numberOfExecutedBranches(): int
    {}

    public function branchCoverageAsPercentage(): float
    {}

    /**
     * @return non-negative-int
     */
    public function numberOfExecutablePaths(): int
    {}

    /**
     * @return non-negative-int
     */
    public function numberOfExecutedPaths(): int
    {}

    public function pathCoverageAsPercentage(): float
    {}

    /**
     * @return non-negative-int
     */
    public function numberOfFilesWithoutBranchCoverageData(): int
    {}

    /**
     * @return non-empty-string
     */
    public function asString(): string
    {}

    /**
     * @param non-negative-int $fraction
     * @param non-negative-int $total
     */
    private function percentage(int $fraction, int $total): float
    {}
}
