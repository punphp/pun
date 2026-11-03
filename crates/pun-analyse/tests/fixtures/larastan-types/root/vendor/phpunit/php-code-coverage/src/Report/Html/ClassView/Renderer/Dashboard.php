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

use function array_pop;
use function count;
use function htmlspecialchars;
use function sprintf;
use function str_repeat;
use function substr_count;
use function usort;
use SebastianBergmann\CodeCoverage\Data\ProcessedClassType;
use SebastianBergmann\CodeCoverage\Data\ProcessedMethodType;
use SebastianBergmann\CodeCoverage\FileCouldNotBeWrittenException;
use SebastianBergmann\CodeCoverage\Node\CrapIndex;
use SebastianBergmann\CodeCoverage\Report\Html\BubbleChart;
use SebastianBergmann\CodeCoverage\Report\Html\ClassView\Node\ClassNode;
use SebastianBergmann\CodeCoverage\Report\Html\ClassView\Node\NamespaceNode;
use SebastianBergmann\CodeCoverage\Report\Html\Renderer;
use SebastianBergmann\Template\Exception;
use SebastianBergmann\Template\Template;

/**
 * @internal This class is not covered by the backward compatibility promise for phpunit/php-code-coverage
 *
 * @no-named-arguments Parameter names are not covered by the backward compatibility promise for phpunit/php-code-coverage
 */
final class Dashboard extends Renderer
{
    public function render(NamespaceNode $node, string $file): void
    {}

    protected function setCommonTemplateVariablesForNamespace(Template $template, NamespaceNode $node): void
    {}

    protected function breadcrumbsForDashboard(NamespaceNode $node): string
    {}

    /**
     * @param array<string, ProcessedClassType> $classes
     * @param array<string, string>             $classPageLinks
     *
     * @return list<array{name: string, coverage: float|int, executableLines: int, complexity: int, link: string}>
     */
    private function classItems(array $classes, string $pathToRoot, array $classPageLinks): array
    {}

    /**
     * @param array<string, ProcessedClassType> $classes
     * @param array<string, string>             $classPageLinks
     *
     * @return list<array{name: string, coverage: float|int, executableLines: int, complexity: int, link: string}>
     */
    private function methodItems(array $classes, string $pathToRoot, array $classPageLinks): array
    {}

    /**
     * @param array<string, ProcessedClassType> $classes
     * @param array<string, string>             $classPageLinks
     */
    private function classCrapTable(array $classes, string $pathToRoot, array $classPageLinks): string
    {}

    /**
     * @param array<string, ProcessedClassType> $classes
     * @param array<string, string>             $classPageLinks
     */
    private function methodCrapTable(array $classes, string $pathToRoot, array $classPageLinks): string
    {}

    /**
     * @param array<string, string> $classPageLinks
     */
    private function methodLink(array $classPageLinks, string $className, ProcessedMethodType $method): string
    {}

    /**
     * Maps class names to class page paths relative to the class view root.
     *
     * The links carried by the processed class and method types point into
     * the file view; when that view is not rendered, the dashboard has to
     * link to the class pages instead.
     *
     * @return array<string, string>
     */
    private function classPageLinks(NamespaceNode $node): array
    {}

    /**
     * @param list<array{name: string, coverage: float|int, crap: string, link: string}> $items
     */
    private function crapTable(array $items, string $entityLabel): string
    {}

    private function pathToRootForNamespace(NamespaceNode $node): string
    {}
}
