<?php declare(strict_types=1);
/*
 * This file is part of phpunit/php-code-coverage.
 *
 * (c) Sebastian Bergmann <sebastian@phpunit.de>
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */
namespace SebastianBergmann\CodeCoverage\Report\Jsonl;

use function count;
use SebastianBergmann\CodeCoverage\Data\ProcessedBranchCoverageData;

/**
 * @internal This class is not covered by the backward compatibility promise for phpunit/php-code-coverage
 *
 * @no-named-arguments Parameter names are not covered by the backward compatibility promise for phpunit/php-code-coverage
 */
final readonly class BranchCoverage
{
    public int $executed;
    public int $total;

    /**
     * @var list<int>
     */
    public array $uncoveredLines;

    /**
     * @param array<int, ProcessedBranchCoverageData> $branches
     */
    public static function fromBranches(array $branches): self
    {}

    /**
     * @param list<int> $uncoveredLines
     */
    private function __construct(int $executed, int $total, array $uncoveredLines)
    {}
}
