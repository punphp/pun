<?php declare(strict_types=1);
/*
 * This file is part of phpunit/php-code-coverage.
 *
 * (c) Sebastian Bergmann <sebastian@phpunit.de>
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */
namespace SebastianBergmann\CodeCoverage\Report\Html;

use const PHP_INT_MAX;
use function array_key_exists;
use function array_keys;
use function array_merge;
use function array_pop;
use function array_slice;
use function array_sum;
use function array_unique;
use function count;
use function explode;
use function htmlspecialchars;
use function implode;
use function min;
use function range;
use function sprintf;
use function uasort;
use SebastianBergmann\CodeCoverage\Data\ProcessedBranchCoverageData;
use SebastianBergmann\CodeCoverage\Data\ProcessedClassType;
use SebastianBergmann\CodeCoverage\Data\ProcessedFunctionCoverageData;
use SebastianBergmann\CodeCoverage\Data\ProcessedFunctionType;
use SebastianBergmann\CodeCoverage\Data\ProcessedMethodType;
use SebastianBergmann\CodeCoverage\Data\ProcessedPathCoverageData;
use SebastianBergmann\CodeCoverage\Data\ProcessedTraitType;
use SebastianBergmann\CodeCoverage\FileCouldNotBeWrittenException;
use SebastianBergmann\CodeCoverage\Node\File as FileNode;
use SebastianBergmann\CodeCoverage\Util\Json;
use SebastianBergmann\CodeCoverage\Util\Percentage;
use SebastianBergmann\Template\Exception;
use SebastianBergmann\Template\Template;

/**
 * @internal This class is not covered by the backward compatibility promise for phpunit/php-code-coverage
 *
 * @no-named-arguments Parameter names are not covered by the backward compatibility promise for phpunit/php-code-coverage
 *
 * @phpstan-import-type TestIndexType from \SebastianBergmann\CodeCoverage\Data\ProcessedCodeCoverageData
 * @phpstan-import-type TestDataType from \SebastianBergmann\CodeCoverage\Node\Builder
 * @phpstan-import-type CoverageItemData from \SebastianBergmann\CodeCoverage\Report\Html\Renderer
 */
final class File extends Renderer
{
    /**
     * Maximum number of paths per method for which table rows, interactive
     * graph highlighting data, and graph edge classes are rendered; path
     * counts grow combinatorially and would bloat the report otherwise.
     */
    private const int MAX_RENDERED_PATHS        = 100;
    private ?ControlFlowGraph $controlFlowGraph = null;

    public function render(FileNode $node, string $file): void
    {}

    /**
     * @return CoverageItemData
     */
    private function nodeData(FileNode $node): array
    {}

    private function renderItems(FileNode $node): string
    {}

    /**
     * @param array<string, ProcessedClassType|ProcessedTraitType> $items
     */
    private function renderTraitOrClassItems(array $items, Template $template, Template $methodItemTemplate): string
    {}

    /**
     * @param array<string, ProcessedFunctionType> $functions
     */
    private function renderFunctionItems(array $functions, Template $template): string
    {}

    private function renderFunctionOrMethodItem(Template $template, ProcessedFunctionType|ProcessedMethodType $item, string $indent = ''): string
    {}

    private function coverageDataJsonForClassOrTrait(ProcessedClassType|ProcessedTraitType $item): string
    {}

    /**
     * @return list<string>
     */
    private function highlightedSourceFor(FileNode $node): array
    {}

    private function renderSourceWithLineCoverage(FileNode $node): string
    {}

    private function renderViewTabs(string $fileName, string $activeView): string
    {}

    private function renderSourceWithBranchCoverage(FileNode $node): string
    {}

    private function renderSourceWithPathCoverage(FileNode $node): string
    {}

    private function renderBranchStructure(FileNode $node): string
    {}

    private function renderPathStructure(FileNode $node): string
    {}

    private function controlFlowGraph(): ControlFlowGraph
    {}

    /**
     * @param array<string, ProcessedFunctionCoverageData> $coverageData
     *
     * @return array<string, ProcessedFunctionCoverageData>
     */
    private function sortedByStartLine(array $coverageData): array
    {}

    private function abbreviateClassName(string $className): string
    {}

    private function abbreviateMethodName(string $methodName): string
    {}

    private static function startLine(ProcessedFunctionCoverageData $methodData): int
    {}
}
