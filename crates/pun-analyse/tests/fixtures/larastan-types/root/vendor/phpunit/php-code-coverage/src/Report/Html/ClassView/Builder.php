<?php declare(strict_types=1);
/*
 * This file is part of phpunit/php-code-coverage.
 *
 * (c) Sebastian Bergmann <sebastian@phpunit.de>
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */
namespace SebastianBergmann\CodeCoverage\Report\Html\ClassView;

use function array_key_exists;
use function array_keys;
use function count;
use function explode;
use function in_array;
use SebastianBergmann\CodeCoverage\Data\ProcessedClassType;
use SebastianBergmann\CodeCoverage\Data\ProcessedTraitType;
use SebastianBergmann\CodeCoverage\Node\Directory as DirectoryNode;
use SebastianBergmann\CodeCoverage\Node\File as FileNode;
use SebastianBergmann\CodeCoverage\Report\Html\ClassView\Node\ClassNode;
use SebastianBergmann\CodeCoverage\Report\Html\ClassView\Node\NamespaceNode;
use SebastianBergmann\CodeCoverage\Report\Html\ClassView\Node\ParentSection;
use SebastianBergmann\CodeCoverage\Report\Html\ClassView\Node\TraitSection;
use SebastianBergmann\CodeCoverage\StaticAnalysis\Class_;
use SebastianBergmann\CodeCoverage\StaticAnalysis\Trait_;

/**
 * @internal This class is not covered by the backward compatibility promise for phpunit/php-code-coverage
 *
 * @no-named-arguments Parameter names are not covered by the backward compatibility promise for phpunit/php-code-coverage
 */
final class Builder
{
    /**
     * @var array<non-empty-string, array{file: FileNode, raw: Class_, processed: ProcessedClassType}>
     */
    private array $classRegistry = [];

    /**
     * @var array<non-empty-string, array{file: FileNode, raw: Trait_, processed: ProcessedTraitType}>
     */
    private array $traitRegistry = [];

    public function build(DirectoryNode $root): NamespaceNode
    {}

    private function reduceRoot(NamespaceNode $root): NamespaceNode
    {}

    private function collectRegistries(DirectoryNode $directory): void
    {}

    /**
     * @param array<string, NamespaceNode> $namespaceMap
     */
    private function ensureNamespaceExists(string $namespace, array &$namespaceMap, NamespaceNode $rootNamespace): NamespaceNode
    {}

    /**
     * @return list<TraitSection>
     */
    private function resolveTraits(Class_ $class): array
    {}

    /**
     * @return list<ParentSection>
     */
    private function resolveParents(Class_ $class): array
    {}
}
