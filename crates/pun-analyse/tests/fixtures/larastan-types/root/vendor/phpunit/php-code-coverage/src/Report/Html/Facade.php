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

use const DIRECTORY_SEPARATOR;
use function array_key_exists;
use function assert;
use function copy;
use function date;
use function dirname;
use function str_ends_with;
use SebastianBergmann\CodeCoverage\FileCouldNotBeWrittenException;
use SebastianBergmann\CodeCoverage\Node\AbstractNode;
use SebastianBergmann\CodeCoverage\Node\Directory as DirectoryNode;
use SebastianBergmann\CodeCoverage\Node\File as FileNode;
use SebastianBergmann\CodeCoverage\Report\Html\ClassView\Builder;
use SebastianBergmann\CodeCoverage\Report\Html\ClassView\Node\ClassNode;
use SebastianBergmann\CodeCoverage\Report\Html\ClassView\Node\NamespaceNode;
use SebastianBergmann\CodeCoverage\Report\Html\ClassView\Renderer\Class_ as ClassRenderer;
use SebastianBergmann\CodeCoverage\Report\Html\ClassView\Renderer\Dashboard as ClassDashboard;
use SebastianBergmann\CodeCoverage\Report\Html\ClassView\Renderer\Namespace_ as NamespaceRenderer;
use SebastianBergmann\CodeCoverage\Report\Thresholds;
use SebastianBergmann\CodeCoverage\Test\TestSizes;
use SebastianBergmann\CodeCoverage\Util\Filesystem;
use SebastianBergmann\Template\Exception;
use SebastianBergmann\Template\Template;

/**
 * @internal This class is not covered by the backward compatibility promise for phpunit/php-code-coverage
 *
 * @no-named-arguments Parameter names are not covered by the backward compatibility promise for phpunit/php-code-coverage
 */
final readonly class Facade
{
    private string $templatePath;
    private string $generator;
    private Colors $colors;
    private Thresholds $thresholds;
    private CustomCssFile $customCssFile;
    private Views $views;

    public function __construct(string $generator = '', ?Colors $colors = null, ?Thresholds $thresholds = null, ?CustomCssFile $customCssFile = null, Views $views = Views::FileViewAndClassView)
    {}

    public function process(DirectoryNode $report, string $target): void
    {}

    /**
     * The test sizes for which the report has coverage data.
     *
     * A test that did not cover any code is not part of the coverage data, so
     * a test size for which no line was executed is a test size that cannot be
     * filtered by in a meaningful way.
     *
     * @return int<0, 7>
     */
    private function testSizes(DirectoryNode $report): int
    {}

    /**
     * @param int<0, 7>             $testSizes
     * @param array<string, string> $fileToClassMap
     */
    private function renderFileView(DirectoryNode $report, string $target, string $date, bool $hasBranchCoverage, bool $hasPathCoverage, int $testSizes, array $fileToClassMap): void
    {}

    /**
     * @param int<0, 7> $testSizes
     */
    private function renderClassView(NamespaceNode $rootNamespace, string $target, string $date, bool $hasBranchCoverage, bool $hasPathCoverage, int $testSizes): void
    {}

    /**
     * @return array<string, string>
     */
    private function buildFileToClassMap(NamespaceNode $rootNamespace): array
    {}

    private function copyFiles(string $target): void
    {}

    private function renderCss(string $target): void
    {}

    private function directory(string $directory): string
    {}
}
