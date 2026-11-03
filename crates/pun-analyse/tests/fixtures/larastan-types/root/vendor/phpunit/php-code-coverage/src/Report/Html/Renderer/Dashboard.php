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

use function htmlspecialchars;
use function sprintf;
use function str_replace;
use function usort;
use SebastianBergmann\CodeCoverage\Data\ProcessedClassType;
use SebastianBergmann\CodeCoverage\Data\ProcessedTraitType;
use SebastianBergmann\CodeCoverage\FileCouldNotBeWrittenException;
use SebastianBergmann\CodeCoverage\Node\AbstractNode;
use SebastianBergmann\CodeCoverage\Node\CrapIndex;
use SebastianBergmann\CodeCoverage\Node\Directory as DirectoryNode;
use SebastianBergmann\Template\Exception;
use SebastianBergmann\Template\Template;

/**
 * @internal This class is not covered by the backward compatibility promise for phpunit/php-code-coverage
 *
 * @no-named-arguments Parameter names are not covered by the backward compatibility promise for phpunit/php-code-coverage
 */
final class Dashboard extends Renderer
{
    public function render(DirectoryNode $node, string $file): void
    {}

    protected function classViewTarget(AbstractNode $node): string
    {}

    protected function activeBreadcrumb(AbstractNode $node): string
    {}

    /**
     * @param array<string, ProcessedClassType|ProcessedTraitType> $classes
     *
     * @return list<array{name: string, coverage: float|int, executableLines: int, complexity: int, link: string}>
     */
    private function classItems(array $classes, string $baseLink): array
    {}

    /**
     * @param array<string, ProcessedClassType|ProcessedTraitType> $classes
     *
     * @return list<array{name: string, coverage: float|int, executableLines: int, complexity: int, link: string}>
     */
    private function methodItems(array $classes, string $baseLink): array
    {}

    /**
     * @param array<string, ProcessedClassType|ProcessedTraitType> $classes
     */
    private function classCrapTable(array $classes, string $baseLink): string
    {}

    /**
     * @param array<string, ProcessedClassType|ProcessedTraitType> $classes
     */
    private function methodCrapTable(array $classes, string $baseLink): string
    {}

    /**
     * @param list<array{name: string, coverage: float|int, crap: string, link: string}> $items
     */
    private function crapTable(array $items, string $entityLabel): string
    {}
}
