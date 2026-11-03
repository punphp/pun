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

use function array_diff;
use function array_diff_key;
use function array_flip;
use function array_keys;
use function is_file;
use SebastianBergmann\CodeCoverage\Data\ProcessedCodeCoverageData;
use SebastianBergmann\CodeCoverage\Data\RawCodeCoverageData;
use SebastianBergmann\CodeCoverage\StaticAnalysis\FileAnalyser;
use SebastianBergmann\CodeCoverage\Test\Target\Mapper;
use SebastianBergmann\CodeCoverage\Test\Target\TargetCollection;
use SebastianBergmann\CodeCoverage\Test\TestSize;

/**
 * @internal This class is not covered by the backward compatibility promise for phpunit/php-code-coverage
 *
 * @no-named-arguments Parameter names are not covered by the backward compatibility promise for phpunit/php-code-coverage
 *
 * @phpstan-import-type TargetedLines from CodeCoverage
 */
final readonly class FilterProcessor
{
    /**
     * @param false|TargetedLines $linesToBeCovered
     * @param TargetedLines       $linesToBeUsed
     * @param list<class-string>  $parentClassesExcludedFromUnintentionallyCoveredCodeCheck
     *
     * @throws ReflectionException
     * @throws UnintentionallyCoveredCodeException
     */
    public function applyCoversAndUsesFilter(RawCodeCoverageData $rawData, array|false $linesToBeCovered, array $linesToBeUsed, TestSize $size, bool $checkForUnintentionallyCoveredCode, Mapper $targetMapper, array $parentClassesExcludedFromUnintentionallyCoveredCodeCheck, TargetCollection $covers, TargetCollection $uses): void
    {}

    public function applyFilter(RawCodeCoverageData $data, Filter $filter): void
    {}

    public function applyExecutableLinesFilter(RawCodeCoverageData $data, Filter $filter, FileAnalyser $analyser): void
    {}

    public function applyIgnoredLinesFilter(RawCodeCoverageData $data, Filter $filter, FileAnalyser $analyser): void
    {}

    /**
     * @return list<RawCodeCoverageData>
     */
    public function uncoveredFilesFromFilter(Filter $filter, ProcessedCodeCoverageData $data, FileAnalyser $analyser): array
    {}
}
