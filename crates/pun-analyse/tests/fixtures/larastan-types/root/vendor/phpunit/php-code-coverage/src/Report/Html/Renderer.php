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

use const ENT_COMPAT;
use const ENT_HTML401;
use const ENT_HTML5;
use const ENT_QUOTES;
use const ENT_SUBSTITUTE;
use function assert;
use function count;
use function file_get_contents;
use function htmlspecialchars;
use function round;
use function rtrim;
use function sprintf;
use function str_repeat;
use function str_replace;
use function strtolower;
use function substr_count;
use RoundingMode;
use SebastianBergmann\CodeCoverage\Data\ProcessedFunctionType;
use SebastianBergmann\CodeCoverage\Data\ProcessedMethodType;
use SebastianBergmann\CodeCoverage\Node\AbstractNode;
use SebastianBergmann\CodeCoverage\Node\Directory as DirectoryNode;
use SebastianBergmann\CodeCoverage\Node\File as FileNode;
use SebastianBergmann\CodeCoverage\Report\Html\ClassView\Node\ClassNode;
use SebastianBergmann\CodeCoverage\Report\Thresholds;
use SebastianBergmann\CodeCoverage\Test\TestSizes;
use SebastianBergmann\CodeCoverage\Util\Json;
use SebastianBergmann\CodeCoverage\Version;
use SebastianBergmann\Environment\Runtime;
use SebastianBergmann\Template\Template;

/**
 * @internal This class is not covered by the backward compatibility promise for phpunit/php-code-coverage
 *
 * @no-named-arguments Parameter names are not covered by the backward compatibility promise for phpunit/php-code-coverage
 *
 * @phpstan-type CoverageItemData array{
 *     name: string,
 *     numClasses?: int,
 *     numTestedClasses?: int,
 *     testedClassesPercent?: float,
 *     testedClassesPercentAsString?: string,
 *     numMethods: int,
 *     numTestedMethods: int,
 *     testedMethodsPercent: float,
 *     testedMethodsPercentAsString: string,
 *     numExecutableLines: int,
 *     numExecutedLines: int,
 *     linesExecutedPercent: float,
 *     linesExecutedPercentAsString: string,
 *     numExecutableBranches: int,
 *     numExecutedBranches: int,
 *     branchesExecutedPercent: float,
 *     branchesExecutedPercentAsString: string,
 *     numExecutablePaths: int,
 *     numExecutedPaths: int,
 *     pathsExecutedPercent: float,
 *     pathsExecutedPercentAsString: string,
 *     numFilesWithoutBranchCoverageData?: int,
 *     icon?: string,
 *     crap?: int|string,
 *     coverageDataJson?: string,
 * }
 * @phpstan-type CoverageMetric array{level: string, percent: string, number: string, bar: string}
 *
 * @phpstan-import-type TestDataType from \SebastianBergmann\CodeCoverage\Node\Builder
 * @phpstan-import-type TestIndexType from \SebastianBergmann\CodeCoverage\Data\ProcessedCodeCoverageData
 */
abstract class Renderer
{
    protected const int HTML_SPECIAL_CHARS_FLAGS = ENT_COMPAT | ENT_HTML401 | ENT_SUBSTITUTE;

    /**
     * Maps each combination of test sizes to the key suffix used in the
     * coverage data JSON that drives the test size filter.
     *
     * @var array<int, non-empty-string>
     */
    protected const array TEST_SIZE_JSON_KEY_SUFFIXES = [
        TestSizes::SMALL                                        => 'Small',
        TestSizes::MEDIUM                                       => 'Medium',
        TestSizes::LARGE                                        => 'Large',
        TestSizes::SMALL | TestSizes::MEDIUM                    => 'SM',
        TestSizes::SMALL | TestSizes::LARGE                     => 'SL',
        TestSizes::MEDIUM | TestSizes::LARGE                    => 'ML',
        TestSizes::SMALL | TestSizes::MEDIUM | TestSizes::LARGE => 'SML',
    ];

    /**
     * The individual test sizes the test size filter can offer, in the order
     * in which they are offered.
     *
     * @var array<int, non-empty-string>
     */
    private const array TEST_SIZE_LABELS = [
        TestSizes::SMALL  => 'Small',
        TestSizes::MEDIUM => 'Medium',
        TestSizes::LARGE  => 'Large',
    ];
    protected readonly SyntaxHighlighter $syntaxHighlighter;
    protected string $templatePath;
    protected string $generator;
    protected string $date;
    protected Thresholds $thresholds;
    protected bool $hasBranchCoverage;
    protected bool $hasPathCoverage;

    /**
     * The test sizes for which the report has coverage data.
     *
     * @var int<0, 7>
     */
    protected int $testSizes;
    protected Views $views;
    protected string $version;

