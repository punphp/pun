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
use function sprintf;
use function str_repeat;
use function substr_count;
use SebastianBergmann\CodeCoverage\FileCouldNotBeWrittenException;
use SebastianBergmann\CodeCoverage\Report\Html\ClassView\Node\ClassNode;
use SebastianBergmann\CodeCoverage\Report\Html\ClassView\Node\NamespaceNode;
use SebastianBergmann\CodeCoverage\Report\Html\Renderer;
use SebastianBergmann\Template\Exception;
use SebastianBergmann\Template\Template;

/**
 * @internal This class is not covered by the backward compatibility promise for phpunit/php-code-coverage
 *
 * @no-named-arguments Parameter names are not covered by the backward compatibility promise for phpunit/php-code-coverage
 *
 * @phpstan-import-type CoverageItemData from \SebastianBergmann\CodeCoverage\Report\Html\Renderer
 */
final class Namespace_ extends Renderer
{
    public function render(NamespaceNode $node, string $file): void
    {}

    protected function setCommonTemplateVariablesForNamespace(Template $template, NamespaceNode $node): void
    {}

    protected function breadcrumbsForNamespace(NamespaceNode $node): string
    {}

    /**
     * @return CoverageItemData
     */
    private function itemData(NamespaceNode $node): array
    {}

    private function renderItem(NamespaceNode $node): string
    {}

    private function renderClassItem(ClassNode $class): string
    {}

    private function coverageDataJsonForNamespace(NamespaceNode $node): string
    {}

    private function pathToRootForNamespace(NamespaceNode $node): string
    {}
}
