<?php declare(strict_types=1);
/*
 * This file is part of phpunit/php-code-coverage.
 *
 * (c) Sebastian Bergmann <sebastian@phpunit.de>
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */
namespace SebastianBergmann\CodeCoverage\Report\Html\ClassView\Renderer;

use function array_key_exists;
use function array_keys;
use function array_pop;
use function array_sum;
use function count;
use function htmlspecialchars;
use function sprintf;
use function str_repeat;
use function substr_count;
use SebastianBergmann\CodeCoverage\Data\ProcessedMethodType;
use SebastianBergmann\CodeCoverage\FileCouldNotBeWrittenException;
use SebastianBergmann\CodeCoverage\Report\Html\ClassView\Node\ClassNode;
use SebastianBergmann\CodeCoverage\Report\Html\Renderer;
use SebastianBergmann\CodeCoverage\Util\Percentage;
use SebastianBergmann\Template\Exception;
use SebastianBergmann\Template\Template;

/**
 * @internal This class is not covered by the backward compatibility promise for phpunit/php-code-coverage
 *
 * @no-named-arguments Parameter names are not covered by the backward compatibility promise for phpunit/php-code-coverage
 *
 * @phpstan-import-type TestDataType from \SebastianBergmann\CodeCoverage\Node\Builder
 * @phpstan-import-type TestIndexType from \SebastianBergmann\CodeCoverage\Data\ProcessedCodeCoverageData
 * @phpstan-import-type CoverageItemData from \SebastianBergmann\CodeCoverage\Report\Html\Renderer
 */
final class Class_ extends Renderer
{
    public function render(ClassNode $node, string $file): void
    {}

    protected function setCommonTemplateVariablesForClass(Template $template, ClassNode $node): void
    {}

    protected function breadcrumbsForClass(ClassNode $node): string
    {}

    /**
     * @return CoverageItemData
     */
    private function nodeData(ClassNode $node): array
    {}

    private function renderItems(ClassNode $node): string
    {}

    private function renderMethodItem(Template $template, ProcessedMethodType $method, string $indent = '&nbsp;', string $anchorPrefix = ''): string
    {}

    private function renderSourceSections(ClassNode $node): string
    {}

    private function renderSectionHeader(string $title): string
    {}

    /**
     * @param non-empty-string                                $filePath
     * @param array<int, ?array<TestIndexType, positive-int>> $coverageData
     * @param array<TestIndexType, TestDataType>              $testData
     */
    private function renderSourceSection(string $filePath, int $startLine, int $endLine, array $coverageData, array $testData, bool $collectsHitCounts, string $anchorPrefix = ''): string
    {}

    private function pathToRootForClass(ClassNode $node): string
    {}
}