    /**
     * @var array<string, string>
     */
    private array $fileToClassMap = [];

    /**
     * @var array<non-empty-string, Template>
     */
    private array $templates = [];

    /**
     * @var array<TestIndexType, string>
     */
    private array $popoverContentForTest = [];
    private ?string $lineFormat          = null;

    /**
     * @param int<0, 7> $testSizes
     */
    public function __construct(string $templatePath, string $generator, string $date, Thresholds $thresholds, bool $hasBranchCoverage, bool $hasPathCoverage, int $testSizes = TestSizes::ALL, Views $views = Views::FileViewAndClassView)
    {}

    /**
     * @param array<string, string> $map
     */
    public function setFileToClassMap(array $map): void
    {}

    /**
     * The text of a template does not change while a report is rendered, so the
     * same instance is used again instead of reading and parsing the same
     * template file for every node, source line, and coverage bar.
     *
     * An instance returned by this method still carries the values of its
     * previous use: only use it for templates that have all of their
     * placeholders set on every use.
     *
     * @param non-empty-string $name
     */
    protected function template(string $name): Template
    {}

    /**
     * @return non-empty-string
     */
    protected function templateNameForTier(string $base): string
    {}

    /**
     * @param CoverageItemData $data
     */
    protected function renderItemTemplate(Template $template, array $data): string
    {}

    /**
     * Renders the headline figures shown above a coverage table.
     *
     * @param CoverageItemData $data
     */
    protected function renderSummary(array $data, string $methodsLabel, string $classesLabel): string
    {}

    protected function setCommonTemplateVariables(Template $template, AbstractNode $node): void
    {}

    /**
     * The toolbar that filters the coverage metrics by test size.
     *
     * A test size for which the report has no coverage data is offered, but
     * disabled: filtering by it could only ever result in 0%, and showing it
     * as unavailable tells the difference between "no test of this size
     * covers any code" and "tests of this size cover nothing", which an
     * enabled checkbox that results in 0% does not. When no test size at all
     * has coverage data, the toolbar is not rendered because there is nothing
     * it could filter by.
     */
    protected function testSizeFilter(): string
    {}

    /**
     * Target of the "Classes" tab, relative to the root of the report.
     */
    protected function classViewTarget(AbstractNode $node): string
    {}

    protected function escapeHtml(string $value): string
    {}

    protected function viewSwitcher(string $pathToRoot, string $activeView, string $filesTarget = 'index.html', string $classesTarget = '_classes/index.html'): string
    {}

    protected function breadcrumbs(AbstractNode $node): string
    {}

    protected function activeBreadcrumb(AbstractNode $node): string
    {}

    protected function inactiveBreadcrumb(AbstractNode $node, string $pathToRoot): string
    {}

    protected function pathToRoot(AbstractNode $node): string
    {}

    protected function coverageBar(float $percent): string
    {}

    protected function thresholdsLegend(): string
    {}

    protected function lineCoverageLegend(): string
    {}

    protected function branchCoverageLegend(): string
    {}

    protected function colorLevel(float $percent): string
    {}

    /**
     * @param array<string, float|int> $data
     */
    protected function buildCoverageDataJson(array $data): string
    {}

    protected function coverageDataJsonFor(AbstractNode $node): string
    {}

    protected function coverageDataJsonForClassNode(ClassNode $node): string
    {}

    protected function coverageDataJsonForFunctionOrMethod(ProcessedFunctionType|ProcessedMethodType $item): string
    {}

    protected function renderLine(Template $template, int $lineNumber, string $lineContent, string $class, string $popover, string $anchorPrefix = '', string $coverageCount = ''): string
    {}

    /**
     * The attributes that turn a source line or a table cell into the trigger
     * of a popover that lists the tests which cover it.
     */
    protected function popoverAttributes(string $title, string $content): string
    {}

    /**
     * The list item for a test is the same everywhere that test covers a line,
     * and a test usually covers many lines, so it is only built once.
     *
     * @param TestIndexType $testIndex
     * @param TestDataType  $testData
     */
    protected function createPopoverContentForTest(int $testIndex, array $testData): string
    {}

    protected function runtimeString(): string
    {}

    /**
     * The line template is rendered once per source line, so it is compiled
     * into a sprintf() format once instead of being searched for its
     * placeholders on every line.
     */
    private function lineFormat(): string
    {}

    /**
     * @param CoverageItemData $data
     *
     * @return array{lines: CoverageMetric, branches: CoverageMetric, paths: CoverageMetric, methods: CoverageMetric, classes: CoverageMetric}
     */
    private function metrics(array $data): array
    {}

    /**
     * @return CoverageMetric
     */
    private function metric(int $tested, int $total, float $percent, string $percentAsString, bool $incomplete = false): array
    {}
}
